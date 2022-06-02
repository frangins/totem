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

#[rtic::app(device = stm32l4xx_hal::pac, dispatchers = [TIM2])]
mod app {
    use embedded_hal::blocking::delay::DelayUs;
    use stm32l4xx_hal::{
        adc::ADC,
        gpio::{Analog, PA0},
        prelude::*,
    };
    use systick_monotonic::Systick;

    /// A basic Delay using `cortex_m::asm::delay`.
    pub struct AsmDelay {
        /// The AHB Frequency in Hz.
        ahb_frequency: u32,
    }

    impl AsmDelay {
        /// Initialises an AsmDelay.
        pub fn new(ahb_frequency: u32) -> Self {
            Self { ahb_frequency }
        }
    }

    impl DelayUs<u32> for AsmDelay {
        fn delay_us(&mut self, us: u32) {
            let tick = (us as u64) * (self.ahb_frequency as u64) / 1_000_000;
            cortex_m::asm::delay(tick as u32);
        }
    }

    #[monotonic(binds = SysTick, default = true)]
    type Monotonic = Systick<100>;

    #[shared]
    struct SharedResources {}

    #[local]
    struct LocalResources {
        adc: ADC,
        p1: PA0<Analog>,
    }

    #[init]
    fn init(
        cx: init::Context,
    ) -> (SharedResources, LocalResources, init::Monotonics) {
        defmt::info!("Firmware starting...");

        let cp = cx.core;
        let dp = cx.device;

        // Clock configuration.
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        let monotonic = Systick::new(cp.SYST, 80_000_000);

        let mut delay = AsmDelay::new(clocks.sysclk().to_Hz());
        let adc = ADC::new(
            dp.ADC1,
            dp.ADC_COMMON,
            &mut rcc.ahb2,
            &mut rcc.ccipr,
            &mut delay,
        );

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let p1 = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

        defmt::info!("Firmware initialised!");

        print_value::spawn().unwrap();

        (
            SharedResources {},
            LocalResources { adc, p1 },
            init::Monotonics(monotonic),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }

    #[task(local = [adc, p1])]
    fn print_value(cx: print_value::Context) {
        print_value::spawn_at(monotonics::now() + 10.millis()).unwrap();

        let value = cx.local.adc.read(cx.local.p1).unwrap();
        defmt::info!("Value: {}", value);
    }
}
