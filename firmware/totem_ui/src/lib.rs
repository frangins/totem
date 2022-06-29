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

#[cfg(feature = "graphical")]
pub mod graphical;
#[cfg(feature = "physical")]
pub mod physical;
pub mod state;

#[cfg(feature = "graphical")]
pub use graphical::GraphicalUI;
#[cfg(feature = "physical")]
pub use physical::PhysicalUI;

use state::{Brightness, Mode, Speed, Temperature, UIState};

/// The user interface for Totem.
pub trait UI {
    /// Reads the current state of the UI.
    fn read_state(&mut self) -> UIState {
        UIState {
            mode: self.read_mode(),
            brightness: self.read_brightness(),
            speed: self.read_speed(),
            temperature: self.read_temperature(),
        }
    }

    /// Reads the value of the mode potentiometer.
    fn read_mode(&mut self) -> Mode;

    /// Reads the value of the brightness potentiometer.
    fn read_brightness(&mut self) -> Brightness;

    /// Reads the value of the speed potentiometer.
    fn read_speed(&mut self) -> Speed;

    /// Reads the value of the temperature potentiometer.
    fn read_temperature(&mut self) -> Temperature;
}
