use app::Model;
use relm4::RelmApp;

mod app;

fn main() {
    relm4_icons::initialize_icons();
    let app = RelmApp::new("com.github.ShyProton.Gallerust");
    app.run::<Model>(0);
}
