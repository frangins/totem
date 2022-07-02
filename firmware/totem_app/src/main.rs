// Totem - A totem for music festivals, built with love to spread love.
// Copyright (C) 2022 Jean-Philippe Cugnet <jean-philippe@cugnet.eu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![no_main]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

#[cfg(not(any(feature = "ui_physical", feature = "ui_graphical")))]
compile_error!("You must select a UI.");

#[cfg(all(feature = "ui_physical", feature = "ui_graphical"))]
compile_error!("You must select only one UI.");

#[cfg(feature = "panic-probe")]
use panic_probe as _;
#[cfg(not(feature = "panic-probe"))]
use panic_reset as _;

use defmt_rtt as _;

#[rtic::app(device = totem_board::pac, dispatchers = [TIM2, TIM3])]
mod app {
    use systick_monotonic::Systick;

    use embedded_time::{duration::Seconds, rate::Hertz};
    use ercp_basic::{adapter::SerialAdapter, ErcpBasic};
    use led_effects::{
        chaser::{RandomUnicolor, SimpleRandomChaser},
        time::TimeConfig,
    };
    use rand::distributions::Uniform;
    use smart_leds::{brightness, SmartLedsWrite};

    use totem_app::{
        chaser::Chaser,
        ercp::{ErcpContext, TotemRouter},
        led_strip::LedStripExt,
    };
    use totem_board::{
        board::Board,
        constants::{LED_BUFFER_SIZE, NUM_LEDS},
        peripheral::{ErcpSerial, LedStrip},
        prelude::*,
    };
    use totem_ui::{state::Mode, UI as _};
    use totem_utils::fake_timer::FakeTimer;

    #[cfg(feature = "ui_graphical")]
    use totem_ui::GraphicalUI;

    #[cfg(feature = "ui_physical")]
    use totem_board::peripheral::{R1, R2, R3, S1};
    #[cfg(feature = "ui_physical")]
    use totem_ui::PhysicalUI;

    ////////////////////////////////////////////////////////////////////////////
    //                             Resource types                             //
    ////////////////////////////////////////////////////////////////////////////

    #[monotonic(binds = SysTick, default = true)]
    type Monotonic = Systick<100>;

    #[shared]
    struct SharedResources {
        ui: UI,
        ercp: ErcpBasic<SerialAdapter<ErcpSerial>, FakeTimer, TotemRouter>,
    }

    #[local]
    struct LocalResources {
        led_strip: LedStrip,
        time_config: TimeConfig,
        chaser: Chaser<NUM_LEDS>,
    }

    #[cfg(feature = "ui_physical")]
    type UI = PhysicalUI<R1, R2, R3, S1>;
    #[cfg(feature = "ui_graphical")]
    type UI = GraphicalUI;

    ////////////////////////////////////////////////////////////////////////////
    //                             Configuration                              //
    ////////////////////////////////////////////////////////////////////////////

    /// The refresh rate for the update task.
    const REFRESH_RATE: Hertz = Hertz(50);

    ////////////////////////////////////////////////////////////////////////////
    //                                  Init                                  //
    ////////////////////////////////////////////////////////////////////////////

    #[init(local = [led_buffer: [u8; LED_BUFFER_SIZE] = [0; LED_BUFFER_SIZE]])]
    fn init(
        cx: init::Context,
    ) -> (SharedResources, LocalResources, init::Monotonics) {
        defmt::info!("Firmware starting...");

        let cp = cx.core;
        let dp = cx.device;

        ////////////////////////////////////////////////////////////////////////
        //                            System init                             //
        ////////////////////////////////////////////////////////////////////////

        let monotonic = Systick::new(cp.SYST, 80_000_000);

        ////////////////////////////////////////////////////////////////////////
        //                            Board init                              //
        ////////////////////////////////////////////////////////////////////////

        #[allow(unused)]
        let Board {
            r1,
            r2,
            r3,
            r4,
            s1,
            s2,
            b1,
            b2,
            microphone,
            p_adc,
            mut led_strip,
            ercp_serial,
        } = Board::init(dp, cx.local.led_buffer);

        led_strip.off();

        ////////////////////////////////////////////////////////////////////////
        //                          Resources init                            //
        ////////////////////////////////////////////////////////////////////////

        #[cfg(feature = "ui_physical")]
        let ui = PhysicalUI::new(p_adc, r1, r2, r3, s1);
        #[cfg(feature = "ui_graphical")]
        let ui = GraphicalUI::new();

        let time_config = TimeConfig::new(REFRESH_RATE, Seconds(2));
        let chaser = Chaser::None;

        let adapter = SerialAdapter::new(ercp_serial);
        let ercp = ErcpBasic::new(adapter, FakeTimer, TotemRouter);

        defmt::info!("Firmware initialised!");

        ////////////////////////////////////////////////////////////////////////
        //                           Task startup                             //
        ////////////////////////////////////////////////////////////////////////

        update::spawn().unwrap();

        (
            SharedResources { ui, ercp },
            LocalResources {
                led_strip,
                time_config,
                chaser,
            },
            init::Monotonics(monotonic),
        )
    }

    ////////////////////////////////////////////////////////////////////////////
    //                                  Idle                                  //
    ////////////////////////////////////////////////////////////////////////////

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    //                                 Tasks                                  //
    ////////////////////////////////////////////////////////////////////////////

    #[task(priority = 2, local = [led_strip, time_config, chaser], shared = [ui])]
    fn update(mut cx: update::Context) {
        let update::LocalResources {
            led_strip,
            time_config,
            chaser,
        } = cx.local;

        let ui_state = cx.shared.ui.lock(|ui| ui.read_state());
        defmt::debug!("UI State: {:?}", ui_state);

        let period = (1000 / time_config.refresh_rate.0).millis();
        update::spawn_at(monotonics::now() + period).unwrap();

        match ui_state.mode {
            Mode::Off => {
                if !matches!(chaser, Chaser::None) {
                    defmt::info!("Switching to Off mode.");
                    led_strip.off();
                    *chaser = Chaser::None;
                }
            }

            Mode::RandomUnicolor => {
                if !matches!(chaser, Chaser::RandomUnicolor(_)) {
                    defmt::info!("Switching to RandomUnicolor mode.");
                    *chaser = Chaser::RandomUnicolor(RandomUnicolor::new(
                        REFRESH_RATE,
                        Uniform::new(300, 5_000),
                    ));
                }
            }
        }

        if let Some(sequence) = chaser.next() {
            led_strip
                .write(brightness(sequence, ui_state.brightness.value()))
                .unwrap();
        }
    }

    #[task(priority = 3, binds = USART2, shared = [ercp])]
    fn usart2(mut cx: usart2::Context) {
        defmt::trace!("Receiving data on UART");

        cx.shared.ercp.lock(|ercp| {
            ercp.handle_data().ok();

            if ercp.complete_frame_received() {
                defmt::trace!("Complete frame received!");
                ercp_process::spawn().ok();
            }
        });
    }

    #[task(priority = 1, shared = [ui, ercp])]
    fn ercp_process(cx: ercp_process::Context) {
        defmt::debug!("ERCP frame received. Processing it…");

        #[allow(unused)]
        let ercp_process::SharedResources { mut ui, mut ercp } = cx.shared;

        let mut context = ErcpContext::default();
        ercp.lock(|ercp| ercp.process(&mut context).ok());

        #[cfg(feature = "ui_graphical")]
        if let Some(state) = context.ui_state_update {
            ui.lock(|ui| ui.set_state(state));
        }
    }
}
