mod headerbar;
mod welcome;

use granite::traits::SettingsExt;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "io.github.davidmhewitt.elementary-rust-example";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let gtk_settings = gtk::Settings::default().unwrap();

    let granite_settings = granite::Settings::default().unwrap();
    gtk_settings.set_gtk_application_prefer_dark_theme(
        granite_settings.prefers_color_scheme() == granite::SettingsColorScheme::Dark,
    );

    let welcome = welcome::WelcomeView::build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Template")
        .child(&welcome)
        .build();

    window.set_titlebar(Some(&headerbar::HeaderBar::build()));

    window.present();

    granite_settings.connect_prefers_color_scheme_notify(
        clone!(@weak gtk_settings => move |granite_settings| {
            gtk_settings.set_gtk_application_prefer_dark_theme(
                granite_settings.prefers_color_scheme() == granite::SettingsColorScheme::Dark,
            );
        }),
    );
}
