use crate::app;
use gio::prelude::*;
use gtk::prelude::*;
use std::sync::mpsc;

pub enum Signal {}

pub struct Ui {
    from_app: glib::Receiver<Signal>,
    to_app: mpsc::Sender<app::Signal>,
    application: gtk::Application,
}

impl Ui {
    pub fn new(from_app: glib::Receiver<Signal>, to_app: mpsc::Sender<app::Signal>) -> Ui {
        let application = gtk::Application::new(
            Some("nl.frankgroeneveld.timer-for-harvest"),
            Default::default(),
        )
        .unwrap();
        Ui {
            from_app: from_app,
            to_app: to_app,
            application: application,
        }
    }

    pub fn run(ui: Ui) {
        ui.application.connect_activate(move |application| {
            let window = Ui::main_window(application);
            window.show_all();
        });

        ui.from_app.attach(None, move |event| glib::Continue(true));
        ui.application.run(&[]);
    }

    pub fn main_window(application: &gtk::Application) -> gtk::ApplicationWindow {
        let window = gtk::ApplicationWindow::new(application);
        let container = gtk::HeaderBar::new();

        container.set_title(Some("Harvest"));
        container.set_show_close_button(true);

        window.set_title("Harvest");
        window.set_titlebar(Some(&container));
        window.set_border_width(18);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(500, 300);
        window.set_size_request(500, 300);

        window.add_events(gdk::EventMask::KEY_PRESS_MASK);

        let button =
            gtk::Button::new_from_icon_name(Some("list-add-symbolic"), gtk::IconSize::Button);
        button.set_sensitive(false);
        container.pack_start(&button);

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        hbox.set_spacing(0);
        hbox.get_style_context().add_class(&gtk::STYLE_CLASS_LINKED);
        let prev_button =
            gtk::Button::new_from_icon_name(Some("go-previous-symbolic"), gtk::IconSize::Button);
        hbox.pack_start(&prev_button, false, false, 0);

        let next_button =
            gtk::Button::new_from_icon_name(Some("go-next-symbolic"), gtk::IconSize::Button);
        hbox.pack_start(&next_button, false, false, 0);
        container.pack_start(&hbox);

        window
    }
}
