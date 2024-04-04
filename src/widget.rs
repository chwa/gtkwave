use gtk::glib;

glib::wrapper! {
    pub struct MyWidget(
        ObjectSubclass<imp::MyWidget>)
        @extends gtk::Widget;
}

impl MyWidget {
    pub fn new() -> MyWidget {
        glib::Object::new()
    }
}

mod imp {
    use gtk;
    use gtk::cairo;
    use gtk::glib;
    use gtk::glib::prelude::*;
    use gtk::glib::subclass::prelude::*;
    use gtk::glib::subclass::Signal;
    use gtk::graphene::Rect;
    use gtk::prelude::*;
    use gtk::subclass::widget::WidgetImpl;

    pub struct MyWidget {}

    impl MyWidget {
        fn draw(&self, cr: &cairo::Context) {
            let width = self.obj().width() as f64;
            let height = self.obj().height() as f64;
            let left = (width * 0.05).round() + 0.5;
            let bot = (height * 0.05).round() + 0.5;
            let width = (width * 0.9).round();
            let height = (height * 0.9).round();

            cr.rectangle(left, bot, width, height);

            cr.set_source_rgb(0.15, 0.15, 0.15);
            cr.fill_preserve().unwrap();

            cr.set_source_rgb(0.7, 0.7, 0.7);
            cr.set_line_width(1.0);
            cr.stroke().unwrap();

            for i in 1..=9 {
                cr.move_to(left + (i as f64 / 10.0 * width).round(), bot);
                cr.line_to(left + (i as f64 / 10.0 * width).round(), bot + height);
            }
            cr.set_dash(&[3.0, 3.0], 0.0);
            cr.set_line_width(1.0);
            cr.set_source_rgb(0.3, 0.3, 0.3);
            cr.stroke().unwrap();

            for i in 1..=9 {
                cr.move_to(left, bot + (i as f64 / 10.0 * height).round());
                cr.line_to(left + width, bot + (i as f64 / 10.0 * height).round());
            }
            cr.set_line_width(1.0);
            cr.set_source_rgb(0.3, 0.3, 0.3);
            cr.stroke().unwrap();

            cr.set_dash(&[], 0.0);

            cr.move_to(left, bot + height / 2.0);
            for i in 0..=100 {
                cr.line_to(
                    left + (i as f64) / 100.0 * width,
                    bot + height / 2.0 - (height * 0.4) * (0.13 * i as f64).sin(),
                )
            }
            cr.set_source_rgb(0.8, 0.2, 0.8);
            cr.stroke().unwrap();

            cr.move_to(left, bot + height / 2.0);
            for i in 0..=100 {
                cr.line_to(
                    left + (i as f64) / 100.0 * width,
                    bot + height / 2.0 - (height * 0.03) * (0.33 * (i + 30) as f64).sin()
                        + (height * 0.13) * (0.11 * i as f64).sin(),
                )
            }
            cr.set_line_width(2.0);
            cr.set_source_rgb(0.1, 0.6, 0.7);
            cr.stroke().unwrap();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MyWidget {
        const NAME: &'static str = "MyWidget";
        type Type = super::MyWidget;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            Self {}
        }
    }

    impl ObjectImpl for MyWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let w = self.obj();
            w.set_size_request(640, 480);
        }
        fn signals() -> &'static [Signal] {
            use once_cell::sync::Lazy;
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("rating-changed")
                    .param_types([<i32>::static_type()])
                    .run_last()
                    .build()]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![glib::ParamSpecBoolean::builder("auto-update")
                    .nick("Auto-update")
                    .blurb("Whether to auto-update or not")
                    .default_value(true)
                    .read_only()
                    .build()]
            });
            PROPERTIES.as_ref()
        }

        // fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        //     /* ... */
        // }

        // fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        //     let none: Option<&str> = None;
        //     none.to_value()
        // }
    }
    impl WidgetImpl for MyWidget {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let w = self.obj();
            let (width, height) = (w.width() as f32, w.height() as f32);

            let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));

            self.draw(&cr)
        }
    }
}
