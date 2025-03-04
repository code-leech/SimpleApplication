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

use gtk::glib;
use adw::subclass::prelude::*;
use adw::prelude::CheckButtonExt;

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

    impl ObjectImpl for SimpleapplicationThemer {}
    impl WidgetImpl for SimpleapplicationThemer {}
    impl BoxImpl for SimpleapplicationThemer {}

    #[gtk::template_callbacks]
    impl SimpleapplicationThemer {
        #[template_callback]
        fn follow_toggled(follow: gtk::CheckButton) {
            if follow.is_active() {
                let style = adw::StyleManager::default();
                style.set_color_scheme(adw::ColorScheme::Default);
            }
        }

        #[template_callback]
        fn light_toggled(light: gtk::CheckButton) {
            if light.is_active() {
                let style = adw::StyleManager::default();
                style.set_color_scheme(adw::ColorScheme::ForceLight);
            }
        }

        #[template_callback]
        fn dark_toggled(dark: gtk::CheckButton) {
            if dark.is_active() {
                let style = adw::StyleManager::default();
                style.set_color_scheme(adw::ColorScheme::ForceDark);
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

