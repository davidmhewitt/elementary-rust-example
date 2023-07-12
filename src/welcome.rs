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
use granite::traits::ToastExt;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
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
                    &gettext("Icon from GResource"),
                )
                .expect("Unable to construct button in welcome view");

            filter_button.set_widget_name("filter-button");

            let toast = granite::Toast::new(&gettext("Filter clicked"));
            toast.set_widget_name("toast");

            filter_button.connect_clicked(clone!(@weak toast => move |_| {
                toast.send_notification();
            }));

            let overlay = gtk::Overlay::builder().child(&welcome).build();
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

#[cfg(test)]
mod test {
    use super::*;

    fn find_child_widget_by_name(parent: &impl WidgetExt, name: &str) -> Option<gtk::Widget> {
        let mut child = parent.first_child();
        while let Some(ref inner) = child {
            if inner.widget_name() == name {
                child = Some(inner.clone());
                break;
            }

            if let Some(ref recursive) = find_child_widget_by_name(inner, name) {
                child = Some(recursive.clone());
                break;
            }

            child = inner.next_sibling();
        }

        child
    }

    fn find_child_widget_by_type<T: IsA<gtk::Widget>>(parent: &impl WidgetExt) -> Option<T> {
        let mut child = parent.first_child();
        let mut result: Option<T> = None;
        while let Some(ref inner) = child {
            if let Ok(inner) = inner.clone().dynamic_cast::<T>() {
                result = Some(inner);
                break;
            }

            if let Some(ref recursive) = find_child_widget_by_type::<T>(inner) {
                result = Some(recursive.clone());
                break;
            }

            child = inner.next_sibling();
        }

        result
    }

    fn wait(ms: u32) {
        let main_loop = glib::MainLoop::new(None, false);
        glib::timeout_add(
            std::time::Duration::from_millis(ms as u64),
            glib::clone!(@strong main_loop => move || {
                main_loop.quit();
                Continue(false)
            }),
        );

        main_loop.run();
    }

    #[gtk::test]
    fn test_welcome_view() {
        let window = gtk::Window::new();

        let welcome_view = WelcomeView::new();

        window.set_child(Some(&welcome_view));
        window.present();

        let filter = find_child_widget_by_name(&welcome_view, "filter-button")
            .expect("Filter button cannot be found");
        let filter = filter
            .dynamic_cast::<gtk::Button>()
            .expect("Filter button cannot be cast");

        let toast =
            find_child_widget_by_name(&welcome_view, "toast").expect("Toast cannot be found");
        let toast = toast
            .dynamic_cast::<granite::Toast>()
            .expect("Toast cannot be cast");

        let toast_revealer =
            find_child_widget_by_type::<gtk::Revealer>(&toast).expect("Can't get toast revealer");

        assert!(!toast_revealer.is_child_revealed());

        filter.activate();
        wait(1100);

        assert!(toast_revealer.is_child_revealed());

        wait(4000);

        assert!(!toast_revealer.is_child_revealed());
    }
}
