extern crate gtk;
use gtk::prelude::*;
use gdk::{keys::constants as key};
use std::path::Path;
use gtk::{Application, ApplicationWindow, MenuButton, Box, Button, Image, TextTagTable, Menu, MenuBar, Adjustment, MenuItem, Orientation, Paned, Separator, SpinButton, TextView, TextBuffer};

fn main() {
    // Initialize the GTK application
    let application = Application::new(Some("com.toast.groucho"), Default::default());

    // Activate the application
    application.connect_activate(|app| {
        build_ui(app);
    });

    // Run the application
    application.run();
}

fn build_ui(application: &Application) {
    // Create the main application window
    let window = ApplicationWindow::new(application);
    window.set_title("Groucho");
    window.set_default_size(800, 400);

    // Create a header bar
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title(Some("Groucho"));

    // Create the "Save" button on the left side of the header bar
    let save_button = Button::with_label("Save");
    header_bar.pack_start(&save_button);

    // Create the menu button on the right side of the header bar
    let menu_button = MenuButton::new();
    let menu = Menu::new();
    let menu_item = MenuItem::with_label("Menu Item");
    menu.append(&menu_item);
    menu_button.set_popup(Some(&menu));
    header_bar.pack_end(&menu_button);

    // Add the header bar to the application window
    window.set_titlebar(Some(&header_bar));

    // Create a vertical box to hold the main content
    let vbox = Box::new(Orientation::Vertical, 0);

    // Create a paned widget to separate the main window horizontally
    let paned = Paned::new(Orientation::Horizontal);
    vbox.pack_start(&paned, true, true, 0);
    paned.set_position(500);

    // Create a scrollable image on the left side of the paned
    let image_scroll = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    let image = Image::new();
    image_scroll.add(&image);
    paned.pack1(&image_scroll, true, false);

    // Create a vertical box to hold the text entry and spin button on the right side of the paned
    let right_box = Box::new(Orientation::Vertical, 0);
    paned.pack2(&right_box, true, false);

    // Create a text entry at the top of the right box
    let text_view = TextView::new();
    text_view.set_border_width(10);
    text_view.set_editable(true);
    text_view.set_monospace(true);
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_view.set_buffer(Some(&text_buffer));
    let text_scroll = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    text_scroll.add(&text_view);
    right_box.pack_start(&text_scroll, true, true, 0);

    // Create a separator below the text entry
    let separator = Separator::new(Orientation::Horizontal);
    right_box.pack_start(&separator, false, true, 0);

    // Create a spin button to choose the number of threads
    let spin_button = SpinButton::with_range(1.0, 8.0, 1.0);
    right_box.pack_start(&spin_button, false, true, 0);

    // Connect signals
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(|_, event| {
        if let Some(key) = event.keyval().into() {
            if event.state().contains(gdk::ModifierType::CONTROL_MASK) && key == key::q {
                gtk::main_quit();
                Inhibit(true);
            }
        }
        Inhibit(false)
    });

    // Add the main vertical box to the application window
    window.add(&vbox);

    // Show all the widgets
    window.show_all();
}

fn infer() -> String {
    return "".to_string()
}