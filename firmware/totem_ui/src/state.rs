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
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The state of the user interface.
#[derive(Debug, Format, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Format, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mode {
    /// The off mode.
    ///
    /// The Totem is off, only checking UI updates.
    Off,

    /// The random unicolor mode.
    ///
    /// All LEDs are following the same random color progression.
    RandomUnicolor,
}

/// The brightness of the LED strip.
#[derive(
    Debug, Format, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Brightness(pub(crate) u8);

/// The speed of transitions.
#[derive(
    Debug, Format, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Speed(pub(crate) u8);

/// The color temperature.
#[derive(
    Debug, Format, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Temperature(pub(crate) u8);

impl Default for Mode {
    fn default() -> Self {
        Self::Off
    }
}

impl Brightness {
    /// Creates a new brightness.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the brightness value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Speed {
    /// Creates a new speed.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the speed value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Temperature {
    /// Creates a new temperature.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the temperature value.
    pub fn value(&self) -> u8 {
        self.0
    }
}
