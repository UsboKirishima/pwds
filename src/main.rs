use std::fs::File;
use std::io;
use std::io::Write;
use gtk::prelude::*;
use gtk::Application;

mod pwds;
mod gui;
mod crypto;

const ASCII_ART: &str = r###"
          # #### ####
        ### \/#|### |/####                > PWDS <
       ##\/#/ \||/##/_/##/_#        Made by UsboKirishima
     ###  \/###|/ \/ # ###        -------------------------
   ##_\_#\_\## | #/###_/_####        Simple pwds manager
  ## #### # \ #| /  #### ##/##        pwds >>> password
   __#_--###`  |{,###---###-~       
             \ }{         ---------------------------------
              }}{           Database: ~/.pwds.enc
              }}{           Raccomended key: `openssl rand -hex 16`
         ejm  {{}
        , -=-~{ .-^- _
              `}
               {
"###;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() {
    println!("{ASCII_ART}");
    Write::flush(&mut io::stdout()).expect("[-] Error during flush.");

    if !pwds::is_db_file() {
        match File::create(pwds::DB_PATH) {
            Ok(mut file) => {
                println!("[+] Database file created successfully.");

                if let Err(e) = file.write_all(b"Hello, World!") {
                    println!("[-] Error during writing: {}", e);
                }
            }
            Err(err) => {
                println!("[-] Error during database file creation: {err}");
            }
        }
    }

    // Run gui

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| gui::gui::load_css());
    app.connect_activate(gui::gui::build_ui);

    app.run();


    //let encryption_key = pwds::pwds::change_enc_key();
    
    // Testing
    //let enc = pwds::pwds::encrypt(encryption_key.clone(), "hello everyone".to_string());
    //let dec = pwds::pwds::decrypt(encryption_key.clone(), enc.clone())
    //if let Ok(mut file) = File::open(pwds::pwds::DB_PATH) {
    //    file.write_all(dec.as_bytes()).expect("Error ");
    //}
}
