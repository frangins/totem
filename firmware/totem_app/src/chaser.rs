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

//! Abstraction over the chasers used by the Totem application firmware.

use led_effects::{
    chaser::RandomUnicolor, sequence::OneParameterSequenceEnum,
    time::TimeConfig,
};
use rand::distributions::Uniform;

/// A Totem chaser.
pub enum Chaser<const N: usize> {
    /// No chaser.
    None,

    /// A random unicolor chaser.
    RandomUnicolor(RandomUnicolor<Uniform<i16>, Uniform<u32>, N>),
}

impl<const N: usize> led_effects::chaser::Chaser<N> for Chaser<N> {
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        match self {
            Self::None => (),
            Self::RandomUnicolor(chaser) => chaser.set_time_config(time_config),
        }
    }
}

impl<const N: usize> Iterator for Chaser<N> {
    type Item = OneParameterSequenceEnum<N>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::RandomUnicolor(chaser) => chaser.next().map(Into::into),
        }
    }
}
