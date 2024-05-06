#include <iostream>
#include <gtk/gtk.h>
#include <unistd.h>
#include <cstdlib>

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

ActionWidget poweroff, suspend, reboot, logout;

void power_clicked(GtkWidget *widget, gpointer data) {
    auto *pw = (ActionWidget::PowerData *)data;
    const string path = pw->path;
    const string arg1 = pw->arg1;
    const string arg2 = pw->arg2;
    const string session_id = getenv("XDG_SESSION_ID");

    if (arg2 == "suspend") {
        execl(path.c_str(), arg1.c_str(), arg2.c_str(), (char *)nullptr);
        return;
    }
    if (arg2 == "kill-session") {
        execl(path.c_str(), arg1.c_str(), "kill-session", session_id.c_str(), (char *)nullptr);
        return;
    }

    execl(path.c_str(), arg1.c_str(), (char *)nullptr);
}

void gui(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "powerdialog");
    gtk_window_set_default_size(GTK_WINDOW(window), 400, 80);
    gtk_window_set_resizable (GTK_WINDOW(window), FALSE);
    gtk_window_set_type_hint(GTK_WINDOW(window), GDK_WINDOW_TYPE_HINT_DIALOG);
    gtk_window_set_focus(GTK_WINDOW(window), NULL);

    g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

    GtkWidget *container = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 0);

    // Action widgets
    poweroff.init("poweroff", *imageData_poweroff, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);
    suspend.init("suspend", *imageData_suspend, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);
    reboot.init("reboot", *imageData_reboot, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);
    logout.init("logout", *imageData_logout, IMAGE_WIDTH, IMAGE_HEIGHT, BYTES_PER_PIXEL, ACTION_WIDTH, ACTION_HEIGHT);

    // Clicked widget
    ActionWidget::PowerData pw1;
    pw1.path = "/usr/bin/poweroff";
    pw1.arg1 = "poweroff";

    ActionWidget::PowerData pw2;
    pw2.path = "/usr/bin/systemctl";
    pw2.arg1 = "systemctl";
    pw2.arg2 = "suspend";

    ActionWidget::PowerData pw3;
    pw3.path = "/usr/bin/reboot";
    pw3.arg1 = "reboot";

    ActionWidget::PowerData pw4;
    pw4.path = "/usr/bin/loginctl";
    pw4.arg1 = "loginctl";
    pw4.arg2 = "kill-session";

    poweroff.onClicked(G_CALLBACK(power_clicked), &pw1);
    suspend.onClicked(G_CALLBACK(power_clicked), &pw2);
    reboot.onClicked(G_CALLBACK(power_clicked), &pw3);
    logout.onClicked(G_CALLBACK(power_clicked), &pw4);

    gtk_container_add(GTK_CONTAINER(window), container);
    poweroff.addToBox(container);
    suspend.addToBox(container);
    reboot.addToBox(container);
    logout.addToBox(container);

    //CSS
    CssManager css;

    string path = string(homeDir) + "/.config/powerdialog/style.css";

    css.loadFromHex(style_css, style_css_len);
    css.loadFromFile(path.c_str());

    gtk_widget_show_all(window);

    gtk_main();
}

int main(int argc, char *argv[]) {
    // Start gui
    gui(argc, argv);
    return 0;
}