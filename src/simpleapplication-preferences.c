
#include "simpleapplication-preferences.h"

struct _CustomPreferencesDialog
{
  AdwPreferencesDialog parent_instance;
  AdwSwitchRow *switchrow;
};

G_DEFINE_FINAL_TYPE (CustomPreferencesDialog, custom_preferences_dialog, ADW_TYPE_PREFERENCES_DIALOG)


G_MODULE_EXPORT void
simpleapplication_switch_theme (GObject    *object,
                                GParamSpec *pspec,
                                gpointer   user_data)
{
  AdwStyleManager *style = ADW_STYLE_MANAGER (adw_style_manager_get_default ());

  if (adw_switch_row_get_active (ADW_SWITCH_ROW (object)))
    adw_style_manager_set_color_scheme (style, ADW_COLOR_SCHEME_FORCE_DARK);
  else
    adw_style_manager_set_color_scheme (style, ADW_COLOR_SCHEME_FORCE_LIGHT);
}

static void
custom_preferences_dialog_class_init (CustomPreferencesDialogClass *klass)
{
  GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

  gtk_widget_class_set_template_from_resource (widget_class, "/org/self/SimpleApplication/simpleapplication-preferences.ui");
  gtk_widget_class_bind_template_child (widget_class, CustomPreferencesDialog, switchrow);
}

static void
custom_preferences_dialog_init (CustomPreferencesDialog *self)
{
  AdwStyleManager *style = ADW_STYLE_MANAGER (adw_style_manager_get_default ());
  gtk_widget_init_template (GTK_WIDGET (self));

  if (adw_style_manager_get_color_scheme (style) == ADW_COLOR_SCHEME_FORCE_DARK)
    adw_switch_row_set_active (self->switchrow, true);
  else
    adw_switch_row_set_active (self->switchrow, false);
}

CustomPreferencesDialog *
custom_preferences_dialog_new (void)
{
  return g_object_new (CUSTOM_TYPE_PREFERENCES_DIALOG, NULL);
}


