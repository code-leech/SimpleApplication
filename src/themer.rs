/* themer.rs
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

use gtk::{glib, gio};
use adw::subclass::prelude::*;
use adw::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/self/SimpleApplication/themer.ui")]
    pub struct SimpleapplicationThemer {
        #[template_child]
        pub light: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub dark: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub follow: TemplateChild<gtk::CheckButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleapplicationThemer {
        const NAME: &'static str = "SimpleapplicationThemer";
        type Type = super::SimpleapplicationThemer;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SimpleapplicationThemer {
        fn constructed(&self) {
            self.parent_constructed();
            let settings = gio::Settings::new("org.self.SimpleApplication");
            glib::spawn_future_local(glib::clone!(#[weak(rename_to = obj)] self, async move {
                let saved_style =  settings.int("theme");
                match saved_style {
                    3 => obj.dark.set_active(true),
                    2 => obj.light.set_active(true),
                    1 => obj.follow.set_active(true),
                    _ => (),
                }
            }));
        }
    }
    impl WidgetImpl for SimpleapplicationThemer {}
    impl BoxImpl for SimpleapplicationThemer {}

    #[gtk::template_callbacks]
    impl SimpleapplicationThemer {
        #[template_callback]
        fn follow_toggled(follow: gtk::CheckButton) {
            if follow.is_active() {
                glib::spawn_future_local(async move {
                    let style = adw::StyleManager::default();
                    let settings = gio::Settings::new("org.self.SimpleApplication");
                    style.set_color_scheme(adw::ColorScheme::Default);
                    settings.set_int("theme", 1).expect("Failed to save theme");
                });
            }
        }

        #[template_callback]
        fn light_toggled(light: gtk::CheckButton) {
            if light.is_active() {
                glib::spawn_future_local(async move {
                    let style = adw::StyleManager::default();
                    let settings = gio::Settings::new("org.self.SimpleApplication");
                    style.set_color_scheme(adw::ColorScheme::ForceLight);
                    settings.set_int("theme", 2).expect("Failed to save theme");
                });
            }
        }

        #[template_callback]
        fn dark_toggled(dark: gtk::CheckButton) {
            if dark.is_active() {
                glib::spawn_future_local(async move {
                    let style = adw::StyleManager::default();
                    let settings = gio::Settings::new("org.self.SimpleApplication");
                    style.set_color_scheme(adw::ColorScheme::ForceDark);
                    settings.set_int("theme", 3).expect("Failed to save theme");
                });
            }
        }
    }
}

glib::wrapper! {
    pub struct SimpleapplicationThemer(ObjectSubclass<imp::SimpleapplicationThemer>)
        @extends gtk::Widget, gtk::Box;
}

impl SimpleapplicationThemer {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

