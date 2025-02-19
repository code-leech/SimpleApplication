
#include "simpleapplication-preferences.h"

struct _CustomPreferencesDialog
{
        AdwPreferencesDialog parent_instance;
};

G_DEFINE_FINAL_TYPE (CustomPreferencesDialog, custom_preferences_dialog, ADW_TYPE_PREFERENCES_DIALOG)


G_MODULE_EXPORT void
simpleapplication_switch_theme (GObject    *object,
                                GParamSpec *pspec,
                                gpointer   user_data)
{
        AdwStyleManager *style = ADW_STYLE_MANAGER (adw_style_manager_get_default ());

        if (adw_switch_row_get_active (ADW_SWITCH_ROW (object)))
        {
                adw_style_manager_set_color_scheme (style, ADW_COLOR_SCHEME_PREFER_DARK);
        }

        else
        {
                adw_style_manager_set_color_scheme (style, ADW_COLOR_SCHEME_PREFER_LIGHT);
        }

}

static void
custom_preferences_dialog_class_init (CustomPreferencesDialogClass *klass)
{
        GtkWidgetClass *widget_class = GTK_WIDGET_CLASS (klass);

        gtk_widget_class_set_template_from_resource (widget_class, "/org/self/SimpleApplication/simpleapplication-preferences.ui");
}

static void
custom_preferences_dialog_init (CustomPreferencesDialog *self)
{
        gtk_widget_init_template (GTK_WIDGET (self));
}

CustomPreferencesDialog *
custom_preferences_dialog_new (void)
{
        return g_object_new (CUSTOM_TYPE_PREFERENCES_DIALOG, NULL);

}


