#include "ActionWidget.h"

ActionWidget::ActionWidget() {

}

void ActionWidget::init(const char *name, const char *image_path, int size_width, int size_height) {
    image_pixbuf = gdk_pixbuf_new_from_file_at_size(image_path, size_width, size_height, nullptr);
    if (image_pixbuf == nullptr) {
        cout << "Failed to load image: " << image_path << endl;
        return;
    }
    image = gtk_image_new_from_pixbuf(image_pixbuf);

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