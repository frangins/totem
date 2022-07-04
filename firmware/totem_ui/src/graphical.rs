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

//! The graphical user interface for Totem.
//!
//! This interface uses ERCP Basic to command Totem from a GUI.

use ercp_basic::{ack, command::nack_reason, nack, Command};

use crate::{state::*, UI};

/// The graphical user interface for Totem.
#[derive(Default)]
pub struct GraphicalUI {
    state: UIState,
}

/// The UI_Update ERCP Basic command code.
pub const UI_UPDATE: u8 = 0x20;

impl GraphicalUI {
    /// Creates a new graphical UI.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the state.
    pub fn set_state(&mut self, state: UIState) {
        self.state = state;
    }
}

impl UI for GraphicalUI {
    fn read_mode(&mut self) -> Mode {
        self.state.mode
    }

    fn read_brightness(&mut self) -> Brightness {
        self.state.brightness
    }

    fn read_speed(&mut self) -> Speed {
        self.state.speed
    }

    fn read_temperature(&mut self) -> Temperature {
        self.state.temperature
    }

    fn read_screen_state(&mut self) -> ScreenState {
        self.state.screen_state
    }
}

/// Handles UI_Update commands.
pub fn ui_update<'a>(
    command: Command,
    ui_state_update: &mut Option<UIState>,
) -> Option<Command<'a>> {
    if command.code() != UI_UPDATE {
        return Some(nack!(nack_reason::INVALID_ARGUMENTS));
    }

    if let Ok(ui_state) = postcard::from_bytes(command.value()) {
        *ui_state_update = Some(ui_state);
        Some(ack!())
    } else {
        Some(nack!(nack_reason::INVALID_ARGUMENTS))
    }
}
