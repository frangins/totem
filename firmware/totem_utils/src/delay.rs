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

//! Delay implementation.

use embedded_hal::blocking::delay::{DelayMs, DelayUs};

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

impl DelayMs<u16> for AsmDelay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_us(ms as u32 * 1000);
    }
}
