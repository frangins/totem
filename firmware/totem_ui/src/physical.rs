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

//! The physical user interface for Totem.

use core::ops::Range;

use embedded_time::duration::Milliseconds;
use totem_board::{
    adc::{Channel, ADC},
    peripheral::CalibratedPotentiometer,
    prelude::*,
};

use crate::{state::*, UI};

/// The physical user interface for Totem.
pub struct PhysicalUI<PMode, PBrightness, PSpeed, PTemperature> {
    p_adc: ADC,
    p_mode: PMode,
    p_brightness: PBrightness,
    p_speed: PSpeed,
    p_temperature: PTemperature,
}

const ITERATIONS: u32 = 200;

impl<
        PMode: CalibratedPotentiometer,
        PBrightness: CalibratedPotentiometer,
        PSpeed: CalibratedPotentiometer,
        PTemperature: CalibratedPotentiometer,
    > PhysicalUI<PMode, PBrightness, PSpeed, PTemperature>
{
    /// Creates a new physical UI.
    pub fn new(
        p_adc: ADC,
        p_mode: PMode,
        p_brightness: PBrightness,
        p_speed: PSpeed,
        p_temperature: PTemperature,
    ) -> Self {
        Self {
            p_adc,
            p_mode,
            p_brightness,
            p_speed,
            p_temperature,
        }
    }
}

impl<
        PMode: CalibratedPotentiometer,
        PBrightness: CalibratedPotentiometer,
        PSpeed: CalibratedPotentiometer,
        PTemperature: CalibratedPotentiometer,
    > UI for PhysicalUI<PMode, PBrightness, PSpeed, PTemperature>
{
    fn read_mode(&mut self) -> Mode {
        let value = read_mean(&mut self.p_adc, &mut self.p_mode, ITERATIONS);

        if value < (PMode::MAX - PMode::MIN) / 2 {
            Mode::Off
        } else {
            Mode::RandomUnicolor
        }
    }

    fn read_brightness(&mut self) -> Brightness {
        let value =
            read_mean(&mut self.p_adc, &mut self.p_brightness, ITERATIONS);

        Brightness(adc_to_range(
            value,
            PBrightness::MIN..PBrightness::MAX,
            (Brightness::MIN.into())..(Brightness::MAX.into()),
        ) as u8)
    }

    fn read_speed(&mut self) -> Speed {
        let value = read_mean(&mut self.p_adc, &mut self.p_speed, ITERATIONS);
        let transition_ms = adc_to_inverted_range(
            value,
            PSpeed::MIN..PSpeed::MAX,
            (Speed::MIN as i32)..(Speed::MAX as i32),
        );

        Speed(Milliseconds(transition_ms as u32))
    }

    fn read_temperature(&mut self) -> Temperature {
        let value =
            read_mean(&mut self.p_adc, &mut self.p_temperature, ITERATIONS);

        Temperature(adc_to_range(
            value,
            PTemperature::MIN..PTemperature::MAX,
            (Temperature::MIN.into())..(Temperature::MAX.into()),
        ) as i8)
    }
}

fn read_mean(
    adc: &mut ADC,
    channel: &mut impl Channel,
    iterations: u32,
) -> u16 {
    ((0..iterations)
        .fold(0, |sum: u32, _| sum + adc.read(channel).unwrap() as u32)
        / iterations as u32) as u16
}

fn adc_to_range(
    adc_value: u16,
    adc_range: Range<u16>,
    output_range: Range<i32>,
) -> i32 {
    let adc_dynamic = adc_range.len() as i32;
    let output_dynamic = output_range.len() as i32;

    // Value in 0..adc_dynamic.
    let base_value = adc_value.saturating_sub(adc_range.start) as i32;

    // Value in 0..output_dynamic.
    let scaled_value = if adc_dynamic >= output_dynamic {
        let scale = adc_dynamic / output_dynamic;
        base_value / scale
    } else {
        // As the scaling is truncated, it can be insufficient to achieve the
        // maximum value. Hence let’s add 1.
        let scale = output_dynamic / adc_dynamic + 1;
        base_value.saturating_mul(scale)
    };

    // Value in output_range.
    scaled_value
        .saturating_add(output_range.start)
        .min(output_range.end)
}

fn adc_to_inverted_range(
    value: u16,
    adc_range: Range<u16>,
    output_range: Range<i32>,
) -> i32 {
    let output_dynamic = output_range.len() as i32;
    output_range.end - adc_to_range(value, adc_range, 0..output_dynamic)
}
