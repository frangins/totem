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
    chaser::{RainbowChaser, RandomUnicolor},
    sequence::{
        ConfigWithMainColor, Duplicate, DuplicateConfig, Rainbow,
        RainbowConfig, Symmetry, Unicolor, UnicolorConfig,
    },
    time::TimeConfig,
};
use rand::distributions::Uniform;
use smart_leds::RGB8;
use totem_board::constants::{LEDS_PER_HALF_STRIP, LEDS_PER_STRIP, NUM_LEDS};
use totem_ui::state::Temperature;

type SymmetricRainbow = Duplicate<
    Symmetry<Rainbow<LEDS_PER_HALF_STRIP>, LEDS_PER_STRIP, LEDS_PER_HALF_STRIP>,
    NUM_LEDS,
    LEDS_PER_STRIP,
>;

/// A Totem chaser.
pub enum Chaser {
    /// No chaser.
    None,

    /// A random unicolor chaser.
    RandomUnicolor(RandomUnicolor<Uniform<i16>, Uniform<u32>, NUM_LEDS>),
    /// A rainbow fontain chaser.
    RainbowFontain(RainbowChaser<SymmetricRainbow, NUM_LEDS>),
}

/// A Totem sequence.
pub enum Sequence {
    /// A unicolor sequence.
    Unicolor(Unicolor<RGB8, NUM_LEDS>),
    /// A symmetric rainbow sequence.
    SymmetricRainbow(SymmetricRainbow),
}

/// A Totem sequence configuration.
#[derive(Clone, Copy)]
pub enum Config {
    /// A unicolor sequence configuration.
    Unicolor(UnicolorConfig<RGB8>),
    /// A rainbow sequence configuration.
    Rainbow(RainbowConfig),
}

impl led_effects::chaser::Chaser<NUM_LEDS> for Chaser {
    fn set_time_config(&mut self, time_config: &TimeConfig) {
        match self {
            Self::None => (),
            Self::RandomUnicolor(chaser) => chaser.set_time_config(time_config),
            Self::RainbowFontain(chaser) => chaser.set_time_config(time_config),
        }
    }
}

impl Iterator for Chaser {
    type Item = Sequence;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::None => None,
            Self::RandomUnicolor(chaser) => {
                chaser.next().map(Sequence::Unicolor)
            }
            Self::RainbowFontain(chaser) => {
                chaser.next().map(Sequence::SymmetricRainbow)
            }
        }
    }
}

impl Chaser {
    /// Sets the color temperature.
    pub fn set_temperature(&mut self, temperature: Temperature) {
        match self {
            Self::None => (),
            Self::RandomUnicolor(chaser) => {
                chaser.set_temperature(temperature.value())
            }
            Self::RainbowFontain(_) => (),
        }
    }
}

impl led_effects::sequence::Sequence<NUM_LEDS> for Sequence {
    type Config = Config;

    fn new(config: Self::Config) -> Self {
        match config {
            Config::Unicolor(config) => Self::Unicolor(Unicolor::new(config)),
            Config::Rainbow(config) => {
                Self::SymmetricRainbow(Duplicate::new(DuplicateConfig {
                    config,
                    duplicates: 8,
                }))
            }
        }
    }

    fn config(&self) -> Self::Config {
        match self {
            Sequence::Unicolor(sequence) => Config::Unicolor(sequence.config()),
            Sequence::SymmetricRainbow(sequence) => {
                Config::Rainbow(sequence.config().config)
            }
        }
    }
}

impl Iterator for Sequence {
    type Item = RGB8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Unicolor(sequence) => sequence.next(),
            Self::SymmetricRainbow(sequence) => sequence.next(),
        }
    }
}

impl ConfigWithMainColor for Config {
    fn main_color(&self) -> RGB8 {
        match self {
            Config::Unicolor(config) => config.main_color(),
            Config::Rainbow(config) => config.main_color(),
        }
    }

    fn set_main_color(&mut self, color: RGB8) {
        match self {
            Config::Unicolor(config) => config.set_main_color(color),
            Config::Rainbow(config) => config.set_main_color(color),
        }
    }
}
