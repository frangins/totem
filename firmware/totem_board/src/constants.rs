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

//! Constants of the Totem board.

/// The number of LEDs per strip.
pub const LEDS_PER_STRIP: usize = 13;

/// The number of LEDs per half strip.
pub const LEDS_PER_HALF_STRIP: usize = 7;

/// The number of LED strips per side.
pub const STRIPS_PER_SIDE: usize = 2;

/// The total number of LEDs.
pub const NUM_LEDS: usize = LEDS_PER_STRIP * STRIPS_PER_SIDE * 4;

/// The size of the buffer for the LED driver.
pub const LED_BUFFER_SIZE: usize = NUM_LEDS * 12 + 20;
