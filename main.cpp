#include <iostream>
#include <gtk/gtk.h>
#include <filesystem>

#include "modules/CssManager.h"

#include "widgets/ActionWidget.h"

using namespace std;

#define ACTION_WIDTH 40
#define ACTION_HEIGHT 40

ActionWidget poweroff, suspend, reboot;

void gui(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "powerdialog");
    gtk_window_set_default_size(GTK_WINDOW(window), 400, 100);
    gtk_window_set_type_hint(GTK_WINDOW(window), GDK_WINDOW_TYPE_HINT_DIALOG);

    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

    GtkWidget *container = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);

    // Action widgets
    poweroff.init("poweroff", "images/poweroff.svg", ACTION_WIDTH, ACTION_HEIGHT);
    suspend.init("suspend", "images/suspend.svg", ACTION_WIDTH, ACTION_HEIGHT);
    reboot.init("reboot", "images/reboot.svg", ACTION_WIDTH, ACTION_HEIGHT);

    gtk_container_add(GTK_CONTAINER(window), container);
    poweroff.addToBox(container);
    suspend.addToBox(container);
    reboot.addToBox(container);

    //CSS
    CssManager css;
    css.loadFromFile("style.css");

    gtk_widget_show_all(window);

    gtk_main();
}

int main(int argc, char *argv[]) {
    gui(argc, argv);
    return 0;
}