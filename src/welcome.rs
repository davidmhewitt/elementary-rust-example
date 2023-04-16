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
use granite::traits::PlaceholderExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use granite::traits::ToastExt;
    use gtk::glib::clone;

    use super::*;

    #[derive(Debug, Default)]
    pub struct WelcomeView;

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomeView {
        const NAME: &'static str = "WelcomeView";
        type Type = super::WelcomeView;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for WelcomeView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let welcome = granite::Placeholder::builder()
                .title(gettext("Welcome"))
                .build();

            let source_button = welcome
                .append_button(
                    &gio::ThemedIcon::new("applications-development"),
                    &gettext("Info"),
                    &gettext("Learn more about this application"),
                )
                .expect("Unable to construct button in welcome view");

            source_button.connect_clicked(|_| {
                gtk::show_uri(
                    gtk::Window::NONE,
                    "https://github.com/davidmhewitt/elementary-rust-example",
                    gtk::gdk::CURRENT_TIME,
                );
            });

            let docs_button = welcome
                .append_button(
                    &gio::ThemedIcon::new("help-contents"),
                    &gettext("Docs"),
                    &gettext("Rust docs for the Granite crate"),
                )
                .expect("Unable to construct button in welcome view");

            docs_button.connect_clicked(|_| {
                gtk::show_uri(
                    gtk::Window::NONE,
                    "https://docs.rs/granite-rs/latest/granite/",
                    gtk::gdk::CURRENT_TIME,
                );
            });

            let filter_button = welcome
                .append_button(
                    &gio::ThemedIcon::new("filter"),
                    &gettext("Custom Icon"),
                    &gettext("Icon from GResource")
                )
                .expect("Unable to construct button in welcome view");

            let toast = granite::Toast::new(&gettext("Filter clicked"));

            filter_button.connect_clicked(clone!(@weak toast => move |_| {
                toast.send_notification();
            }));

            let overlay = gtk::Overlay::builder()
                .child(&welcome)
                .build();
            overlay.add_overlay(&toast);

            obj.append(&overlay);
        }
    }
    impl WidgetImpl for WelcomeView {}
    impl BoxImpl for WelcomeView {}
}

glib::wrapper! {
    pub struct WelcomeView(ObjectSubclass<imp::WelcomeView>)
        @extends gtk::Widget, gtk::Box;
}

impl WelcomeView {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("orientation", gtk::Orientation::Vertical)
            .property("spacing", 0)
            .build()
    }
}

impl Default for WelcomeView {
    fn default() -> Self {
        Self::new()
    }
}
