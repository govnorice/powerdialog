#ifndef TEST_CSSMANAGER_H
#define TEST_CSSMANAGER_H

#include <gtk/gtk.h>

class CssManager {
public:
    CssManager();
    ~CssManager();
    void loadFromFile(const gchar *filename);
    void loadFromData(const gchar *data);
private:
    GtkCssProvider *provider;
    GtkStyleContext *context;
};

#endif //TEST_CSSMANAGER_H