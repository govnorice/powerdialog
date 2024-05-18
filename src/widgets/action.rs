use gtk::{Button, Image};
use gtk::Align::{Center, Start};
use gtk::gdk_pixbuf::InterpType::Bilinear;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::{ButtonExt, ImageExt, PixbufLoaderExt, WidgetExt};
use system_shutdown::{logout, reboot, shutdown, sleep};


pub struct ActionWidget {
    pub button: Button,
    pub image: Image,
}

impl ActionWidget {
    fn new() -> ActionWidget {
        ActionWidget { button: Button::new(), image: Image::new() }
    }

    pub fn init(image_data: &[u8], name: &str) -> Button {
        let name = name.to_string();
        let aw = ActionWidget::new();
        let pixbuf_loader = PixbufLoader::new();

        pixbuf_loader.write(&image_data).expect("Failed to write image data");
        pixbuf_loader.close().expect("Failed to close pixbuf loader");

        let pixbuf_resize = pixbuf_loader.pixbuf().expect("Failed to create pixbuf_loader.pixbuf()").scale_simple(24, 24, Bilinear).expect("Failed to create pixbuf resize");

        aw.image.set_from_pixbuf(Some(&pixbuf_resize));

        aw.button.set_image(Some(&aw.image));
        aw.button.set_widget_name(&name);
        aw.button.set_valign(Center);
        aw.button.set_halign(Start);

        aw.button.connect_clicked(move |_| {
            match name.as_str() {
                "poweroff" => {
                    match shutdown() {
                        Ok(_) => println!("Poweroff success"),
                        Err(error) => eprintln!("Failed to poweroff: {}", error),
                    }
                }
                "suspend" => {
                    match sleep() {
                        Ok(_) => println!("Suspend success"),
                        Err(error) => eprintln!("Failed to suspend: {}", error),
                    }
                }
                "reboot" => {
                    match reboot() {
                        Ok(_) => println!("Reboot success"),
                        Err(error) => eprintln!("Failed to reboot: {}", error),
                    }
                }
                "logout" => {
                    match logout() {
                        Ok(_) => println!("Logout success"),
                        Err(error) => eprintln!("Failed to logout: {}", error),
                    }
                }
                _ => println!("Critical error: unknown action")
            }
        });

        aw.button
    }
}