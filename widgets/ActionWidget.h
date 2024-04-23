#ifndef POWERDIALOG_ACTIONWIDGET_H
#define POWERDIALOG_ACTIONWIDGET_H

#include <iostream>
#include <gtk/gtk.h>

using namespace std;

// Widget (poweroff, suspend, reboot)
class ActionWidget {
public:
    ActionWidget();
    void init(const char *name, guchar *data, int width, int height, int bytes_per_pixel, int size_width, int size_height);
    void addToBox(GtkWidget *box);
    void onClicked(GCallback callback_func, gpointer *command);
private:
    GdkPixbuf *image_pixbuf;
    GtkWidget *image;
    GtkWidget *button;
};


#endif //POWERDIALOG_ACTIONWIDGET_H