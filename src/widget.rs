use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Adjustment;

use crate::wave::PlotWaveform;

glib::wrapper! {
    pub struct Chart(ObjectSubclass<imp::Chart>)
        @extends gtk::Widget;
}

impl Chart {
    pub fn new() -> Chart {
        let w: Self = glib::Object::new();

        w.imp().hscroll.borrow().set_hexpand(true);
        w
    }

    pub fn add_wave(&mut self, w: PlotWaveform) {
        self.imp().state.borrow_mut().waves.push(w);
    }
}

pub struct AxisView {
    data_min: f64,
    data_max: f64,
    window_min: f64,
    window_max: f64,
}

impl Default for AxisView {
    fn default() -> Self {
        Self {
            data_min: 0.0,
            data_max: 1.0,
            window_min: 0.2,
            window_max: 0.8,
        }
    }
}

impl AxisView {
    pub fn get_window(&self) -> (f64, f64) {
        (self.window_min, self.window_max)
    }

    fn adjustment(&self) -> Adjustment {
        let value = self.window_min;
        let lower = self.data_min;
        let upper = self.data_max;
        let page_size = self.window_max - self.window_min;
        let step_increment = 0.1 * page_size;
        let page_increment = 0.8 * page_size;
        Adjustment::new(
            value,
            lower,
            upper,
            step_increment,
            page_increment,
            page_size,
        )
    }
}

#[derive(Default)]
pub struct View {
    pub x: AxisView,
    pub y: AxisView,
}

#[derive(Default)]
pub struct ChartState {
    pub waves: Vec<PlotWaveform>,
    pub view: View,
}

mod imp {
    use std::cell::RefCell;

    use gtk;
    use gtk::glib;
    use gtk::glib::prelude::*;
    use gtk::glib::subclass::prelude::*;
    use gtk::glib::subclass::Signal;
    use gtk::prelude::*;
    use gtk::subclass::widget::WidgetClassExt;
    use gtk::subclass::widget::WidgetImpl;

    use crate::plotarea::PlotArea;

    use super::ChartState;
    use super::View;

    pub struct Chart {
        pub plotarea: RefCell<PlotArea>,
        pub hscroll: RefCell<gtk::Scrollbar>,
        pub state: RefCell<ChartState>,
    }

    impl Default for Chart {
        fn default() -> Self {
            let view = View::default();
            Self {
                plotarea: RefCell::new(PlotArea::new()),
                hscroll: RefCell::new(gtk::Scrollbar::new(
                    gtk::Orientation::Horizontal,
                    Some(&view.x.adjustment()),
                )),
                state: RefCell::new(ChartState::default()),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Chart {
        const NAME: &'static str = "Chart";
        type Type = super::Chart;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BoxLayout>();
        }
    }

    impl ObjectImpl for Chart {
        fn constructed(&self) {
            self.parent_constructed();
            let w = self.obj();

            let layout = w.layout_manager().unwrap().downcast::<gtk::BoxLayout>().unwrap();
            layout.set_orientation(gtk::Orientation::Vertical);

            self.plotarea.borrow().set_parent(&*w);
            self.plotarea.borrow().set_vexpand(true);
            self.hscroll.borrow().set_parent(&*w);
            self.hscroll.borrow().set_hexpand(true);
        }

        fn dispose(&self) {
            self.plotarea.borrow().unparent();
            self.hscroll.borrow().unparent();
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
    impl WidgetImpl for Chart {}
}
