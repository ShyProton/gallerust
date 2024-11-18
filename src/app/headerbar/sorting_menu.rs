use relm4::gtk::prelude::*;
use relm4::gtk::{Box, CheckButton, Label, Orientation, Popover, Separator};
use relm4::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Alphabetical,
    ReverseAlphabetical,
    LastModified,
    FirstModified,
    LargestSize,
    SmallestSize,
    FileType,
}

#[derive(Debug)]
pub enum Output {
    OptionSelected(Input),
}

pub type Init = Input;

pub struct SortingMenu {
    initial_option: Init,
}

#[relm4::component(pub)]
impl SimpleComponent for SortingMenu {
    type Input = Input;
    type Output = Output;
    type Init = Init;

    view! {
        Popover {
            #[name = "sorting_radio_group"]
            Box {
                set_orientation: Orientation::Vertical,
                set_spacing: 0,
                set_margin_all: 10,

                Label {
                    set_label: "Sorting Options",
                    add_css_class: "title-4",
                },

                Separator {
                    set_margin_top: 3,
                    set_margin_bottom: 3
                },

                #[name = "alphabetical"]
                CheckButton {
                    set_label: Some("A-Z"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::Alphabetical,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::Alphabetical);
                        }
                    }
                },

                #[name = "reverse_alphabetical"]
                CheckButton {
                    set_label: Some("Z-A"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::ReverseAlphabetical,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::ReverseAlphabetical);
                        }
                    }
                },

                #[name = "last_modified"]
                CheckButton {
                    set_label: Some("Last Modified"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::LastModified,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::LastModified);
                        }
                    }
                },

                #[name = "first_modified"]
                CheckButton {
                    set_label: Some("First Modified"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::FirstModified,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::FirstModified);
                        }
                    }
                },

                #[name = "largest_size"]
                CheckButton {
                    set_label: Some("Largest Size"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::LargestSize,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::LargestSize);
                        }
                    }
                },

                #[name = "smallest_size"]
                CheckButton {
                    set_label: Some("Smallest Size"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::SmallestSize,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::SmallestSize);
                        }
                    }
                },

                #[name = "file_type"]
                CheckButton {
                    set_label: Some("File Type"),
                    set_group: Some(&alphabetical),
                    set_active: model.initial_option == Init::FileType,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.input(Input::FileType);
                        }
                    }
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            initial_option: init,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        let message_str = format!("{message:?}");

        sender
            .output(Output::OptionSelected(message))
            .unwrap_or_else(|_| log::error!("Could not set sorting option {message_str}"));
    }
}
