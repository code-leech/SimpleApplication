#pragma once

#include <adwaita.h>

G_BEGIN_DECLS

#define CUSTOM_TYPE_PREFERENCES_DIALOG (custom_preferences_dialog_get_type())

G_DECLARE_FINAL_TYPE (CustomPreferencesDialog, custom_preferences_dialog, CUSTOM, PREFERENCES_DIALOG, AdwPreferencesDialog)

CustomPreferencesDialog *custom_preferences_dialog_new (void);

void simpleapplication_switch_theme (GObject    *object,
                                     GParamSpec *pspec,
                                     gpointer    user_data);

G_END_DECLS

