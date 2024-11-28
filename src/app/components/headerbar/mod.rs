mod sorting_menu;
mod view_options;

use sorting_menu::SortingMenu;
use view_options::ViewOptions;

use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_names;

pub struct HeaderBarModel {
    sorting_menu: Controller<SortingMenu>,
    view_options: Controller<ViewOptions>,
}

#[derive(Debug)]
pub enum Input {
    SortingOption(sorting_menu::Input),
    ViewOption(view_options::Input),
}

pub type Output = Input;

#[relm4::component(pub)]
impl SimpleComponent for HeaderBarModel {
    type Input = Input;
    type Output = Output;
    type Init = ();

    view! {
        // TODO: Incorporate progress bar to show loading status of images.
        adw::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                set_hexpand: true,
                set_halign: gtk::Align::End,
                set_margin_all: 5,

                adw::SplitButton {
                    add_css_class: "raised",

                    set_label: "View",
                    set_icon_name: icon_names::GRID_FILLED,

                    set_popover = Some(model.sorting_menu.widget()),
                    set_dropdown_tooltip: "Sorting Options",
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

        let view_options: Controller<ViewOptions> = ViewOptions::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {});

        let model = Self {
            sorting_menu,
            view_options,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        let message_str = format!("{message:?}");

        sender.output(message).unwrap_or_else(|_| {
            log::error!("Could not send HeaderBar input to parent: {message_str}");
        });
    }
}
