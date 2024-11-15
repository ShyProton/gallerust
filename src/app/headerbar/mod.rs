use relm4::gtk::prelude::*;
use relm4::gtk::{Box, Button, HeaderBar, MenuButton};
use relm4::prelude::*;
use relm4::{gtk, view};
use relm4_icons::icon_names;

pub struct HeaderModel;

#[derive(Debug)]
pub enum Input {}

#[relm4::component(pub)]
impl SimpleComponent for HeaderModel {
    type Input = Input;
    type Output = ();
    type Init = ();

    view! {
        HeaderBar {
            set_show_title_buttons: true,

            #[wrap(Some)]
            set_title_widget = &Box {
                gtk::Image {
                    set_icon_size: gtk::IconSize::Large,
                    set_icon_name: Some(icon_names::PLUS)
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
