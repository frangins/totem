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

#![deny(unused_must_use)]
#![forbid(unsafe_code)]

use gtk::{
    prelude::*,
    Orientation::{Horizontal, Vertical},
};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

use embedded_time::duration::Milliseconds;
use ercp_device::Device;
use totem_ui::state::{Brightness, Mode, Speed, Temperature, UIState};

use totem_gui::device::{DeviceExt, TIMEOUT};

struct AppModel {
    port: String,
    device: Option<Device>,
    connection_status: String,
    ping_status: String,
    ui_state: UIState,
}

enum AppMsg {
    UpdatePort(String),
    UpdateMode(Mode),
    UpdateBrightness(Brightness),
    UpdateSpeed(Speed),
    UpdateTemperature(Temperature),
    Connect,
    Ping,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            port: String::from("/dev/ttyACM0"),
            device: None,
            connection_status: String::from("Disconnected."),
            ping_status: String::from("Not yet."),
            ui_state: UIState::default(),
        }
    }
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: AppMsg,
        _components: &(),
        _sender: Sender<AppMsg>,
    ) -> bool {
        match msg {
            AppMsg::UpdatePort(port) => self.port = port,

            AppMsg::UpdateMode(mode) => {
                self.ui_state.mode = mode;
                if let Some(device) = &mut self.device {
                    device.ui_update(&self.ui_state).ok();
                }
            }

            AppMsg::UpdateBrightness(brightness) => {
                if brightness != self.ui_state.brightness {
                    self.ui_state.brightness = brightness;
                    if let Some(device) = &mut self.device {
                        device.ui_update(&self.ui_state).ok();
                    }
                }
            }

            AppMsg::UpdateSpeed(speed) => {
                if speed != self.ui_state.speed {
                    self.ui_state.speed = speed;
                    if let Some(device) = &mut self.device {
                        device.ui_update(&self.ui_state).ok();
                    }
                }
            }

            AppMsg::UpdateTemperature(temperature) => {
                if temperature != self.ui_state.temperature {
                    self.ui_state.temperature = temperature;
                    if let Some(device) = &mut self.device {
                        device.ui_update(&self.ui_state).ok();
                    }
                }
            }

            AppMsg::Connect => match Device::new(&self.port) {
                Ok(device) => {
                    self.device = Some(device);
                    self.connection_status =
                        format!("Connected to {}.", self.port);
                }

                Err(error) => {
                    self.connection_status = format!("Error: {}.", error);
                }
            },

            AppMsg::Ping => {
                if let Some(device) = &mut self.device {
                    match device.ping(TIMEOUT) {
                        Ok(Ok(())) => {
                            self.ping_status = String::from("Pong!");
                        }

                        _ => {
                            self.ping_status = String::from("Error :(");
                        }
                    }
                }
            }
        }

        true
    }
}

#[relm4_macros::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Totem GUI"),
            set_child = Some(&gtk::Box) {
                set_orientation: Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Entry {
                    set_placeholder_text: Some("/dev/ttyACM0"),
                    connect_changed(sender) => move |entry| {
                        let port = entry.text().to_string();
                        send!(sender, AppMsg::UpdatePort(port));
                    }
                },

                append = &gtk::Box {
                    set_orientation: Horizontal,
                    set_homogeneous: true,

                    append = &gtk::Button {
                        set_label: "Connect",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Connect);
                        },
                    },

                    append = &gtk::Label {
                        set_label: watch! { &model.connection_status },
                    }
                },

                append = &gtk::Box {
                    set_orientation: Horizontal,
                    set_homogeneous: true,

                    append = &gtk::Button {
                        set_label: "Ping",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::Ping);
                        },
                    },

                    append = &gtk::Label {
                        set_label: watch! { &model.ping_status },
                    }
                },

                append = &gtk::Box {
                    set_orientation: Vertical,

                    append = &gtk::Label {
                        set_label: "Mode",
                    },

                    append = &gtk::Box {
                        set_orientation: Horizontal,

                        append: mode_selector = &gtk::CheckButton {
                            set_label: Some("Off"),
                            set_active: true,
                            connect_toggled(sender) => move |button| {
                                if button.is_active() {
                                    send!(sender, AppMsg::UpdateMode(Mode::Off));
                                }
                            },
                        },

                        append = &gtk::CheckButton {
                            set_label: Some("Random unicolor"),
                            set_group: Some(&mode_selector),
                            connect_toggled(sender) => move |button| {
                                if button.is_active() {
                                    send!(sender, AppMsg::UpdateMode(Mode::RandomUnicolor));
                                }
                            },
                        },
                    },
                },

                append = &gtk::Box {
                    set_orientation: Vertical,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_label: "Brightness",
                    },

                    append = &gtk::Scale {
                        set_orientation: Horizontal,
                        set_adjustment: &gtk::Adjustment::new(0.0, 0.0, 255.0, 1.0, 1.0, 1.0),
                        connect_value_changed(sender) => move |value| {
                            let brightness = Brightness::new(value.value() as u8);
                            send!(sender, AppMsg::UpdateBrightness(brightness));
                        },
                    },
                },

                append = &gtk::Box {
                    set_orientation: Vertical,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_label: "Speed",
                    },

                    append = &gtk::Scale {
                        set_orientation: Horizontal,
                        set_adjustment: &gtk::Adjustment::new(100.0, 100.0, 13_000.0, 1.0, 1.0, 1.0),
                        connect_value_changed(sender) => move |value| {
                            let speed = Speed::new(Milliseconds(value.value() as u32));
                            send!(sender, AppMsg::UpdateSpeed(speed));
                        },
                    },
                },

                append = &gtk::Box {
                    set_orientation: Vertical,
                    set_homogeneous: true,

                    append = &gtk::Label {
                        set_label: "Temperature",
                    },

                    append = &gtk::Scale {
                        set_orientation: Horizontal,
                        set_adjustment: &gtk::Adjustment::new(0.0, -85.0, 85.0, 1.0, 1.0, 1.0),
                        connect_value_changed(sender) => move |value| {
                            let temperature = Temperature::new(value.value() as i8);
                            send!(sender, AppMsg::UpdateTemperature(temperature));
                        },
                    },
                },
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
