mod plotarea;
mod wave;
mod widget;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use wave::{PlotWaveform, Waveform};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let vbox = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    vbox.append(&button);

    let mut w = widget::Chart::new();
    w.add_wave(PlotWaveform::new(Waveform::example(), "sine wave"));

    w.set_vexpand(true);

    // let lbl = Label::new(Some("Text"));
    vbox.append(&w);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    // Present window
    window.present();
}
