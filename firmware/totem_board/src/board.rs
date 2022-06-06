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

//! The Totem board.

use totem_utils::delay::AsmDelay;

use crate::{
    adc::ADC,
    gpio::{Analog, PA0},
    prelude::*,
};

/// The Totem board.
pub struct Board {
    /// The ADC for potentiometers.
    pub p_adc: P_ADC,
    /// The first potentiometer.
    pub p1: P1,
}

/// The ADC for potentiometers.
pub type P_ADC = ADC;

/// The pin for the first potentiometer.
pub type P1 = PA0<Analog>;

impl Board {
    /// Initialises the board.
    pub fn init(dp: crate::pac::Peripherals) -> Self {
        // Clock configuration.
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let p1 = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

        let mut delay = AsmDelay::new(clocks.sysclk().to_Hz());
        let p_adc = ADC::new(
            dp.ADC1,
            dp.ADC_COMMON,
            &mut rcc.ahb2,
            &mut rcc.ccipr,
            &mut delay,
        );

        Self { p_adc, p1 }
    }
}
