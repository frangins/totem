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

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = totem_board::pac, dispatchers = [TIM2])]
mod app {
    use embedded_time::{duration::Seconds, rate::Hertz};
    use ercp_basic::{adapter::SerialAdapter, ErcpBasic};
    use led_effects::{
        chaser::{OneParameterChaser, RainbowChaser},
        sequence::Rainbow,
        time::TimeConfig,
    };
    use smart_leds::{brightness, colors::BLUE, SmartLedsWrite};
    use systick_monotonic::Systick;

    use totem_app::ercp::TotemRouter;
    use totem_board::{
        board::Board,
        constants::{LED_BUFFER_SIZE, NUM_LEDS},
        peripheral::{ErcpSerial, LedStrip},
        prelude::*,
    };
    use totem_ui::UI as _;
    use totem_utils::fake_timer::FakeTimer;

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
        chaser: RainbowChaser<Rainbow<NUM_LEDS>, NUM_LEDS>,
    }

    #[cfg(feature = "ui_physical")]
    type UI = PhysicalUI<R1, R2, R3, S1>;

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
            led_strip,
            ercp_serial,
        } = Board::init(dp, cx.local.led_buffer);

        ////////////////////////////////////////////////////////////////////////
        //                          Resources init                            //
        ////////////////////////////////////////////////////////////////////////

        #[cfg(feature = "ui_physical")]
        let ui = PhysicalUI::new(p_adc, r1, r2, r3, s1);

        let time_config = TimeConfig::new(REFRESH_RATE, Seconds(2));
        let chaser = RainbowChaser::new(BLUE, &time_config);

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

    #[task(local = [led_strip, time_config, chaser], shared = [ui])]
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

        if let Some(sequence) = chaser.next() {
            led_strip
                .write(brightness(sequence, ui_state.brightness.value()))
                .unwrap();
        }
    }

    #[task(binds = USART2, shared = [ercp])]
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

    #[task(shared = [ercp])]
    fn ercp_process(mut cx: ercp_process::Context) {
        defmt::debug!("ERCP frame received. Processing it…");
        cx.shared.ercp.lock(|ercp| ercp.process(&mut ()).ok());
    }
}
