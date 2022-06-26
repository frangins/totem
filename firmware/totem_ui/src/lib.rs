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

    /// Reads the value of the mode potentiometer.
    pub fn read_mode(&mut self) -> u16 {
        read_mean(&mut self.p_adc, &mut self.p_mode, ITERATIONS)
    }

    /// Reads the value of the brightness potentiometer.
    pub fn read_brightness(&mut self) -> u16 {
        read_mean(&mut self.p_adc, &mut self.p_brightness, ITERATIONS)
    }

    /// Reads the value of the speed potentiometer.
    pub fn read_speed(&mut self) -> u16 {
        read_mean(&mut self.p_adc, &mut self.p_speed, ITERATIONS)
    }

    /// Reads the value of the temperature potentiometer.
    pub fn read_temperature(&mut self) -> u16 {
        read_mean(&mut self.p_adc, &mut self.p_temperature, ITERATIONS)
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
