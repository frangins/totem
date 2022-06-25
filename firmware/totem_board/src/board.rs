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
    gpio::{
        Alternate, Analog, Input, PullDown, PushPull, PA0, PA1, PA4, PA5, PA6,
        PA7, PB0, PC0, PC1, PC2, PC3, PC4,
    },
    prelude::*,
    spi::Spi,
    SPI1,
};

/// The Totem board.
pub struct Board {
    /// The first rotation potentiometer.
    pub r1: R1,
    /// The second rotation potentiometer.
    pub r2: R2,
    /// The third rotation potentiometer.
    pub r3: R3,
    /// The fourth rotation potentiometer.
    pub r4: R4,
    /// The first slider.
    pub s1: S1,
    /// The second slider
    pub s2: S2,
    /// The first button.
    pub b1: B1,
    /// The second button.
    pub b2: B2,
    /// The microphone.
    pub microphone: Microphone,
    /// The ADC for potentiometers.
    pub p_adc: P_ADC,
    /// The SPI driver for the LED strip.
    pub led_spi: LedSpi,
}

/// The pin for the first potentiometer.
pub type R1 = PA0<Analog>;

/// The pin for the second potentiometer.
pub type R2 = PA1<Analog>;

/// The pin for the third potentiometer.
pub type R3 = PA4<Analog>;

/// The pin for the fourth potentiometer.
pub type R4 = PB0<Analog>;

/// The pin for the fifth potentiometer.
pub type S1 = PC1<Analog>;

/// The pin for the sixth potentiometer.
pub type S2 = PC0<Analog>;

/// The pin for the first button.
pub type B1 = PC2<Input<PullDown>>;

/// The pin for the second button.
pub type B2 = PC3<Input<PullDown>>;

/// The pin for the sound sensor.
pub type Microphone = PC4<Analog>;

/// The pin for the LED SPI clock line.
pub type LED_SCK = PA5<Alternate<PushPull, 5>>;

/// The pin for the LED SPI MISO line.
pub type LED_MISO = PA6<Alternate<PushPull, 5>>;

/// The pin for the LED SPI MOSI line.
pub type LED_MOSI = PA7<Alternate<PushPull, 5>>;

/// The SPI for driving LEDs.
pub type LED_SPI = SPI1;

/// The ADC for potentiometers.
pub type P_ADC = ADC;

/// The SPI for driving LEDs.
pub type LedSpi = Spi<LED_SPI, (LED_SCK, LED_MISO, LED_MOSI)>;

impl Board {
    /// Initialises the board.
    pub fn init(dp: crate::pac::Peripherals) -> Self {
        // Clock configuration.
        let mut rcc = dp.RCC.constrain();
        let mut flash = dp.FLASH.constrain();
        let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
        let clocks = rcc.cfgr.sysclk(80.MHz()).freeze(&mut flash.acr, &mut pwr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

        let r1 = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
        let r2 = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
        let r3 = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
        let r4 = gpiob.pb0.into_analog(&mut gpiob.moder, &mut gpiob.pupdr);
        let s1 = gpioc.pc1.into_analog(&mut gpioc.moder, &mut gpioc.pupdr);
        let s2 = gpioc.pc0.into_analog(&mut gpioc.moder, &mut gpioc.pupdr);
        let b1 = gpioc
            .pc2
            .into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr);
        let b2 = gpioc
            .pc3
            .into_pull_down_input(&mut gpioc.moder, &mut gpioc.pupdr);

        let microphone =
            gpioc.pc4.into_analog(&mut gpioc.moder, &mut gpioc.pupdr);

        let led_sck = gpioa.pa5.into_alternate(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpioa.afrl,
        );

        let led_miso = gpioa.pa6.into_alternate(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpioa.afrl,
        );

        let led_mosi = gpioa.pa7.into_alternate(
            &mut gpioa.moder,
            &mut gpioa.otyper,
            &mut gpioa.afrl,
        );

        let mut delay = AsmDelay::new(clocks.sysclk().to_Hz());
        let p_adc = ADC::new(
            dp.ADC1,
            dp.ADC_COMMON,
            &mut rcc.ahb2,
            &mut rcc.ccipr,
            &mut delay,
        );

        let led_spi = Spi::spi1(
            dp.SPI1,
            (led_sck, led_miso, led_mosi),
            ws2812_spi::MODE,
            3.MHz(),
            clocks,
            &mut rcc.apb2,
        );

        Self {
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
            led_spi,
        }
    }
}
