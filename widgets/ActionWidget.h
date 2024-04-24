#ifndef POWERDIALOG_ACTIONWIDGET_H
#define POWERDIALOG_ACTIONWIDGET_H

#include <iostream>
#include <gtk/gtk.h>
#include <string>

using namespace std;

// Widget (poweroff, suspend, reboot)
class ActionWidget {
public:
    typedef struct {
        string path;
        string arg1;
        string arg2;
    } PowerData;

    ActionWidget();
    void init(const char *name, guchar *data, int width, int height, int bytes_per_pixel, int size_width, int size_height);
    void addToBox(GtkWidget *box);
    void onClicked(GCallback callback_func, PowerData *pw);
private:
    GdkPixbuf *image_pixbuf;
    GtkWidget *image;
    GtkWidget *button;
};


#endif //POWERDIALOG_ACTIONWIDGET_H