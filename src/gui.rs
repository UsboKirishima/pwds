pub mod gui {
    use gtk::prelude::*;
    use gtk::{Application, ApplicationWindow, Button, GestureClick, Image};
    use std::cell::RefCell;
    use std::rc::Rc;

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
        let window = ApplicationWindow::builder()
            .application(app)
            .title("pwds")
            .default_width(700)
            .default_height(350)
            .build();

        /* Get password */
        let get_pwd_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let get_pwd_entry = gtk::Entry::builder()
            .css_name("entry")
            .placeholder_text("Encryption Key")
            .build();
        let get_pwd_button = gtk::Button::with_label("Get Password");
        get_pwd_button.add_css_class("mgr_button");

        get_pwd_box.append(&get_pwd_entry);
        get_pwd_box.append(&get_pwd_button);

        get_pwd_box.set_valign(gtk::Align::Center);
        get_pwd_box.set_halign(gtk::Align::Center);

        window.set_child(Some(&get_pwd_box));

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

        /*
         * Volatile variable, default: "manager"
         * Store the current view in a Rc<RefCell> for mutability
         */
        let current_view = Rc::new(RefCell::new(None::<gtk::Box>));

        // Create the main content area
        let content_area = gtk::Box::new(gtk::Orientation::Vertical, 10);
        content_area.add_css_class("page");
        content_area.set_hexpand(true);
        content_area.set_vexpand(true);
        main_box.append(&content_area);

        let default_page = manager_page();
        content_area.append(&default_page);
        content_area.set_margin_start(12);
        content_area.set_margin_bottom(12);

        for option in menu_options {
            let button = Button::builder()
                .label(option)
                .name(option.to_lowercase().replace(" ", "_"))
                .build();

            let content_area_clone = content_area.clone();
            let current_view_clone = Rc::clone(&current_view);

            button.connect_clicked(move |btn| {
                let content_area = &content_area_clone;
                let mut current_view = current_view_clone.borrow_mut();

                /* Clean up previous view */
                while let Some(child) = content_area.last_child() {
                    content_area.remove(&child);
                }

                match btn.label().unwrap().as_str() {
                    "manager" => {
                        let mgr_page = manager_page();
                        content_area.append(&mgr_page);
                        *current_view = Some(mgr_page);
                    }
                    "pwds" => {
                        let pwds_page = pwds_page();
                        content_area.append(&pwds_page);
                        *current_view = Some(pwds_page);
                    }
                    "credits" => {
                        let crds_page = credits_page();
                        content_area.append(&crds_page);
                        *current_view = Some(crds_page);
                    }
                    _ => {
                        let mgr_page = manager_page();
                        content_area.append(&mgr_page);
                        *current_view = Some(mgr_page);
                    }
                }
            });

            sidebar.append(&button);
        }

        window.add_css_class("window");
        window.present();

        get_pwd_button.connect_clicked(move |_| {
            window.set_child(Some(&main_box));
        });
    }

    fn manager_page() -> gtk::Box {
        let manager_box = gtk::Box::new(gtk::Orientation::Vertical, 7);
        manager_box.set_hexpand(true);
        manager_box.set_vexpand(true);
        manager_box.set_valign(gtk::Align::End);

        let page_title = gtk::Label::new(Some("Manager"));
        page_title.set_css_classes(&["title"]);
        page_title.set_halign(gtk::Align::Center);
        page_title.set_valign(gtk::Align::Start);
        page_title.set_margin_bottom(24);

        let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            spacer.set_vexpand(true);
            manager_box.append(&spacer);

        manager_box.set_margin_top(12);
        manager_box.set_margin_end(12);
        manager_box.set_margin_bottom(12);
        manager_box.set_margin_start(12);

        let add_button = Button::with_label("Add Password");
        let modify_button = Button::with_label("Edit Password");
        let remove_button = Button::with_label("Remove Password");

        /* Css classes */
        add_button.add_css_class("mgr_button");
        modify_button.add_css_class("mgr_button");
        remove_button.add_css_class("mgr_button");

        let entries_box = gtk::Box::new(gtk::Orientation::Vertical, 7);

        let username_entry = gtk::Entry::builder()
            .css_name("entry")
            .placeholder_text("Username")
            .build();

        let password_entry = gtk::Entry::builder()
            .css_name("entry")
            .placeholder_text("Password")
            .build();

        entries_box.append(&username_entry);
        entries_box.append(&password_entry);
        entries_box.set_hexpand(true);
        entries_box.set_vexpand(true);
        entries_box.set_margin_bottom(12);
        entries_box.set_valign(gtk::Align::End);

        manager_box.append(&page_title);
        manager_box.append(&entries_box);
        manager_box.append(&add_button);
        manager_box.append(&modify_button);
        manager_box.append(&remove_button);

        add_button.connect_clicked(|_| println!("Add Password"));
        modify_button.connect_clicked(|_| println!("Edit Password"));
        remove_button.connect_clicked(|_| println!("Remove Password"));

        manager_box
    }

    fn pwds_page() -> gtk::Box {
        let pwds_box = gtk::Box::new(gtk::Orientation::Vertical, 7);
        pwds_box.set_hexpand(true);
        pwds_box.set_vexpand(true);
        pwds_box.set_valign(gtk::Align::Start);

        let page_title = gtk::Label::new(Some("Pwds"));
        page_title.set_css_classes(&["title"]);
        page_title.set_halign(gtk::Align::Center);
        page_title.set_valign(gtk::Align::Start);
        page_title.set_margin_bottom(24);

        let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            spacer.set_vexpand(true);
            pwds_box.append(&spacer);

        pwds_box.set_margin_top(12);
        pwds_box.set_margin_end(12);
        pwds_box.set_margin_bottom(12);
        pwds_box.set_margin_start(12);

        pwds_box.append(&page_title);

        struct Credentials {
            username: String,
            password: String,
        }

        let credentials = Rc::new(RefCell::new(vec![
            Credentials {
                username: String::from("user1"),
                password: String::from("password1"),
            },
            Credentials {
                username: String::from("user2"),
                password: String::from("password2"),
            },
            Credentials {
                username: String::from("user3"),
                password: String::from("password3"),
            },
        ]));

        for cred in credentials.borrow().iter() {
            let pwds_box_clone = pwds_box.clone();

            let cred_box = gtk::Box::new(gtk::Orientation::Horizontal, 14);
            //cred_box.set_hexpand(true);
            //cred_box.set_vexpand(true);
            //cred_box.set_valign(gtk::Align::Start);
            //cred_box.set_halign(gtk::Align::Center);
            cred_box.add_css_class("cred_card");
            cred_box.set_margin_top(7);

            let cred_username = gtk::Label::new(Some(&cred.username));
            cred_username.add_css_class("cred_username");
            cred_username.set_margin_bottom(12);
            cred_username.set_margin_top(12);
            cred_username.set_margin_start(12);
            cred_username.set_halign(gtk::Align::Start);
            cred_username.set_valign(gtk::Align::Center);

            cred_box.append(&cred_username);

            let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            spacer.set_hexpand(true);
            cred_box.append(&spacer);

            let cred_pwd = gtk::Label::new(Some("unhide"));
            cred_pwd.add_css_class("cred_pwd");
            cred_pwd.set_margin_bottom(12);
            cred_pwd.set_margin_top(12);
            cred_pwd.set_margin_end(2);
            cred_pwd.set_halign(gtk::Align::End);
            cred_pwd.set_valign(gtk::Align::Center);

            let gesture = GestureClick::new();
            let cred_pwd_clone = cred_pwd.clone();
            let cred_password_clone = cred.password.clone();

            gesture.connect_pressed(move |_, _, _, _| match cred_pwd_clone.text().as_str() {
                "unhide" => {
                    cred_pwd_clone.set_text(&cred_password_clone);
                }
                _ => {
                    cred_pwd_clone.set_text("unhide");
                }
            });

            cred_box.append(&cred_pwd);
            cred_pwd.add_controller(gesture);

            let cred_copy_pwd = gtk::Button::builder()
                .label("copy")
                .css_name("copy_pwd")
                .build();
            cred_copy_pwd.set_size_request(40, 12);
            cred_copy_pwd.set_margin_bottom(7);
            cred_copy_pwd.set_margin_top(7);
            cred_copy_pwd.set_margin_end(12);

            cred_box.append(&cred_copy_pwd);

            pwds_box_clone.append(&cred_box);
        }

        pwds_box
    }

    fn credits_page() -> gtk::Box {
        let credits_box = gtk::Box::new(gtk::Orientation::Vertical, 7);
        credits_box.set_hexpand(true);
        credits_box.set_vexpand(true);
        credits_box.set_valign(gtk::Align::Start);

        let page_title = gtk::Label::new(Some("Credits"));
        page_title.set_css_classes(&["title"]);
        page_title.set_halign(gtk::Align::Center);
        page_title.set_valign(gtk::Align::Start);
        page_title.set_margin_bottom(24);

        let spacer = gtk::Box::new(gtk::Orientation::Vertical, 0);
            spacer.set_vexpand(true);
            credits_box.append(&spacer);

        credits_box.set_margin_top(12);
        credits_box.set_margin_end(12);
        credits_box.set_margin_bottom(12);
        credits_box.set_margin_start(12);

        credits_box.append(&page_title);

        let author_label = gtk::Label::new(Some(
            "Made by UsboKirishima",
        ));
        author_label.add_css_class("content");
        author_label.set_halign(gtk::Align::Start);
        author_label.set_valign(gtk::Align::End);
        author_label.set_margin_bottom(10);

        let copyright_label = gtk::Label::new(Some("© 2025 Davide Usberti. All Rights Reserved."));
        copyright_label.add_css_class("content");
        copyright_label.set_halign(gtk::Align::Start);
        copyright_label.set_valign(gtk::Align::End);
        copyright_label.set_margin_bottom(2);

        let github_label = gtk::Label::new(Some("GitHub: https://github.com/UsboKirishima"));
        github_label.add_css_class("content");
        github_label.set_halign(gtk::Align::Start);
        github_label.set_valign(gtk::Align::End);
        github_label.set_margin_bottom(2);
        github_label.set_selectable(true);

        let repo_label = gtk::Label::new(Some("Repository: https://github.com/UsboKirishima/pwds"));
        repo_label.add_css_class("content");
        repo_label.set_halign(gtk::Align::Start);
        repo_label.set_valign(gtk::Align::End);
        repo_label.set_margin_bottom(2);
        repo_label.set_selectable(true);

        let desc_label = gtk::Label::new(Some(
        "Description: My custom-made Password Manager, written in just one day after learning Rust."));
        desc_label.add_css_class("content");
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_valign(gtk::Align::End);
        desc_label.set_wrap(true);

        credits_box.append(&author_label);
        credits_box.append(&copyright_label);
        credits_box.append(&github_label);
        credits_box.append(&repo_label);
        credits_box.append(&desc_label);

        credits_box
    }
}
