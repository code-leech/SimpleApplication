/* preferences.rs
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

use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/self/SimpleApplication/preferences.ui")]
    pub struct SimpleapplicationPreferences {
        #[template_child]
        pub switchrow: TemplateChild<adw::SwitchRow>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleapplicationPreferences {
        const NAME: &'static str = "SimpleapplicationPreferences";
        type Type = super::SimpleapplicationPreferences;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SimpleapplicationPreferences {
        fn constructed(&self) {
            self.parent_constructed();
            glib::spawn_future_local(
                glib::clone!(
                    #[weak (rename_to = obj)]
                    self,
                    async move {
                        let style = adw::StyleManager::default();
                        if style.is_dark() {
                            obj.switchrow.set_active(true);
                        }
                    }
                )
            );
        }
    }
    impl WidgetImpl for SimpleapplicationPreferences {}
    impl AdwDialogImpl for SimpleapplicationPreferences {}
    impl PreferencesDialogImpl for SimpleapplicationPreferences {}

    #[gtk::template_callbacks]
    impl SimpleapplicationPreferences {
        #[template_callback]
        fn toggle_theme(switchrow: &adw::SwitchRow) {
            let style = adw::StyleManager::default();
            if switchrow.is_active() {
                style.set_color_scheme(adw::ColorScheme::ForceDark);
            } else {
                style.set_color_scheme(adw::ColorScheme::ForceLight);
            }
        }
    }
}

glib::wrapper! {
    pub struct SimpleapplicationPreferences(ObjectSubclass<imp::SimpleapplicationPreferences>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog;
}

impl SimpleapplicationPreferences {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
