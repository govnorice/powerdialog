#ifndef POWERDIALOG_ACTIONWIDGET_H
#define POWERDIALOG_ACTIONWIDGET_H

#include <iostream>
#include <gtk/gtk.h>

using namespace std;

// Widget (poweroff, suspend, reboot)
class ActionWidget {
public:
    ActionWidget();
    void init(const char *name, const char *image_path, int size_width, int size_height);
    void addToBox(GtkWidget *box);
private:
    GdkPixbuf *image_pixbuf;
    GtkWidget *image;
    GtkWidget *button;
};


#endif //POWERDIALOG_ACTIONWIDGET_H