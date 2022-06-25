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
    use systick_monotonic::Systick;
    use totem_board::{
        board::{Board, P1, P2, P3, P5},
        prelude::*,
    };
    use totem_ui::UI;

    #[monotonic(binds = SysTick, default = true)]
    type Monotonic = Systick<100>;

    #[shared]
    struct SharedResources {}

    #[local]
    struct LocalResources {
        ui: UI<P1, P2, P3, P5>,
    }

    #[init]
    fn init(
        cx: init::Context,
    ) -> (SharedResources, LocalResources, init::Monotonics) {
        defmt::info!("Firmware starting...");

        let cp = cx.core;
        let dp = cx.device;

        let monotonic = Systick::new(cp.SYST, 80_000_000);

        let Board {
            p1,
            p2,
            p3,
            p4: _,
            p5,
            p6: _,
            b1: _,
            b2: _,
            microphone: _,
            p_adc,
            led_spi: _,
        } = Board::init(dp);

        let ui = UI::new(p_adc, p1, p2, p3, p5);

        defmt::info!("Firmware initialised!");

        print_value::spawn().unwrap();

        (
            SharedResources {},
            LocalResources { ui },
            init::Monotonics(monotonic),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(local = [ui])]
    fn print_value(cx: print_value::Context) {
        print_value::spawn_at(monotonics::now() + 10.millis()).unwrap();

        let ui = cx.local.ui;
        let value = ui.read_mode();
        defmt::info!("Value: {}", value);
    }
}
