use gtk::prelude::*;
use gtk::{glib, Builder};

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("sync_widgets.ui");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Couldn't add from string");

    let slider: gtk::Scale = builder.object("slider").expect("Couldn't get slider");
    let spin_button: gtk::SpinButton = builder
        .object("spin_button")
        .expect("Couldn't get spin_button");
    let slider_adj = slider.adjustment();
    let spin_button_adj = spin_button.adjustment();
    slider_adj
        .bind_property("value", &spin_button_adj, "value")
        .flags(
            glib::BindingFlags::DEFAULT
                | glib::BindingFlags::SYNC_CREATE
                | glib::BindingFlags::BIDIRECTIONAL,
        )
        .build()
        .unwrap();

    let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.sync_widgets"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}
