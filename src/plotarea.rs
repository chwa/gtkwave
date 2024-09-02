use gtk::glib;

glib::wrapper! {
    pub struct PlotArea(ObjectSubclass<imp::PlotArea>)
        @extends gtk::Widget;
}

impl PlotArea {
    pub fn new() -> PlotArea {
        let w: Self = glib::Object::new();
        w
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
    use gtk::subclass::widget::WidgetClassExt;
    use gtk::subclass::widget::WidgetImpl;

    use crate::wave::PlotWaveform;
    use crate::widget::Chart;
    use crate::widget::ChartState;

    pub struct PlotArea {}

    impl PlotArea {
        fn draw(&self, cr: &cairo::Context, state: &ChartState) {
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

            let xlim = state.view.x.get_window();
            let ylim = state.view.y.get_window();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PlotArea {
        const NAME: &'static str = "PlotArea";
        type Type = super::PlotArea;
        type ParentType = gtk::Widget;

        fn new() -> Self {
            Self {}
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BoxLayout>();
        }
    }

    impl ObjectImpl for PlotArea {
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
    }
    impl WidgetImpl for PlotArea {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let w = self.obj();
            let parent = w.parent().and_downcast::<Chart>().unwrap();
            let (width, height) = (w.width() as f32, w.height() as f32);

            let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));
            self.draw(&cr, &parent.imp().state.borrow());
        }
    }
}
