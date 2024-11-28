use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct AlbumIcon {
    name: String,
}

#[relm4::factory(pub)]
impl FactoryComponent for AlbumIcon {
    type Init = Box<std::path::Path>;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "rounded",
            gtk::Label {
                set_label: self.name.as_str()
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let name = init.file_name().unwrap().to_str().unwrap().to_string();
        Self { name }
    }
}
