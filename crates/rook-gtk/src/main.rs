#[rustfmt::skip]
mod config;
mod app;
mod modals;

use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::LocaleCategory;
use gtk::prelude::ApplicationExt;
use gtk::{gio, glib};
use relm4::{RelmApp, gtk, main_application};

use app::App;

use crate::config::{APP_NAME, PATH_ID};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn setup_gettext() {
    unsafe {
        gettextrs::setlocale(LocaleCategory::LcAll, "");
    };
    gettextrs::bindtextdomain(*GETTEXT_PACKAGE, *LOCALEDIR)
        .expect("Unable to bind the text domain");
    gettextrs::textdomain(*GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
}

fn setup_resources() {
    let res: gio::Resource =
        gio::Resource::load(*RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);
    let data = res
        .lookup_data(
            format!("{}style.css", *PATH_ID).as_ref(),
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    relm4::set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());
}

fn main() {
    gtk::init().unwrap();

    // Enable logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rook=info,relm4=warn".into()),
        )
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();

    setup_gettext();
    setup_resources();

    glib::set_application_name(*APP_NAME);
    gtk::Window::set_default_icon_name(*APP_ID);

    let app = main_application();
    app.set_resource_base_path(Some(*PATH_ID));

    let app = RelmApp::from_app(app);
    app.visible_on_activate(false).run::<App>(());
}
