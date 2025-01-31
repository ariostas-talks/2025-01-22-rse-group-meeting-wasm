// Based on https://github.com/feral-dot-io/leptos-chartistry/blob/master/demo/src/examples/series_line.rs (MPL-2.0 license)

use leptos::prelude::*;
use leptos_chartistry::*;
use leptos_chartistry::IntoInner;

pub struct MyData {
    x: f64,
    y1: f64,
    y2: f64,
}

impl MyData {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self { x, y1, y2 }
    }
}

#[component]
pub fn SimulationPlot() -> impl IntoView {

    let data = load_data();

    // Lines are added to the series
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("AAA"))
        .line(Line::new(|data: &MyData| data.y2).with_name("BBB"));
    view! {
        <h1>Plot with Leptos Chartistry</h1>
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(500.0, 1.2)
            series=series
            data=data

            // Decorate our chart
            top=RotatedLabel::middle("My stocks")
            left=TickLabels::aligned_floats()
            bottom=Legend::end()
            inner=[
                // Standard set of inner layout options
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                XGridLine::default().into_inner(),
                YGridLine::default().into_inner(),
                YGuideLine::over_mouse().into_inner(),
                XGuideLine::over_data().into_inner(),
            ]
            tooltip=Tooltip::left_cursor().show_x_ticks(false)
        />
    }
}

pub fn load_data() -> Signal<Vec<MyData>> {
    Signal::derive(|| {
        vec![
            MyData::new(0.0, 1.0, 0.0),
            MyData::new(1.0, 3.0, 1.0),
            MyData::new(2.0, 5.0, 2.5),
            MyData::new(3.0, 5.5, 3.0),
            MyData::new(4.0, 5.0, 3.0),
            MyData::new(5.0, 2.5, 4.0),
            MyData::new(6.0, 2.25, 9.0),
            MyData::new(7.0, 3.0, 5.0),
            MyData::new(8.0, 7.0, 3.5),
            MyData::new(9.0, 8.5, 3.2),
            MyData::new(10.0, 10.0, 3.0),
        ]
    })
}