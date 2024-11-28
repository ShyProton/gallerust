use app::AppModel;
use relm4::RelmApp;

mod app;

fn main() {
    env_logger::init();
    relm4_icons::initialize_icons();

    adw::init().expect("Failed to initialize libadwaita.");

    let app = RelmApp::new("com.github.ShyProton.Gallerust");
    app.run::<AppModel>(0);
}
