mod components;
mod pages;

use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

use components::appview;
use components::headerbar;

use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub struct AppModel {
    app_view: Controller<appview::AppViewModel>,
}

#[derive(Debug)]
pub enum Input {
    FromHeaderBar(headerbar::Input),
    FromAlbum(pages::album::Input),
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    /// The type of the messages that this component can receive.
    type Input = Input;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = u8;

    view! {
        adw::ApplicationWindow {
            set_title: Some("Gallerust"),

            #[wrap(Some)]
            set_content = model.app_view.widget(),
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let app_view: Controller<appview::AppViewModel> = appview::AppViewModel::builder()
            .launch(())
            .forward(sender.input_sender(), |()| todo!());

        let model = Self { app_view };
        let widgets = view_output!();

        let base_album = pages::album::Album::builder()
            .launch(std::path::Path::new("~/Pictures"))
            .forward(sender.input_sender(), |input| todo!());

        model.app_view.widget().push(base_album.widget());

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        match message {
            Input::FromHeaderBar(input) => self.handle_header_input(input),
            // Input::FromAlbum(input) => self.handle_app_input(input),
            Input::FromAlbum(input) => todo!(),
        }
    }
}

impl AppModel {
    fn handle_header_input(&self, input: headerbar::Input) {
        match input {
            headerbar::Input::SortingOption(option) => {
                log::info!("Sorting option selected: {option:?}");
            }
            headerbar::Input::ViewOption(option) => {
                log::info!("View option selected: {option:?}");
            }
        }
    }

    // fn handle_app_input(&self, input: pages::album::Input) {
    //     let binding = pages::album::Album::builder().launch(());
    //     let page = binding.widget();

    //     match input {
    //         pages::album::Input::AlbumSelect(path) => self.app_view.widget().push(page), // TODO: Build album page and push.
    //         pages::album::Input::ImageSelect(path) => self.app_view.widget().push(page), // TODO: Build image page and push.
    //     }
    // }
}
