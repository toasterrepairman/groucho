mod diffusion;

extern crate gtk;
use gtk::prelude::*;
use gdk::{keys::constants as key};
use std::path::Path;
use gtk::{Application, Grid, Switch, ComboBoxText, Popover, ApplicationWindow, MenuButton, Box, Button, Image, TextTagTable, Menu, MenuBar, Adjustment, MenuItem, Orientation, Paned, Separator, SpinButton, TextView, TextBuffer};
use crate::diffusion::{generate_image, download_weights_for_config, StableDiffusionVersion};

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

    // Create a button to trigger the popover
    let menu_button = Button::with_label("Settings");

    // Create the popover
    let popover = Popover::new(Some(&menu_button));

    // Create a grid to organize widgets with labels
    let grid = Grid::new();
    grid.set_column_spacing(10);
    grid.set_row_spacing(10);
    grid.set_border_width(5);
    popover.add(&grid);

    // Add a label and toggle switch for "f16"
    let label_f16 = gtk::Label::new(Some("Use f16:"));
    let toggle_switch = Switch::new();
    grid.attach(&label_f16, 0, 0, 1, 1);
    grid.attach_next_to(&toggle_switch, Some(&label_f16), gtk::PositionType::Right, 1, 1);
    toggle_switch.set_halign(gtk::Align::Center);

    // Add a label and combobox for version selection
    let label_version = gtk::Label::new(Some("Stable Diffusion\nVersion:"));
    let combobox = ComboBoxText::new();
    grid.attach_next_to(&label_version, Some(&label_f16), gtk::PositionType::Bottom, 1, 1);
    grid.attach_next_to(&combobox, Some(&label_version), gtk::PositionType::Right, 1, 1);

    // Add enum options to the combobox
    for version in &[
        StableDiffusionVersion::V1_5,
        StableDiffusionVersion::V2_1,
        StableDiffusionVersion::Xl,
        StableDiffusionVersion::Turbo,
        // ... add more versions as needed
    ] {
        combobox.append_text(&format!("{:?}", version));
    }

    combobox.set_active(Some(3));
    
    // Create a menu item inside the popover
    let menu_item = Button::with_label("Download Weights");
    grid.attach_next_to(&menu_item, Some(&combobox), gtk::PositionType::Bottom, 1, 1);

    // Add the button to the header bar (or wherever you want)
    header_bar.pack_end(&menu_button);
        
    // Connect the popover to the button
    menu_button.connect_clicked(move |_| {
        popover.show_all();
    });
    
    // Connect the popover to the button
    menu_item.connect_clicked(move |_| {
        // Get the selected version from the combobox
        let version_str = combobox.active_text().expect("No version selected!");
        let version = match version_str.as_str() {
            "V1_5" => StableDiffusionVersion::V1_5,
            "V2_1" => StableDiffusionVersion::V2_1,
            "Xl" => StableDiffusionVersion::Xl,
            "Turbo" => StableDiffusionVersion::Turbo,
            // Handle other enum variants as needed
            _ => panic!("Invalid version selected!"),
        };

        // Get the boolean value from the toggle switch
        let is_enabled = toggle_switch.is_active();

        // Call the function with the selected version and boolean value
        download_weights_for_config(version, is_enabled);
    });

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
    right_box.set_border_width(5);
    paned.pack2(&right_box, true, false);

    // Create a text entry at the top of the right box
    let text_view = TextView::new();
    text_view.set_border_width(10);
    text_view.set_editable(true);
    text_view.set_monospace(true);
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_view.set_buffer(Some(&text_buffer));
    let text_scroll = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    text_scroll.add(&text_view);
    right_box.pack_start(&text_scroll, true, true, 0);

    // Create a separator below the text entry
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_bottom(5);
    right_box.pack_start(&separator, false, true, 0);

    // Create a spin button to choose the number of threads
    let spin_button = SpinButton::with_range(1.0, 8.0, 1.0);
    spin_button.set_margin_bottom(5);
    right_box.pack_start(&spin_button, false, true, 0);

    // Create generation button
    let generate_button = Button::with_label("Generate Image");
    right_box.pack_end(&generate_button, false, true, 0);

    generate_button.connect_clicked(move |_| {
        // &text_view.set_progress_fraction(0.5);
        generate_image("An image of a robot on a beach.", true);
    });

    // Connect signals
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // close window keybind
    window.connect_key_press_event(|_, event| {
        if let Some(key) = event.keyval().into() {
            if event.state().contains(gdk::ModifierType::CONTROL_MASK) && key == key::q {
                gtk::main_quit();
                Inhibit(true);
            }
        }
        Inhibit(false)
    });

    // ctrl + enter for infer
    window.connect_key_press_event(move |_, event| {
        if let Some(key) = event.keyval().into() {
            if event.state().contains(gdk::ModifierType::CONTROL_MASK) && key == key::Return {
                generate_button.clicked();
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
