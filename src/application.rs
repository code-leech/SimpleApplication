/* application.rs
 *
 * Copyright 2025 Carbon751
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gettextrs::gettext;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, gdk};
use gdk::Display;


use crate::config::VERSION;
use crate::SimpleapplicationWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SimpleapplicationApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleapplicationApplication {
        const NAME: &'static str = "SimpleapplicationApplication";
        type Type = super::SimpleapplicationApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for SimpleapplicationApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.connect_startup(|obj| obj.get_styles());
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for SimpleapplicationApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = SimpleapplicationWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for SimpleapplicationApplication {}
    impl AdwApplicationImpl for SimpleapplicationApplication {}
}

glib::wrapper! {
    pub struct SimpleapplicationApplication(ObjectSubclass<imp::SimpleapplicationApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl SimpleapplicationApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn get_styles(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/org/self/SimpleApplication/style.css");

        // Add the provider to the default screen
        gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Simple Application")
            .application_icon("org.self.SimpleApplication")
            .developer_name("Carbon751")
            .version(VERSION)
            .developers(vec!["Carbon751 https://github.com/code-leech", "GTK Devs https://gtk.org/"])
            .translator_credits(&gettext("translator-credits"))
            .build();

        about.present(Some(&window));
    }
}
