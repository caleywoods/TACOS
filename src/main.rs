extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use glib::clone;
use gtk::{
    AboutDialog, AccelFlags, AccelGroup, ApplicationWindow, Image, Label,
    Menu, MenuBar, MenuItem, WindowPosition,
};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("TACOS");
    window.set_position(WindowPosition::Center);
    window.set_size_request(600, 600);

    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::WordChar);
    text_view.set_cursor_visible(true);
    text_view.set_left_margin(5);
    text_view.set_right_margin(5);

    gtk::WidgetExt::set_widget_name(&text_view, "text-view");

    let scrolled_text_view = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    gtk::WidgetExt::set_widget_name(&scrolled_text_view, "text-buffer");

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    gtk::WidgetExt::set_widget_name(&v_box, "v-box");

    let menu = Menu::new();
    let accel_group = AccelGroup::new();
    window.add_accel_group(&accel_group);

    let menu_bar = MenuBar::new();

    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");
    let file_item = MenuItem::new();
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let file_image = Image::new_from_file("resources/file.png");
    let file_label = Label::new(Some("File"));

    file_box.pack_start(&file_image, false, false, 0);
    file_box.pack_start(&file_label, true, true, 0);
    file_item.add(&file_box);
    menu.append(&file_item);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    quit.connect_activate(clone!(@weak window => move |_| {
        window.destroy();
    }));

    // `Primary` is `Ctrl` on Windows and Linux, and `command` on macOS
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, key, modifier, AccelFlags::VISIBLE);

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&scrolled_text_view, true, true, 0);

    window.add(&v_box);
    window.show_all();

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Caley Woods"]);
        p.set_website_label(Some(""));
        p.set_website(Some(""));
        p.set_authors(&["Caley Woods"]);
        p.set_title("About");
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("dev.dfnkt.tacos"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        let css_provider = gtk::CssProvider::new();
        css_provider
            .load_from_path("../../src/css/gtk.css")
            .expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing css provider."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}