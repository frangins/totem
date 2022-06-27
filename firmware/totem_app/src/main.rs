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
    use led_effects::{
        chaser::{OneParameterChaser, RainbowChaser},
        sequence::Rainbow,
        time::TimeConfig,
    };
    use smart_leds::{brightness, colors::BLUE, SmartLedsWrite};
    use systick_monotonic::Systick;

    use totem_board::{
        board::Board,
        constants::{LED_BUFFER_SIZE, NUM_LEDS},
        peripheral::{LedStrip, R1, R2, R3, S1},
        prelude::*,
    };
    use totem_ui::PhysicalUI;

    #[monotonic(binds = SysTick, default = true)]
    type Monotonic = Systick<100>;

    #[shared]
    struct SharedResources {}

    #[local]
    struct LocalResources {
        ui: PhysicalUI<R1, R2, R3, S1>,
        led_strip: LedStrip,
        time_config: TimeConfig,
        chaser: RainbowChaser<Rainbow<NUM_LEDS>, NUM_LEDS>,
    }

    const REFRESH_RATE: Hertz = Hertz(50);

    #[init(local = [led_buffer: [u8; LED_BUFFER_SIZE] = [0; LED_BUFFER_SIZE]])]
    fn init(
        cx: init::Context,
    ) -> (SharedResources, LocalResources, init::Monotonics) {
        defmt::info!("Firmware starting...");

        let cp = cx.core;
        let dp = cx.device;

        let monotonic = Systick::new(cp.SYST, 80_000_000);

        let Board {
            r1,
            r2,
            r3,
            r4: _,
            s1,
            s2: _,
            b1: _,
            b2: _,
            microphone: _,
            p_adc,
            led_strip,
        } = Board::init(dp, cx.local.led_buffer);

        let ui = PhysicalUI::new(p_adc, r1, r2, r3, s1);

        let time_config = TimeConfig::new(REFRESH_RATE, Seconds(2));
        let chaser = RainbowChaser::new(BLUE, &time_config);

        defmt::info!("Firmware initialised!");

        update::spawn().unwrap();

        (
            SharedResources {},
            LocalResources {
                ui,
                led_strip,
                time_config,
                chaser,
            },
            init::Monotonics(monotonic),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(local = [ui, led_strip, time_config, chaser])]
    fn update(cx: update::Context) {
        let update::LocalResources {
            ui,
            led_strip,
            time_config,
            chaser,
        } = cx.local;

        let ui_state = ui.read_state();
        defmt::debug!("UI State: {:?}", ui_state);

        let period = (1000 / time_config.refresh_rate.0).millis();
        update::spawn_at(monotonics::now() + period).unwrap();

        if let Some(sequence) = chaser.next() {
            led_strip
                .write(brightness(sequence, ui_state.brightness.value()))
                .unwrap();
        }
    }
}
