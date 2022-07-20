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

use defmt::{write, Format};
use embedded_time::duration::{Generic, Milliseconds};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The state of the user interface.
#[derive(Debug, Format, Default, Clone, Copy, PartialEq)]
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
    /// The state of the LCD screen.
    pub screen_state: ScreenState,
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

    /// The rainbow fontain mode.
    ///
    /// LED strips show a symmetrical rainbow divergence.
    RainbowFontain,
}

/// The brightness of the LED strip.
#[derive(
    Debug, Format, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Brightness(pub(crate) u8);

/// The speed of transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Speed(pub(crate) Milliseconds);

/// The color temperature.
#[derive(
    Debug, Format, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Temperature(pub(crate) i8);

/// The screen state.
#[derive(Debug, Format, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScreenState {
    /// The screen is off.
    Off,
    /// The screen is on.
    On,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Off
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self(Milliseconds(Self::MIN))
    }
}

impl Default for ScreenState {
    fn default() -> Self {
        Self::Off
    }
}

impl Format for Speed {
    fn format(&self, fmt: defmt::Formatter) {
        write!(fmt, "Speed(duration = {} ms)", self.0 .0);
    }
}

impl Brightness {
    /// The minimum brightness value.
    pub const MIN: u8 = 0;
    /// The maximum brightness value.
    pub const MAX: u8 = u8::MAX;

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
    /// The minimum transition time.
    pub const MIN: u32 = 100;
    /// The maximum transition time.
    pub const MAX: u32 = 13_000;

    /// Creates a new speed.
    pub fn new(transition_time: Milliseconds) -> Self {
        Self(transition_time)
    }

    /// Returns the duration of a transition.
    pub fn transition_time(&self) -> Generic<u32> {
        self.0.into()
    }
}

impl Temperature {
    /// The minimum temperature value.
    pub const MIN: i8 = -85;
    /// The maximum temperature value.
    pub const MAX: i8 = 85;

    /// Creates a new temperature.
    ///
    /// Negative value gives warmer colors, positive values colder.
    pub fn new(value: i8) -> Self {
        Self(value)
    }

    /// Returns the temperature value.
    pub fn value(&self) -> i8 {
        self.0
    }
}
