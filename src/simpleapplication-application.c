/* simpleapplication-application.c
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
#include <glib/gi18n.h>

#include "simpleapplication-application.h"
#include "simpleapplication-window.h"
#include "simpleapplication-preferences.h"
#include <unistd.h>


struct _SimpleapplicationApplication
{
  AdwApplication parent_instance;
  CustomPreferencesDialog *mydialog;
};

G_DEFINE_FINAL_TYPE (SimpleapplicationApplication, simpleapplication_application, ADW_TYPE_APPLICATION)

SimpleapplicationApplication *
simpleapplication_application_new (const char        *application_id,
                                   GApplicationFlags  flags)
{
  g_return_val_if_fail (application_id != NULL, NULL);

  return g_object_new (SIMPLEAPPLICATION_TYPE_APPLICATION,
	                     "application-id", application_id,
	                     "flags", flags,
	                     NULL);
}

static void
simpleapplication_application_activate (GApplication *app)
{
  GtkWindow *window;

  g_assert (SIMPLEAPPLICATION_IS_APPLICATION (app));

  window = gtk_application_get_active_window (GTK_APPLICATION (app));

  if (window == NULL)
    window = g_object_new (SIMPLEAPPLICATION_TYPE_WINDOW,
		           "application", app,
		           NULL);

  gtk_window_present (window);
}

static void
simpleapplication_application_class_init (SimpleapplicationApplicationClass *klass)
{
  GApplicationClass *app_class = G_APPLICATION_CLASS (klass);

  app_class->activate = simpleapplication_application_activate;
}

static void
simpleapplication_application_about_action (GSimpleAction *action,
                                            GVariant      *parameter,
                                            gpointer       user_data)
{
  static const char *developers[] = {"Carbon751 <upperint2011@gmail.com>", "The GTK Devs https://gtk.org/", "The Libadwaita Devs https://gitlab.gnome.org/GNOME/libadwaita/", NULL};
  SimpleapplicationApplication *self = user_data;
  GtkWindow *window = NULL;

  g_assert (SIMPLEAPPLICATION_IS_APPLICATION (self));

  window = gtk_application_get_active_window (GTK_APPLICATION (self));

  adw_show_about_dialog (GTK_WIDGET (window),
	                 "application-name", "Simple Application",
	                 "application-icon", "org.self.SimpleApplication",
	                 "developer-name", "Carbon751 (Matteo Pinti)",
	                 "translator-credits", _("translator-credits"),
	                 "version", "Version 69420",
	                 "developers", developers,
                         "copyright", "",
                          NULL);
}

static void
simpleapplication_preferences_end (GtkWidget *widget, gpointer  user_data)
{
  SimpleapplicationApplication *self = user_data;
  self->mydialog = NULL;
}

static void
simpleapplication_preferences_start (GSimpleAction *action,
                                     GVariant      *variant,
                                     gpointer      user_data)
{
  SimpleapplicationApplication *self = user_data;
  GtkWindow *window = gtk_application_get_active_window (GTK_APPLICATION (self));

  //Create dialog if none is being currently shown.
  if (!self->mydialog)
    {
      self->mydialog =  CUSTOM_PREFERENCES_DIALOG (custom_preferences_dialog_new());
      adw_dialog_present (ADW_DIALOG (self->mydialog), GTK_WIDGET (window));
      g_signal_connect (self->mydialog, "closed", G_CALLBACK (simpleapplication_preferences_end), self);
    }
}


static void
simpleapplication_application_quit_action (GSimpleAction *action,
                                           GVariant      *parameter,
                                           gpointer       user_data)
{
  SimpleapplicationApplication *self = user_data;

  g_assert (SIMPLEAPPLICATION_IS_APPLICATION (self));

  g_application_quit (G_APPLICATION (self));
}

static const GActionEntry app_actions[] = {
  { "quit", simpleapplication_application_quit_action },
  { "about", simpleapplication_application_about_action },
  { "preferences", simpleapplication_preferences_start },
};

static gpointer
simpleapplication_background_setup (gpointer user_data)
{
  AdwStyleManager *style = adw_style_manager_get_default ();
  adw_style_manager_set_color_scheme (style, ADW_COLOR_SCHEME_FORCE_DARK);
  return NULL;
}

static void
simpleapplication_application_init (SimpleapplicationApplication *self)
{
  GThread *background_setup = g_thread_new ("background_setup", simpleapplication_background_setup, NULL);
  g_thread_unref (background_setup);

  g_action_map_add_action_entries (G_ACTION_MAP (self),
	                           app_actions,
	                           G_N_ELEMENTS (app_actions),
	                           self);
  gtk_application_set_accels_for_action (GTK_APPLICATION (self),
	                                 "app.quit",
	                                 (const char *[]) { "<primary>q", NULL });
  gtk_application_set_accels_for_action (GTK_APPLICATION (self),
                                         "app.preferences",
                                         (const char *[]) { "<primary>comma", NULL });


}
