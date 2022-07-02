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

//! LED strip utilities.

use smart_leds::{colors::BLACK, SmartLedsWrite};

use totem_board::{constants::NUM_LEDS, peripheral::LedStrip};

/// LED strip extension trait.
pub trait LedStripExt {
    /// Switches off the LED strip.
    fn off(&mut self);
}

impl LedStripExt for LedStrip {
    fn off(&mut self) {
        self.write([BLACK; NUM_LEDS].into_iter()).unwrap();
    }
}
