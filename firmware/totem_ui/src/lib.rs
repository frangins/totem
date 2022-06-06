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
    adc::Channel,
    board::{P1, P_ADC},
    prelude::*,
};

/// The user interface for Totem.
pub struct UI {
    p_adc: P_ADC,
    p1: P1,
}

const ITERATIONS: u32 = 200;

impl UI {
    /// Creates a new UI.
    pub fn new(p_adc: P_ADC, p1: P1) -> Self {
        Self { p_adc, p1 }
    }

    /// Reads the value of the first potentiometer.
    pub fn read_p1(&mut self) -> u16 {
        read_mean(&mut self.p_adc, &mut self.p1, ITERATIONS)
    }
}

fn read_mean(
    adc: &mut P_ADC,
    channel: &mut impl Channel,
    iterations: u32,
) -> u16 {
    ((0..iterations)
        .fold(0, |sum: u32, _| sum + adc.read(channel).unwrap() as u32)
        / iterations as u32) as u16
}
