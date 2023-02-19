/* MIT License
 *
 * Copyright (c) 2023 David Hewitt
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use gettextrs::gettext;
use gtk::subclass::prelude::*;
use gtk::{gio, gio::Settings, glib, prelude::*};
use once_cell::sync::OnceCell;

use crate::config::APP_ID;

mod imp {
    use crate::welcome::WelcomeView;

    use super::*;

    #[derive(Debug, Default)]
    pub struct ElementaryRustExampleWindow {
        pub settings: OnceCell<Settings>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ElementaryRustExampleWindow {
        const NAME: &'static str = "ElementaryRustExampleWindow";
        type Type = super::ElementaryRustExampleWindow;
        type ParentType = gtk::ApplicationWindow;
    }

    impl ObjectImpl for ElementaryRustExampleWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_settings();
            obj.load_window_size();

            let mode_switch = granite::ModeSwitch::builder()
                .primary_icon_name("display-brightness-symbolic")
                .secondary_icon_name("weather-clear-night-symbolic")
                .primary_icon_tooltip_text(&gettext("Light Background"))
                .secondary_icon_tooltip_text(&gettext("Dark Background"))
                .valign(gtk::Align::Center)
                .build();

            let gtk_settings = gtk::Settings::default().expect("Unable to get GtkSettings object");
            mode_switch
                .bind_property("active", &gtk_settings, "gtk-application-prefer-dark-theme")
                .bidirectional()
                .build();

            let headerbar = gtk::HeaderBar::builder().show_title_buttons(true).build();

            headerbar.style_context().add_class("default-decoration");
            headerbar.pack_end(&mode_switch);

            obj.set_titlebar(Some(&headerbar));
            obj.set_child(Some(&WelcomeView::new()));
        }
    }
    impl WidgetImpl for ElementaryRustExampleWindow {}
    impl WindowImpl for ElementaryRustExampleWindow {
        fn close_request(&self) -> glib::signal::Inhibit {
            self.obj()
                .save_window_size()
                .expect("Failed to save window state");

            self.parent_close_request()
        }
    }
    impl ApplicationWindowImpl for ElementaryRustExampleWindow {}
}

glib::wrapper! {
    pub struct ElementaryRustExampleWindow(ObjectSubclass<imp::ElementaryRustExampleWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl ElementaryRustExampleWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .property("title", "Elementary Rust Sample")
            .build()
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let size = self.default_size();

        self.settings().set_int("window-width", size.0)?;
        self.settings().set_int("window-height", size.1)?;
        self.settings()
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let width = self.settings().int("window-width");
        let height = self.settings().int("window-height");
        let is_maximized = self.settings().boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
