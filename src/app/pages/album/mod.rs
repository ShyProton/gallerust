mod album_icon;
mod image_icon;

use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

use crate::app::components::headerbar;

pub struct Album<'a: 'static> {
    path: &'a std::path::Path,
    name: &'a str,
    header_bar: Controller<headerbar::HeaderBarModel>,
    album_icons: FactoryVecDeque<album_icon::AlbumIcon>,
    // image_icons: FactoryVecDeque<image_icon::ImageIcon>
}

#[derive(Debug)]
pub enum Input {
    AlbumSelect(Box<std::path::Path>),
    ImageSelect(Box<std::path::Path>),
}

type Output = Input;
type Init = Box<std::path::Path>;

#[relm4::component(pub)]
impl<'a: 'static> SimpleComponent for Album<'a> {
    type Input = Input;
    type Output = Output;
    type Init = &'a std::path::Path;

    view! {
        adw::NavigationPage {
            set_title: model.name,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = model.header_bar.widget(),

                #[wrap(Some)]
                set_content = &gtk::Box {
                    #[local_ref]
                    album_icons_box -> gtk::Box {
                        set_spacing: 10,
                    },
                },
            }
        }
    }

    fn init(
        path: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header_bar = headerbar::HeaderBarModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_| todo!());

        let album_icons = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| todo!());

        let name = path.file_name().unwrap().to_str().unwrap();

        let model = Self {
            path,
            name,
            header_bar,
            album_icons,
        };

        let album_icons_box = model.album_icons.widget();
        let widgets = view_output!();

        log::info!("Album page has been initialized with path: {path:?}");

        ComponentParts { model, widgets }
    }
}
