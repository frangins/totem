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

//! Board support crate for Totem.

#![no_std]
#![allow(non_camel_case_types)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::use_self)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

pub use stm32l4xx_hal as hal;

pub use crate::hal::prelude;
pub use crate::hal::*;
pub use crate::pac::*;
pub use cortex_m::*;

pub mod board;
