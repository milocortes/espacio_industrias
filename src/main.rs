use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use charming::{
    component::Legend,
    element::{Label, LabelLayout, LabelPosition, LineStyle, ScaleLimit, Tooltip},
    series::{Graph, GraphData, GraphLayout},
    Chart,WasmRenderer
};

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    info!("starting app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let renderer = use_signal(|| WasmRenderer::new(1500, 700));

    use_effect(move || {
        let data: GraphData = serde_json::from_str(include_str!("espacio_producto_format.json")).unwrap();
        let chart = Chart::new()
        .tooltip(Tooltip::new())
        .legend(Legend::new().data(data.categories.iter().map(|c| c.name.clone()).collect()))
        .series(
            Graph::new()
                .name("Rama")
                .layout(GraphLayout::None)
                .roam(true)
                .label(
                    Label::new()
                        .show(false)
                        .position(LabelPosition::Right)
                        .formatter("{b}"),
                )
                .label_layout(LabelLayout::new().hide_overlap(true))
                .scale_limit(ScaleLimit::new().min(0.4).max(2.0))
                .line_style(LineStyle::new().color("source").curveness(0.3))
                .data(data),
        );
        renderer.read_unchecked().render("chart", &chart).unwrap();
    });

    rsx! (
        div { style: "text-align: center;",
            h1 { "Espacio de Industrias México" }
            //h3 { "Frontend that scales." }
            //p {
            //    "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust."
            //}
        }
        div { style: "width: 100%; text-align: center;",
            div { id: "chart", style: "display: inline-block;" }
        }
    )
}