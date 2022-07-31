pub enum Mode {
    /// The static RGB mode.
    ///
    /// All LEDs are in the same static color, controlled by RGB potentiometers.
    StaticRgb,

    /// The random unicolor mode.
    ///
    /// All LEDs are following the same random progression. The color
    /// temperature can be controlled to constrain the randomness.
    RandomUnicolor {
        /// The brightness of the LED strip.
        brightness: Brightness,
        /// The speed of transitions.
        speed: Speed,
        /// The color temperature.
        temperature: Temperature,
    },

    /// The cutted random unicolor mode.
    ///
    /// Like RandomUnicolor, but with regular cuts.
    CuttedRandomUnicolor {
        /// The brightness of the LED strip.
        brightness: Brightness,
        /// The speed of transitions.
        speed: Speed,
        /// The color temperature.
        temperature: Temperature,
        /// The period of the cuts.
        period: TODO,
    },

    /// The rainbow fontain mode.
    ///
    /// LED strips show a symmetrical rainbow convergence / divergence.
    RainbowFontain {
        /// The brightness of the LED strip.
        brightness: Brightness,
        /// The speed of transitions.
        speed: Speed,
    },

    /// The random fontain mode.
    ///
    /// LED strips show a symmetrical convergence / divergence of a bi-color
    /// pattern. Can be controlled: the color distance between the main and
    /// secondary color, and the size of the color blocks.
    RandomFontain {
        /// The brightness of the LED strip.
        brightness: Brightness,
        /// The speed of transitions.
        speed: Speed,
        /// The color temperature.
        temperature: Temperature,
        pattern_size: PatternSize,
        color_distance: ColorDistance,
    },

    /// The random bouncer mode.
    RandomUnicolorBouncer,

    /// The random firework mode.
    ///
    /// A color starts in the center of a strip and goes accerelating in both
    /// directions, as a moving flash.
    RandomFirework,
}

// TODO: Constructor with limitation + Default implementation (2).
/// The size of patterns.
#[derive(Debug, Format, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PatternSize(pub(crate) u8);

/// The color distance.
#[derive(
    Debug, Format, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColorDistance(pub(crate) u8);

/// An error that can happen when creating a `PatternSize`.
pub enum NewPatternSizeError {
    /// The pattern size is too big.
    TooBig,
}

impl Default for PatternSize {
    fn default() -> Self {
        Self(4)
    }
}

impl PatternSize {
    /// Creates a new pattern size.
    ///
    /// The `value` is the size of the complete pattern.
    pub fn new(value: u8) -> Result<Self, NewPatternSizeError> {
        if value as usize <= totem_board::constants::LEDS_PER_STRIP {
            Ok(Self(value))
        } else {
            Err(NewPatternSizeError::TooBig)
        }
    }

    /// Returns the pattern size value.
    pub fn value(&self) -> u8 {
        self.0
    }
}
