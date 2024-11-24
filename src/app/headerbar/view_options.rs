use adw::prelude::*;
use adw::SpinRow;
use relm4::gtk::prelude::*;

use relm4::prelude::*;
use relm4_icons::icon_names;

pub struct ViewOptions {}

#[derive(Debug)]
pub enum Input {
    SizeChange(f64),
    ShowName(bool),
}

#[derive(Debug)]
pub enum Output {}

#[relm4::component(pub)]
impl SimpleComponent for ViewOptions {
    type Input = Input;
    type Output = Output;
    type Init = (); // TODO: Saved options.

    view! {
        gtk::Popover {
            set_size_request: (300, -1),
            adw::PreferencesGroup {
                set_title: "View Options",
                set_margin_all: 10,
                adw::SpinRow {
                    set_title: "Icon Size",

                    #[wrap(Some)]
                    set_adjustment = &gtk::Adjustment {
                        set_value: 24.0,
                        set_lower: 12.0,
                        set_upper: 60.0,
                        set_step_increment: 12.0,
                        set_page_increment: 10.0,
                        set_page_size: 0.0,
                    },

                    set_snap_to_ticks: true,
                },
                adw::SwitchRow {
                    set_title: "Show Image Name",
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
