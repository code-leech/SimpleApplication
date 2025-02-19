/* simpleapplication-window.c
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

#include "config.h"

#include "simpleapplication-window.h"

struct _SimpleapplicationWindow
{
	AdwApplicationWindow  parent_instance;

	/* Template widgets */
	GtkButton            *button;
};

G_DEFINE_FINAL_TYPE (SimpleapplicationWindow, simpleapplication_window, ADW_TYPE_APPLICATION_WINDOW)

static void
simpleapplication_window_class_init (SimpleapplicationWindowClass *klass)
{
	GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

	gtk_widget_class_set_template_from_resource (widget_class, "/org/self/SimpleApplication/simpleapplication-window.ui");
	gtk_widget_class_bind_template_child (widget_class, SimpleapplicationWindow, button);
}

static void
simpleapplication_button_finish (gpointer user_data)
{
        static const char *button_css[] = {"suggested-action", "title-3", "pill", NULL};
        GtkButton *button = user_data;
        gtk_button_set_label (button, "Click me!");
        gtk_widget_set_css_classes (GTK_WIDGET (button), button_css);
        gtk_widget_set_sensitive (GTK_WIDGET (button), true);
}

static void
simpleapplication_button_start (GtkButton *button,
                                gpointer user_data)
{
        static const char *button_css[] = {"destructive-action", "title-3", "pill", NULL};
        gtk_widget_set_sensitive (GTK_WIDGET (button), false);
        gtk_button_set_label (button, "Clicked!");
        gtk_widget_set_css_classes (GTK_WIDGET (button), button_css);
        g_timeout_add_seconds_once (2, simpleapplication_button_finish, button);
}

static void
simpleapplication_setup (gpointer user_data)
{
        SimpleapplicationWindow *self = user_data;
        AdwStyleManager *style = ADW_STYLE_MANAGER (adw_style_manager_get_default ());

        g_signal_connect (self->button, "clicked", G_CALLBACK (simpleapplication_button_start), NULL);

        adw_style_manager_set_color_scheme (style, ADW_COLOR_SCHEME_PREFER_DARK);

}

static void
simpleapplication_window_init (SimpleapplicationWindow *self)
{
	gtk_widget_init_template (GTK_WIDGET (self));
        g_idle_add_once (simpleapplication_setup, self);
}
