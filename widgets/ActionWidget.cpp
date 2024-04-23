#include "ActionWidget.h"

ActionWidget::ActionWidget() {

}

GdkPixbuf *resize_pixbuf(GdkPixbuf *src_pixbuf, int new_width, int new_height) {
    GdkPixbuf *resized_pixbuf = gdk_pixbuf_scale_simple(src_pixbuf, new_width, new_height, GDK_INTERP_BILINEAR);
    return resized_pixbuf;
}

void ActionWidget::init(const char *name, guchar *data, int size_width, int size_height) {
    GdkPixbuf *image_pixbuf = gdk_pixbuf_new_from_data(data, GDK_COLORSPACE_RGB, TRUE, 8, 52, 69, 3 * 69, NULL, NULL);

    if (image_pixbuf == nullptr) {
        cout << "Failed to load image" << endl;
        return;
    }

    GdkPixbuf *resized_pixbuf = resize_pixbuf(image_pixbuf, size_width, size_height);

    g_object_unref(image_pixbuf);

    image = gtk_image_new_from_pixbuf(resized_pixbuf);

//    image = gtk_image_new_from
    button = gtk_button_new();
    gtk_widget_set_valign(button, GTK_ALIGN_CENTER); // Выравнивание содержимого по вертикали по центру
    gtk_widget_set_halign(button, GTK_ALIGN_START); // Выравнивание содержимого по горизонтали в начало
    gtk_container_add(GTK_CONTAINER(button), image);

    gtk_widget_set_name(button, name);
}

void ActionWidget::addToBox(GtkWidget *box) {
    if (image != nullptr) { // Проверка на nullptr, чтобы избежать ошибки во время выполнения
        gtk_box_pack_start(GTK_BOX(box), button, FALSE, FALSE, 0);
    } else {
        cout << "Error: Image widget is not initialized." << endl;
    }
}

void ActionWidget::onClicked(GCallback callback_func, gpointer *command) {
    g_signal_connect(button, "clicked", G_CALLBACK(callback_func), (gpointer)command);
}