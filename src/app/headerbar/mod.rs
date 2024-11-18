mod sorting_menu;

use sorting_menu::SortingMenu;

use relm4::gtk::prelude::*;
use relm4::gtk::{Align, Box, HeaderBar, MenuButton};
use relm4::prelude::*;
use relm4_icons::icon_names;

pub struct HeaderModel {
    sorting_menu: Controller<SortingMenu>,
}

#[derive(Debug)]
pub enum Input {
    SortingOption(sorting_menu::Input),
}

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
                set_hexpand: true,
                set_halign: Align::End,
                set_margin_all: 5,

                Box {
                    add_css_class: "linked",

                    MenuButton {
                        set_label: "View options",
                        set_icon_name: icon_names::LIST_LARGE,
                    },

                    MenuButton {
                        set_label: "Sorting options",
                        set_icon_name: icon_names::DOWN_SMALL,

                        set_popover = Some(model.sorting_menu.widget()),
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let sorting_menu: Controller<SortingMenu> = SortingMenu::builder()
            .launch(sorting_menu::Init::Alphabetical) // TODO: Replace with saved setting.
            .forward(sender.input_sender(), |msg| match msg {
                sorting_menu::Output::OptionSelected(option) => Input::SortingOption(option),
            });

        let model = Self { sorting_menu };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Input::SortingOption(option) => log::info!("{option:?}"), // TODO: Save setting.
        }
    }
}
