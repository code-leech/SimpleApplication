/* window.rs
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

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/self/SimpleApplication/window.ui")]
    pub struct SimpleapplicationWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleapplicationWindow {
        const NAME: &'static str = "SimpleapplicationWindow";
        type Type = super::SimpleapplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SimpleapplicationWindow {}
    impl WidgetImpl for SimpleapplicationWindow {}
    impl WindowImpl for SimpleapplicationWindow {}
    impl ApplicationWindowImpl for SimpleapplicationWindow {}
    impl AdwApplicationWindowImpl for SimpleapplicationWindow {}

    #[gtk::template_callbacks]
    impl SimpleapplicationWindow {
        #[template_callback]
        fn handle_button_clicked(button: &gtk::Button) {
            glib::spawn_future_local(glib::clone!(#[weak] button, async move {
                // Deactivate the button until the operation is done
                button.set_sensitive(false);
                button.set_label("Clicked!");
                button.set_css_classes(&["pill", "destructive-action", "title-3"]);
                glib::timeout_future_seconds(2).await;
                // Activate the button again
                button.set_label("Click me!");
                button.set_css_classes(&["pill", "suggested-action", "title-3"]);
                button.set_sensitive(true);
            }));
        }
    }
}

glib::wrapper! {
    pub struct SimpleapplicationWindow(ObjectSubclass<imp::SimpleapplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl SimpleapplicationWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
