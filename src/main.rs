mod diffusion;

extern crate gtk;
use gio::glib::clone;
use gtk::prelude::*;
use gdk::{keys::constants as key};
use std::path::Path;
use gtk::{Application, Spinner, Grid, Switch, ComboBoxText, Popover, ApplicationWindow, MenuButton, Box, Button, Image, TextTagTable, Menu, MenuBar, Adjustment, MenuItem, Orientation, Paned, Separator, SpinButton, TextView, TextBuffer};
use crate::diffusion::{generate_image, download_weights_for_config, StableDiffusionVersion};
use std::path::PathBuf;
use std::fs;
use gtk::MessageDialog;
use gtk::MessageType;
use gtk::FileChooserDialog;
use gtk::FileChooserAction;
use gtk::Window;
use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};

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
    window.set_default_size(800, 512);

    // Create a header bar
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title(Some("Groucho"));

    // Create the "Save" button on the left side of the header bar
    let save_button = Button::with_label("Save");
    header_bar.pack_start(&save_button);

    // Create a spinner
    let spinner = Spinner::new();
    header_bar.pack_start(&spinner); // Add the spinner to the end of the header bar

    // Create a button to trigger the popover
    let menu_button = Button::with_label("Settings");

    // Create the popover
    let popover = Popover::new(Some(&menu_button));

    // Create a grid to organize widgets with labels
    let grid = Grid::new();
    grid.set_column_spacing(5);
    grid.set_row_spacing(5);
    grid.set_border_width(5);
    popover.add(&grid);
   
    // Create a menu item inside the popover
    let menu_item = Button::with_label("Download Weights");
    grid.attach(&menu_item, 0, 0, 1, 1);

    let cache_button = Button::with_label("Cache");
    grid.attach_next_to(&cache_button, Some(&menu_item), gtk::PositionType::Left, 1, 1);
   
    // Connect the button click event to open the file manager at the specified path
    cache_button.connect_clicked(move |_| {

    });
    
    // Add the button to the header bar (or wherever you want)
    header_bar.pack_end(&menu_button);
    
    // Connect the popover to the button
    menu_button.connect_clicked(move |_| {
        popover.show_all();
    });

    // Add the header bar to the application window
    window.set_titlebar(Some(&header_bar));

    // Create a vertical box to hold the main content
    let vbox = Box::new(Orientation::Vertical, 0);

    // Create a paned widget to separate the main window horizontally
    let paned = Paned::new(Orientation::Horizontal);
    vbox.pack_start(&paned, true, true, 0);
    paned.set_position(512);

    // Create a scrollable image on the left side of the paned
    let image_scroll = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    let image = Image::new();
    image_scroll.add(&image);
    paned.pack1(&image_scroll, true, false);

    // Create a vertical box to hold the text entry and spin button on the right side of the paned
    let right_box = Grid::new();
    right_box.set_border_width(5);
    right_box.set_hexpand(true);
    paned.pack2(&right_box, true, false);

    // Create a text entry at the top of the right box
    let text_view = TextView::new();
    text_view.set_border_width(10);
    text_view.set_editable(true);
    text_view.set_monospace(true);
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_hexpand(true);
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_view.set_buffer(Some(&text_buffer));
    let text_scroll = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    text_scroll.add(&text_view);
    text_scroll.set_hexpand(true);
    text_scroll.set_vexpand(true);
    right_box.attach(&text_scroll, 0, 0, 2, 1);

    // Create a separator below the text entry
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_bottom(5);
    right_box.attach_next_to(&separator, Some(&text_scroll), gtk::PositionType::Bottom, 2, 1);

    // create Switchboard
    let switchboard = gtk::Box::new(Orientation::Horizontal, 0);
    right_box.attach_next_to(&switchboard, Some(&separator), gtk::PositionType::Bottom, 2, 1);

    // Add a label and toggle switch for "f16"
    let label_f16 = gtk::Label::new(Some("Use f16:"));
    let toggle_switch = Switch::new();
    switchboard.pack_start(&label_f16, true, false, 5);
    switchboard.pack_start(&toggle_switch, false, false, 5);
    toggle_switch.set_halign(gtk::Align::Center);

    // Add a label and toggle switch for "f16"
    let label_CPU = gtk::Label::new(Some("Use CPU:"));
    let toggle_CPU = Switch::new();
    switchboard.pack_start(&label_CPU, true, false, 5);
    switchboard.pack_start(&toggle_CPU, false, false, 5);
    toggle_CPU.set_halign(gtk::Align::Center);

    // Add guidance scale controls
    let guidance_box = gtk::Box::new(Orientation::Horizontal, 0);
    let guidance_label = gtk::Label::new(Some("Guidance Scale:"));
    let guidance_enable = Switch::new();
    let guidance_scale = gtk::Scale::new(Orientation::Horizontal, Some(&Adjustment::new(1.0, 1.0, 26.0, 1.0, 1.0, 1.0)));
    guidance_box.pack_start(&guidance_label, false, false, 5);
    guidance_enable.set_vexpand(false);
    guidance_enable.set_halign(gtk::Align::Center); 
    guidance_enable.set_valign(gtk::Align::Center); 
    guidance_box.pack_start(&guidance_enable, false, false, 5);
    guidance_box.pack_start(&guidance_scale, true, true, 5);
    right_box.attach_next_to(&guidance_box, Some(&switchboard), gtk::PositionType::Bottom, 2, 1);

    // Add a label and combobox for version selection
    let label_version = gtk::Label::new(Some("Version:"));
    let combobox = ComboBoxText::new();
    combobox.set_margin_top(5);
    combobox.set_margin_bottom(5);
    right_box.attach_next_to(&label_version, Some(&guidance_box), gtk::PositionType::Bottom, 1, 1);
    right_box.attach_next_to(&combobox, Some(&label_version), gtk::PositionType::Right, 1, 1);

    // Create a spin button to choose the number of samples
    let spin_button = SpinButton::with_range(1.0, 50.0, 1.0);
    spin_button.set_margin_bottom(5);
    let spinlabel = gtk::Label::new(Some("Samples:"));
    right_box.attach_next_to(&spinlabel, Some(&label_version), gtk::PositionType::Bottom, 1, 1);
    right_box.attach_next_to(&spin_button, Some(&spinlabel), gtk::PositionType::Right, 1, 1);

    // Load variables
    // let cpu = cpu_toggle_switch.is_active();

    // Create generation button
    let generate_button = Button::with_label("Generate Image");
    right_box.attach_next_to(&generate_button, Some(&spinlabel), gtk::PositionType::Bottom, 2, 1);

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

    // Set settings
    combobox.set_active(Some(3));
    toggle_switch.set_active(true);
    toggle_CPU.set_active(true);

    // Create clones
    let clone_combo0 = combobox.clone();
    let clone_f16 = toggle_switch.clone();
    // Connect the popover to the button
    menu_item.connect_clicked(move |_| {
        // Get the selected version from the combobox
        let version_str = &clone_combo0.active_text().expect("No version selected!");
        let version = match version_str.as_str() {
            "V1_5" => StableDiffusionVersion::V1_5,
            "V2_1" => StableDiffusionVersion::V2_1,
            "Xl" => StableDiffusionVersion::Xl,
            "Turbo" => StableDiffusionVersion::Turbo,
            // Handle other enum variants as needed
            _ => panic!("Invalid version selected!"),
        };

        // Get the boolean value from the toggle switch
        let is_enabled = clone_f16.is_active();

        // Call the function with the selected version and boolean value
        download_weights_for_config(version, is_enabled);
    });

    // Connect the save_button click event to open a save dialog and copy the file
    save_button.connect_clicked(move |_| {
        // Get the parent window of the save_button
        let parent_window = separator.toplevel().and_then(|toplevel| toplevel.downcast::<Window>().ok());
        save();
    });

    let clone_combo1 = combobox.clone();
    generate_button.connect_clicked(move |_| {
        spinner.start();
        // &text_view.set_progress_fraction(0.5);
        // Define a variable typed Option<f64>
        let guide: Option<f64> = if guidance_enable.state() == true {
            Some(guidance_scale.digits() as f64)
        } else {
            None
        };
        let prompt: String = text_buffer.text(&text_buffer.start_iter(), &text_buffer.end_iter(), true).unwrap().to_string();
        let f16 = toggle_switch.is_active();
        let cpu = toggle_CPU.is_active();
        let samples = spin_button.value() as usize;
        let sd_version = get_selected_sd_version(&clone_combo1);
        generate_image(&prompt, cpu, f16, sd_version, samples, guide);
        image.set_from_file(Some("groucho.png"));
        spinner.stop();
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

    // changes spinnerwheel
    combobox.connect_changed(move |_| {
        
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

// Helper function to get the selected StableDiffusionVersion from the combobox
fn get_selected_sd_version(combobox: &ComboBoxText) -> StableDiffusionVersion {
    if let Some(active_text) = combobox.active_text() {
        match active_text.as_str() {
            "V1_5" => StableDiffusionVersion::V1_5,
            "V2_1" => StableDiffusionVersion::V2_1,
            "Xl" => StableDiffusionVersion::Xl,
            "Turbo" => StableDiffusionVersion::Turbo,
            _ => panic!("Invalid version selected!"),
        }
    } else {
        panic!("No version selected!");
    }
}

async fn save() -> ashpd::Result<()> {
    let files = SelectedFiles::save_file()
        .title("open a file to write")
        .accept_label("write")
        .current_name("image.png")
        .modal(true)
        .filter(FileFilter::new("PNG Image").glob("*.png"))
        .send()
        .await?
        .response()?;

    println!("{:#?}", files);

    Ok(())
}
