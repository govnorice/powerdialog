pub mod widgets;
pub mod images;
pub mod modules;

use images::IMAGES;

use modules::css_manager::CssManager;

use gtk::{Application, ApplicationWindow, Box};
use gtk::Orientation::Horizontal;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, ContainerExt, WidgetExt};

use widgets::{action::ActionWidget};
use std::env;

fn main() {
    let app = Application::builder()
        .application_id("org.govnorice.powerdialog")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
           .application(app)
           .default_width(400)
           .default_height(80)
           .title("powerdialog")
           .build();

        let container = Box::new(Horizontal, 0);

        let poweroff_widget = ActionWidget::init(IMAGES.poweroff, "poweroff");
        let suspend_widget = ActionWidget::init(IMAGES.suspend, "suspend");
        let reboot_widget = ActionWidget::init(IMAGES.reboot, "reboot");
        let logout_widget = ActionWidget::init(IMAGES.logout, "logout");

        container.pack_start(&poweroff_widget, false, false, 0);
        container.pack_start(&suspend_widget, false, false, 0);
        container.pack_start(&reboot_widget, false, false, 0);
        container.pack_start(&logout_widget, false, false, 0);

        win.add(&container);

        // CSS
        let cm = CssManager::new();
        let style = "* {
                            color: white;
                        }
                        window {
                            background: black;
                        }
                        image {
                            margin: 0;
                            padding: 0;
                            padding: 10px;
                        }
                        button {
                            margin-right: 3em;
                            padding: 0;
                            background: image(transparent);
                            border: none;
                            border-radius: 100%;
                        }
                        button:focus {
                            outline: none;
                        }
                        #poweroff {
                            margin-left: 2.8em;
                        }

                        #poweroff:hover, #poweroff:focus {
                            background: image(red);
                        }
                        #suspend:hover, #suspend:focus {
                            background: image(blue);
                        }
                        #reboot:hover, #reboot:focus {
                            background: image(green);
                        }
                        #logout:hover, #logout:focus {
                            background: image(#720cad);
                        }";
        cm.load_from_data(style.as_bytes());
        if let Some(home_dir) = env::var_os("HOME") {
            if let Some(home_path) = home_dir.to_str() {
                let path: String = home_path.to_string() + "/.config/powerdialog/styles.css";
                cm.load_from_file(&path);
            } else {
                println!("Failed to convert HOME environment variable to string");
            }
        } else {
            println!("HOME environment variable is not set");
        }
        win.show_all();
    });

    app.run();
}