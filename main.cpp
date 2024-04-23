#include <iostream>
#include <gtk/gtk.h>
#include <fstream>

// Modules
#include "modules/CssManager.h"

// Widgets
#include "widgets/ActionWidget.h"

// Images
#include "images/images.h"
#include "style.hex"

using namespace std;

#define ACTION_WIDTH 22
#define ACTION_HEIGHT 22

const char *homeDir = getenv("HOME");

ActionWidget poweroff, suspend, reboot;

void power_clicked(GtkWidget *widget, gpointer data) {
    char *command = (char *) data;
    system(command);
}

void gui(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "powerdialog");
    gtk_window_set_default_size(GTK_WINDOW(window), 300, 80);
    gtk_window_set_type_hint(GTK_WINDOW(window), GDK_WINDOW_TYPE_HINT_DIALOG);

    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

    GtkWidget *container = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);

    // Action widgets
    poweroff.init("poweroff", *imageData_poweroff, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);
    suspend.init("suspend", *imageData_suspend, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);
    reboot.init("reboot", *imageData_reboot, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);

    poweroff.onClicked(G_CALLBACK(power_clicked), (gpointer *)"poweroff");
    suspend.onClicked(G_CALLBACK(power_clicked), (gpointer *)"systemctl suspend");
    reboot.onClicked(G_CALLBACK(power_clicked), (gpointer *)"reboot");

    gtk_container_add(GTK_CONTAINER(window), container);
    poweroff.addToBox(container);
    suspend.addToBox(container);
    reboot.addToBox(container);


    //CSS
    CssManager css;

    string path = string(homeDir) + "/.config/powerdialog/style.css";

    css.loadFromHex(powerdialog_style_css, powerdialog_style_css_len);
    css.loadFromFile(path.c_str());

    gtk_widget_show_all(window);

    gtk_main();
}

int main(int argc, char *argv[]) {
    // Start gui
    gui(argc, argv);
    return 0;
}