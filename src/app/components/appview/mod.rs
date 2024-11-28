use adw::prelude::*;
use gtk::prelude::*;
use relm4::prelude::*;

pub struct AppViewModel {}

#[relm4::component(pub)]
impl SimpleComponent for AppViewModel {
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        adw::NavigationView {

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
