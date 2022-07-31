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

use lcd_1602_i2c::Lcd;
use ws2812_spi::prerendered::Ws2812;

use crate::{
    adc::{Channel, ADC},
    gpio::{
        Alternate, Analog, Input, OpenDrain, PullDown, PushPull, PA0, PA1, PA2,
        PA3, PA4, PA5, PA6, PA7, PB0, PB8, PB9, PC0, PC1, PC2, PC3, PC4,
    },
    i2c::I2c,
    serial::Serial,
    spi::Spi,
    I2C1, SPI1, USART2,
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

/// The pin for the LCD screen I²C SCL line.
pub type SCREEN_SCL = PB8<Alternate<OpenDrain, 4>>;

/// The pin for the LCD screen I²C SDA line.
pub type SCREEN_SDA = PB9<Alternate<OpenDrain, 4>>;

/// The ERCP Basic Tx line.
pub type ERCP_TX = PA2<Alternate<PushPull, 7>>;

/// The ERCP Basic Rx line.
pub type ERCP_RX = PA3<Alternate<PushPull, 7>>;

/// The ADC for potentiometers.
pub type P_ADC = ADC;

/// The SPI peripheral for driving LEDs.
pub type LED_SPI = SPI1;

/// The I²C peripheral for driving the LCD screen.
pub type SCREEN_I2C = I2C1;

/// The UART peripheral for ERCP Basic.
pub type ERCP_UART = USART2;

/// The SPI for driving LEDs.
pub type LedSpi = Spi<LED_SPI, (LED_SCK, LED_MISO, LED_MOSI)>;

/// The I²C for driving the LCD screen.
pub type ScreenI2c = I2c<SCREEN_I2C, (SCREEN_SCL, SCREEN_SDA)>;

/// The serial for ERCP Basic.
pub type ErcpSerial = Serial<ERCP_UART, (ERCP_TX, ERCP_RX)>;

/// The LED strip driver.
pub type LedStrip = Ws2812<'static, LedSpi>;

/// The LCD screen driver.
pub type Screen = Lcd<ScreenI2c>;

/// The LCD screen I²C address.
pub const SCREEN_LCD_ADDRESS: u8 = 0x7C >> 1;

/// The LCD screen RGB backlight controller I²C address.
pub const SCREEN_RGB_ADDRESS: u8 = 0xC0 >> 1;

/// A calibrated potentiometer.
pub trait CalibratedPotentiometer: Channel {
    /// The minimum value reported by the potentiometer.
    const MIN: u16;
    /// The maximum value reported by the potentiometer.
    const MAX: u16;
}

impl CalibratedPotentiometer for R1 {
    const MIN: u16 = 53;
    const MAX: u16 = 3832;
}

impl CalibratedPotentiometer for R2 {
    const MIN: u16 = 53;
    const MAX: u16 = 3832;
}

impl CalibratedPotentiometer for R3 {
    const MIN: u16 = 60;
    const MAX: u16 = 3832;
}

impl CalibratedPotentiometer for R4 {
    const MIN: u16 = 53;
    const MAX: u16 = 3832;
}

impl CalibratedPotentiometer for S1 {
    const MIN: u16 = 30;
    const MAX: u16 = 4020;
}

impl CalibratedPotentiometer for S2 {
    const MIN: u16 = 30;
    const MAX: u16 = 4020;
}
