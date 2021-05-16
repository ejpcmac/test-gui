use gtk::{
    prelude::*,
    Orientation::{Horizontal, Vertical},
};
use relm::Widget;
use relm_derive::{widget, Msg};

use test_gui::{Color, Device};

pub struct Model {
    port: String,
    device: Option<Device>,
    status: String,
    color: Color,
}

#[derive(Msg)]
pub enum Msg {
    UpdatePort(String),
    UpdateHue(f64),
    UpdateSat(f64),
    UpdateVal(f64),
    Connect,
    Disconnect,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            port: String::new(),
            device: None,
            status: String::from("Disconnected"),
            color: Color {
                hue: 0,
                sat: 255,
                val: 100,
            },
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdatePort(port) => self.model.port = port,

            Msg::UpdateHue(hue) => {
                let hue = hue as u8;
                if hue != self.model.color.hue {
                    self.model.color.hue = hue;
                    if let Some(device) = &mut self.model.device {
                        device.update_color(&self.model.color).ok();
                    }
                }
            }

            Msg::UpdateSat(sat) => {
                let sat = sat as u8;
                if sat != self.model.color.sat {
                    self.model.color.sat = sat;
                    if let Some(device) = &mut self.model.device {
                        device.update_color(&self.model.color).ok();
                    }
                }
            }

            Msg::UpdateVal(val) => {
                let val = val as u8;
                if val != self.model.color.val {
                    self.model.color.val = val;
                    if let Some(device) = &mut self.model.device {
                        device.update_color(&self.model.color).ok();
                    }
                }
            }

            Msg::Connect => match Device::new(&self.model.port) {
                Ok(device) => {
                    self.model.device = Some(device);
                    self.widgets.connect_button.set_label("Disconnect");
                    // TODO: Also update the event.
                    self.model.status =
                        format!("Connected to {}.", self.model.port);
                }

                Err(error) => {
                    self.model.status =
                        format!("Error: {}.", error.to_string());
                }
            },

            Msg::Disconnect => {
                self.model.device = None;
                self.widgets.connect_button.set_label("Connect");
                // TODO: Also update the event.
                self.model.status = String::from("Disconnected");
            }

            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Entry {
                    placeholder_text: Some("TTY port path"),
                    changed(entry) => {
                        let port = entry.get_text().to_string();
                        Msg::UpdatePort(port)
                    },
                },

                #[name = "connect_button"]
                gtk::Button {
                    label: "Connect",
                    clicked => Msg::Connect,
                },

                gtk::Label {
                    text: &self.model.status,
                },

                // TODO: Ping button.

                gtk::Label {
                    text: "Hue",
                },

                gtk::Scale {
                    orientation: Horizontal,
                    adjustment: &gtk::Adjustment::new(0.0, 0.0, 255.0, 1.0, 1.0, 1.0),
                    change_value(_, _, value) => (Msg::UpdateHue(value), Inhibit(false)),
                },

                gtk::Label {
                    text: "Saturation",
                },

                gtk::Scale {
                    orientation: Horizontal,
                    adjustment: &gtk::Adjustment::new(255.0, 0.0, 255.0, 1.0, 1.0, 1.0),
                    change_value(_, _, value) => (Msg::UpdateSat(value), Inhibit(false)),
                },

                gtk::Label {
                    text: "Value",
                },

                gtk::Scale {
                    orientation: Horizontal,
                    adjustment: &gtk::Adjustment::new(100.0, 0.0, 255.0, 1.0, 1.0, 1.0),
                    change_value(_, _, value) => (Msg::UpdateVal(value), Inhibit(false)),
                },
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
