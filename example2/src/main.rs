use leptos::prelude::*;
use leptos::html::Input;
use leptos::ev::SubmitEvent;

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut t1 = 0;
    let mut t2 = 1;

    for _ in 1..n {
        let t = t1 + t2;
        t1 = t2;
        t2 = t;
    }

    t2
}

#[component]
fn App() -> impl IntoView {
    let (n, set_n) = signal(0);
    let (res, set_res) = signal(0);

    let input_element: NodeRef<Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        let value = input_element.get()
            .expect("Input element not found")
            .value()
            .parse::<u32>()
            .unwrap_or(0);

        set_res.set(fibonacci(value));
        set_n.set(value);
    };

    view! {
        <h1>"WebAssembly Fibonacci"</h1>
        <form on:submit=on_submit>
            <input 
                type="text"
                value=n
                node_ref=input_element
            />
            <input type="submit" value="Compute Fn"/>
        </form>
        <p>"F"<sub>{n}</sub> " = " {res}</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
