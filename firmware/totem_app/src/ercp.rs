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

//! ERCP Basic integration for the Totem application firmware.

use ercp_basic::Router;

#[cfg(feature = "ui_graphical")]
use totem_ui::state::UIState;

/// The context for the ERCP Basic router.
#[derive(Default)]
pub struct ErcpContext {
    #[cfg(feature = "ui_graphical")]
    /// The graphical UI state.
    pub ui_state: UIState,
}

/// The ERCP Basic router for Totem.
pub struct TotemRouter;

impl Router for TotemRouter {
    type Context = ErcpContext;

    fn route(
        &mut self,
        command: ercp_basic::Command,
        #[allow(unused)] ctx: &mut Self::Context,
    ) -> Option<ercp_basic::Command> {
        match command.code() {
            #[cfg(feature = "ui_graphical")]
            totem_ui::graphical::UI_UPDATE => {
                totem_ui::graphical::ui_update(command, &mut ctx.ui_state)
            }

            _ => self.default_routes(command),
        }
    }

    fn firmware_version(&self) -> &str {
        concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"))
    }

    fn description(&self) -> &str {
        env!("CARGO_PKG_DESCRIPTION")
    }
}
