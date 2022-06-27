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

//! Types modeling the state of the Totem UI.

use defmt::Format;

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
pub struct Brightness(pub(crate) u8);

/// The speed of transitions.
#[derive(Format)]
pub struct Speed(pub(crate) u8);

/// The color temperature.
#[derive(Format)]
pub struct Temperature(pub(crate) u8);

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