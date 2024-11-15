mod headerbar;

use headerbar::HeaderModel;
use relm4::gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    RelmWidgetExt, SimpleComponent,
};

pub struct Model {
    header: Controller<HeaderModel>,
}

#[derive(Debug)]
pub enum Input {
    Increment,
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for Model {
    /// The type of the messages that this component can receive.
    type Input = Input;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = u8;

    view! {
        gtk::Window {
            set_title: Some("Gallerust"),
            set_titlebar: Some(model.header.widget()),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "among us",
                    connect_clicked => Input::Increment
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked => Input::Decrement
                },

            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header: Controller<HeaderModel> = HeaderModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| todo!());

        let model = Self { header };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
    //     match message {
    //         Input::Increment => self.counter = self.counter.wrapping_add(1),
    //         Input::Decrement => self.counter = self.counter.wrapping_sub(1),
    //     }
    // }
}
