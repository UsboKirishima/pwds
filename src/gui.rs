pub mod gui {
    use gtk::prelude::*;
    use gtk::{Application, ApplicationWindow, Button, Image};

    pub fn load_css() {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("../theme.css"));

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    pub fn build_ui(app: &Application) {
        let button = Button::builder()
            .label("Press me!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        button.connect_clicked(|button| {
            button.set_label("Hello World!");
        });

        // Main box
        let main_box = gtk::Box::builder()
            .margin_start(10)
            .margin_end(20)
            .margin_top(20)
            .margin_bottom(10)
            .orientation(gtk::Orientation::Horizontal)
            .build();

        // Menu box
        let sidebar = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .css_name("sidebar")
            .build();
        sidebar.set_size_request(200, -1);
        main_box.append(&sidebar);

        let logo_image: Image = Image::from_file("logo.png");
        logo_image.set_pixel_size(128);
        logo_image.set_margin_bottom(20);
        sidebar.append(&logo_image);

        let menu_options: [&str; 3] = ["manager", "pwds", "credits"];

        for option in menu_options {
            let button = Button::builder()
                .label(option)
                .name(option.to_lowercase().replace(" ", "_"))
                .build();
            sidebar.append(&button);
        }

        let window = ApplicationWindow::builder()
            .application(app)
            .title("pwds")
            .child(&main_box)
            .default_width(800)
            .default_height(400)
            .build();
        window.add_css_class("window");
        window.present();
    }
}
