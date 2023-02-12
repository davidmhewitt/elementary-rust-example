use gtk::{
    prelude::*,
    traits::{StyleContextExt, WidgetExt},
};

pub struct HeaderBar;

impl HeaderBar {
    pub fn build() -> gtk::WindowHandle {
        let gtk_settings = gtk::Settings::default().unwrap();

        let mode_switch = granite::ModeSwitch::builder()
            .primary_icon_name("display-brightness-symbolic")
            .secondary_icon_name("weather-clear-night-symbolic")
            .primary_icon_tooltip_text("Light Background")
            .secondary_icon_tooltip_text("Dark Background")
            .build();

        mode_switch.set_valign(gtk::Align::Center);
        mode_switch
            .bind_property("active", &gtk_settings, "gtk-application-prefer-dark-theme")
            .bidirectional()
            .build();

        let headerbar = gtk::HeaderBar::builder().show_title_buttons(true).build();

        headerbar.style_context().add_class("default-decoration");
        headerbar.pack_end(&mode_switch);

        gtk::WindowHandle::builder().child(&headerbar).build()
    }
}
