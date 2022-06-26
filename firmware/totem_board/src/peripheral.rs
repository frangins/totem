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

//! The peripherals of the Totem board.

use ws2812_spi::prerendered::Ws2812;

use crate::{
    adc::ADC,
    gpio::{
        Alternate, Analog, Input, PullDown, PushPull, PA0, PA1, PA4, PA5, PA6,
        PA7, PB0, PC0, PC1, PC2, PC3, PC4,
    },
    spi::Spi,
    SPI1,
};

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

/// The LED strip driver.
pub type LedStrip = Ws2812<'static, LedSpi>;
