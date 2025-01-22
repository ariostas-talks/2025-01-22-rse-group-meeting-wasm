use leptos::prelude::*;

pub mod edg_inputs;
pub mod plotting;
pub mod solver;

fn main() {
    let res_data = edg_inputs::load_fuels_input(2030);
    let df_print = res_data.to_string().replace("\n", "<br/>");
    leptos::mount::mount_to_body(move || view! {
        <h1>"Loaded dataframe with Polars"</h1>
        <div inner_html=df_print></div> 
    });

    leptos::mount::mount_to_body(solver::ExampleProblem);

    leptos::mount::mount_to_body(plotting::SimulationPlot);
}

