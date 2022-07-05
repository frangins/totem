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

    use defmt::Format;
    use embedded_time::{duration::Seconds, rate::Hertz};
    use ercp_basic::{adapter::SerialAdapter, ErcpBasic};
    use led_effects::{
        chaser::{Chaser as _, RandomUnicolor},
        sequence::Sequence as _,
        time::TimeConfig,
    };
    use rand::distributions::Uniform;
    use smart_leds::{brightness as set_brightness, SmartLedsWrite};

    use totem_app::{
        chaser::Chaser,
        ercp::{ErcpContext, TotemRouter},
        led_strip::LedStripExt,
    };
    use totem_board::{
        board::Board,
        constants::{LED_BUFFER_SIZE, NUM_LEDS},
        peripheral::{ErcpSerial, LedStrip, Screen},
        prelude::*,
    };
    use totem_ui::{
        state::{Brightness, Mode, ScreenState, UIState},
        UI as _,
    };
    use totem_utils::{delay::AsmDelay, fake_timer::FakeTimer};

    #[cfg(feature = "ui_graphical")]
    use totem_ui::GraphicalUI;

    #[cfg(feature = "ui_physical")]
    use totem_board::peripheral::{B1, R1, R2, R3, S1};
    #[cfg(feature = "ui_physical")]
    use totem_ui::PhysicalUI;

    const MESSAGES: [(&str, &str); 2] =
        [(" Chateau Perche", "  Avrilly 2022"), ("Totem <3<3", "")];

    ////////////////////////////////////////////////////////////////////////////
    //                             Resource types                             //
    ////////////////////////////////////////////////////////////////////////////

    #[monotonic(binds = SysTick, default = true)]
    type Monotonic = Systick<100>;

    #[shared]
    struct SharedResources {
        ui: UI,
        screen: Option<Screen>,
        ercp: ErcpBasic<SerialAdapter<ErcpSerial>, FakeTimer, TotemRouter>,
    }

    #[local]
    struct LocalResources {
        // UI task
        ui_state: UIState,

        // LED task
        led_strip: LedStrip,
        brightness: Brightness,
        time_config: TimeConfig,
        chaser: Chaser<NUM_LEDS>,
    }

    #[cfg(feature = "ui_physical")]
    type UI = PhysicalUI<R1, R2, R3, S1, B1>;
    #[cfg(feature = "ui_graphical")]
    type UI = GraphicalUI;

    ////////////////////////////////////////////////////////////////////////////
    //                             Message types                              //
    ////////////////////////////////////////////////////////////////////////////

    #[derive(Debug, Format)]
    pub enum LedTaskMessage {
        UpdateMode(UIState),
        Next,
    }

    #[derive(Debug, Format)]
    pub enum ScreenTaskMessage {
        Start,
        Stop,
        Next,
    }

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
            mut screen,
            ercp_serial,
        } = Board::init(dp, cx.local.led_buffer);

        // Ensure both the LED strip and screen start off.
        led_strip.off();
        if let Some(ref mut screen) = screen {
            screen.set_rgb(0, 0, 0).unwrap();
        }

        ////////////////////////////////////////////////////////////////////////
        //                          Resources init                            //
        ////////////////////////////////////////////////////////////////////////

        // Shared

        #[cfg(feature = "ui_physical")]
        let ui = PhysicalUI::new(p_adc, r1, r2, r3, s1, b1);
        #[cfg(feature = "ui_graphical")]
        let ui = GraphicalUI::new();

        let adapter = SerialAdapter::new(ercp_serial);
        let ercp = ErcpBasic::new(adapter, FakeTimer, TotemRouter);

        // UI task

        let ui_state = UIState::default();

        // LED task

        let brightness = Brightness::default();
        let time_config = TimeConfig::new(REFRESH_RATE, Seconds(1));
        let chaser = Chaser::None;

        defmt::info!("Firmware initialised!");

        ////////////////////////////////////////////////////////////////////////
        //                           Task startup                             //
        ////////////////////////////////////////////////////////////////////////

        ui_task::spawn().unwrap();

        (
            SharedResources { ui, screen, ercp },
            LocalResources {
                ui_state,
                led_strip,
                brightness,
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

    #[task(priority = 1, local = [ui_state], shared = [ui])]
    fn ui_task(mut cx: ui_task::Context) {
        let ui_task::LocalResources { ui_state } = cx.local;

        ui_task::spawn_at(monotonics::now() + 10.millis()).unwrap();

        let state = cx.shared.ui.lock(|ui| ui.read_state());

        if state != *ui_state {
            defmt::debug!("UI State: {:?}", state);

            led_task::spawn(LedTaskMessage::UpdateMode(state)).unwrap();

            if state.mode != ui_state.mode
                || state.screen_state != ui_state.screen_state
            {
                let screen_message = match (state.mode, state.screen_state) {
                    (Mode::Off, _) => ScreenTaskMessage::Stop,
                    (_, ScreenState::Off) => ScreenTaskMessage::Stop,
                    (_, ScreenState::On) => ScreenTaskMessage::Start,
                };

                screen_task::spawn(screen_message).unwrap();
            }

            *ui_state = state;
        }
    }

    #[task(
        priority = 2,
        capacity = 2,
        local = [
            led_strip,
            time_config,
            brightness,
            chaser,
            drive_screen: bool = false,
        ],
        shared = [screen],
    )]
    fn led_task(mut cx: led_task::Context, message: LedTaskMessage) {
        let led_task::LocalResources {
            led_strip,
            time_config,
            brightness,
            chaser,
            drive_screen,
        } = cx.local;

        match message {
            LedTaskMessage::UpdateMode(ui_state) => {
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
                            *chaser =
                                Chaser::RandomUnicolor(RandomUnicolor::new(
                                    REFRESH_RATE,
                                    Uniform::new(0, 255),
                                    Uniform::new(300, 5_000),
                                ));

                            led_task::spawn(LedTaskMessage::Next).unwrap();
                        }
                    }
                }

                *brightness = ui_state.brightness;
                time_config.transition_time = ui_state.speed.transition_time();
                chaser.set_time_config(time_config);
                chaser.set_temperature(ui_state.temperature);
                *drive_screen = ui_state.screen_state == ScreenState::On;
            }

            LedTaskMessage::Next => {
                if let Some(sequence) = chaser.next() {
                    let period = (1000 / time_config.refresh_rate.0).millis();
                    led_task::spawn_at(
                        monotonics::now() + period,
                        LedTaskMessage::Next,
                    )
                    .unwrap();

                    if *drive_screen {
                        let color = sequence.get_main_color();
                        cx.shared.screen.lock(|screen| {
                            if let Some(screen) = screen {
                                screen
                                    .set_rgb(color.r, color.g, color.b)
                                    .unwrap();
                            }
                        });
                    }

                    led_strip
                        .write(set_brightness(sequence, brightness.value()))
                        .unwrap();
                }
            }
        }
    }

    #[task(
        priority = 1,
        capacity = 2,
        local = [
            next_handle: Option<screen_task::SpawnHandle> = None,
            messages: [(&'static str, &'static str); MESSAGES.len()] = MESSAGES,
            index: usize = 0,
        ],
        shared = [screen],
    )]
    fn screen_task(mut cx: screen_task::Context, message: ScreenTaskMessage) {
        let screen_task::LocalResources {
            next_handle,
            messages,
            index,
        } = cx.local;

        cx.shared.screen.lock(|screen| {
            if let Some(screen) = screen {
                match message {
                    ScreenTaskMessage::Start => {
                        if next_handle.is_none() {
                            screen.set_rgb(255, 255, 255).unwrap();
                            screen_task::spawn(ScreenTaskMessage::Next)
                                .unwrap();
                        }
                    }

                    ScreenTaskMessage::Stop => {
                        if let Some(handle) = next_handle.take() {
                            handle.cancel().unwrap();
                        }

                        let mut delay = AsmDelay::new(80_000_000);
                        screen.clear(&mut delay).unwrap();
                        screen.set_rgb(0, 0, 0).unwrap();
                        *index = 0;
                    }

                    ScreenTaskMessage::Next => {
                        let handle = screen_task::spawn_at(
                            monotonics::now() + 4.secs(),
                            ScreenTaskMessage::Next,
                        )
                        .unwrap();

                        *next_handle = Some(handle);

                        let mut delay = AsmDelay::new(80_000_000);
                        screen.clear(&mut delay).unwrap();
                        screen.set_cursor_position(0, 0).unwrap();
                        screen.write_str(messages[*index].0).unwrap();
                        screen.set_cursor_position(0, 1).unwrap();
                        screen.write_str(messages[*index].1).unwrap();

                        *index += 1;
                        if *index == messages.len() {
                            *index = 0;
                        }
                    }
                }
            }
        })
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
