// totem_gui - A graphical user interface for controlling the Totem.
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

//! ERCP device extention for Totem.

use std::time::Duration;

use ercp_device::{CustomCommandError, Device};
use totem_ui::{graphical::UI_UPDATE, state::UIState};

/// The timeout when communication with the Totem.
pub const TIMEOUT: Option<Duration> = Some(Duration::from_millis(100));

/// ERCP device extention for Totem.
pub trait DeviceExt {
    /// Updates the UI.
    fn ui_update(&mut self, state: &UIState) -> Result<(), CustomCommandError>;
}

impl DeviceExt for Device {
    fn ui_update(&mut self, state: &UIState) -> Result<(), CustomCommandError> {
        let value = postcard::to_allocvec(state).unwrap();
        self.command(UI_UPDATE, &value, TIMEOUT)?;
        Ok(())
    }
}
