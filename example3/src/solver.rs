#![allow(non_snake_case)]
use clarabel::algebra::*;
use clarabel::solver::*;
use leptos::prelude::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;

fn problem_data(n: usize) -> (CscMatrix<f64>, Vec<f64>, CscMatrix<f64>, Vec<f64>) {
    let P = CscMatrix::identity(n);

    // construct A = [I; -I]
    let I1 = CscMatrix::<f64>::identity(n);
    let mut I2 = CscMatrix::<f64>::identity(n);
    I2.negate();

    let A = CscMatrix::vcat(&I1, &I2);

    let q = vec![1.; n];
    let b = vec![1.; 2 * n];

    (P, q, A, b)
}

fn solve_problem(n: usize) -> Vec<f64> {
    let (P, q, A, b) = problem_data(n);

    let cones = [NonnegativeConeT(b.len())];

    let settings = DefaultSettingsBuilder::default()
        .equilibrate_enable(true)
        .max_iter(50)
        .build()
        .unwrap();

    let mut solver = DefaultSolver::new(&P, &q, &A, &b, &cones, settings);

    solver.solve();

    solver.solution.x
}

#[component]
pub fn ExampleProblem() -> impl IntoView {

    let (n, set_n) = signal(0);
    let (res, set_res) = signal("Result:".to_string());

    let input_element: NodeRef<Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        let value = input_element.get()
            .expect("Input element not found")
            .value()
            .parse::<usize>()
            .unwrap_or(0);

        let solution = solve_problem(value);

        let solution_str = format!("Result: {:?}", solution);

        set_res.set(solution_str);
        set_n.set(value);
    };

    view! {
        <h1>"Solving with Clarabel"</h1>
        <form on:submit=on_submit>
            <input 
                type="text"
                value=n
                node_ref=input_element
            />
            <input type="submit" value="Compute Fn"/>
        </form>
        <p>{res}</p>
    }
}