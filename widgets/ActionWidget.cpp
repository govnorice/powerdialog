#include "ActionWidget.h"

ActionWidget::ActionWidget() {

}

void ActionWidget::init(char name, const char *image_path, int size_width, int size_height) {
    image_pixbuf = gdk_pixbuf_new_from_file_at_size(image_path, size_width, size_height, nullptr);
    if (image_pixbuf == nullptr) {
        cout << "Failed to load image: " << image_path << endl;
        return;
    }
    image = gtk_image_new_from_pixbuf(image_pixbuf);
}

void ActionWidget::addToBox(GtkWidget *box) {
    if (image != nullptr) { // Проверка на nullptr, чтобы избежать ошибки во время выполнения
        gtk_box_pack_start(GTK_BOX(box), image, FALSE, FALSE, 0);
    } else {
        cout << "Error: Image widget is not initialized." << endl;
    }
}