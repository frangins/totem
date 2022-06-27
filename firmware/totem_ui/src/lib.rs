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

//! The user interface for Totem.

#![no_std]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use defmt::Format;
use totem_board::{
    adc::{Channel, ADC},
    peripheral::CalibratedPotentiometer,
    prelude::*,
};

/// The user interface for Totem.
pub struct UI<PMode, PBrightness, PSpeed, PTemperature> {
    p_adc: ADC,
    p_mode: PMode,
    p_brightness: PBrightness,
    p_speed: PSpeed,
    p_temperature: PTemperature,
}

/// The state of the user interface.
#[derive(Format)]
pub struct UIState {
    /// The mode.
    pub mode: Mode,
    /// The brightness of the LED strip.
    pub brightness: Brightness,
    /// The speed of transitions.
    pub speed: Speed,
    /// The color temperature.
    pub temperature: Temperature,
}

/// The mode.
#[derive(Format)]
pub enum Mode {
    /// The first mode.
    First,
    /// The second mode.
    Second,
}

/// The brightness of the LED strip.
#[derive(Format)]
pub struct Brightness(u8);

/// The speed of transitions.
#[derive(Format)]
pub struct Speed(u8);

/// The color temperature.
#[derive(Format)]
pub struct Temperature(u8);

const ITERATIONS: u32 = 200;

impl<
        PMode: CalibratedPotentiometer,
        PBrightness: CalibratedPotentiometer,
        PSpeed: CalibratedPotentiometer,
        PTemperature: CalibratedPotentiometer,
    > UI<PMode, PBrightness, PSpeed, PTemperature>
{
    /// Creates a new UI.
    pub fn new(
        p_adc: ADC,
        p_mode: PMode,
        p_brightness: PBrightness,
        p_speed: PSpeed,
        p_temperature: PTemperature,
    ) -> Self {
        Self {
            p_adc,
            p_mode,
            p_brightness,
            p_speed,
            p_temperature,
        }
    }

    /// Reads the current state of the UI.
    pub fn read_state(&mut self) -> UIState {
        UIState {
            mode: self.read_mode(),
            brightness: self.read_brightness(),
            speed: self.read_speed(),
            temperature: self.read_temperature(),
        }
    }

    /// Reads the value of the mode potentiometer.
    pub fn read_mode(&mut self) -> Mode {
        let value = read_mean(&mut self.p_adc, &mut self.p_mode, ITERATIONS);

        if value < (PMode::MAX - PMode::MIN) / 2 {
            Mode::First
        } else {
            Mode::Second
        }
    }

    /// Reads the value of the brightness potentiometer.
    pub fn read_brightness(&mut self) -> Brightness {
        let value =
            read_mean(&mut self.p_adc, &mut self.p_brightness, ITERATIONS);

        Brightness(adc_to_u8_full_scale(
            value,
            PBrightness::MIN,
            PBrightness::MAX,
        ))
    }

    /// Reads the value of the speed potentiometer.
    pub fn read_speed(&mut self) -> Speed {
        let value = read_mean(&mut self.p_adc, &mut self.p_speed, ITERATIONS);
        Speed(adc_to_u8_full_scale(value, PSpeed::MIN, PSpeed::MAX))
    }

    /// Reads the value of the temperature potentiometer.
    pub fn read_temperature(&mut self) -> Temperature {
        let value =
            read_mean(&mut self.p_adc, &mut self.p_temperature, ITERATIONS);

        Temperature(adc_to_u8_full_scale(
            value,
            PTemperature::MIN,
            PTemperature::MAX,
        ))
    }
}

impl Brightness {
    /// Returns the brightness value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Speed {
    /// Returns the speed value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Temperature {
    /// Returns the temperature value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

fn read_mean(
    adc: &mut ADC,
    channel: &mut impl Channel,
    iterations: u32,
) -> u16 {
    ((0..iterations)
        .fold(0, |sum: u32, _| sum + adc.read(channel).unwrap() as u32)
        / iterations as u32) as u16
}

fn adc_to_u8_full_scale(value: u16, min_value: u16, max_value: u16) -> u8 {
    let scale = (max_value - min_value) / u8::MAX as u16;
    (value.saturating_sub(min_value) / scale)
        .try_into()
        .unwrap_or(u8::MAX)
}
