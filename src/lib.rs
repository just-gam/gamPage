use wasm_bindgen::prelude::*;
use web_sys::console;
use yew::prelude::*;

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_state(|| 0);

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            console::log_1(&"Incrementando".into());
            counter.set(*counter + 1)
        })
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| {
            console::log_1(&"Decrementando".into());
            counter.set(*counter - 1)
        })
    };

    html! {
        <div class="counter">
            <h1>{ "Contador" }</h1>
            <div class="counter-display">
                <button onclick={decrement}>{ "-" }</button>
                <span>{ *counter }</span>
                <button onclick={increment}>{ "+" }</button>
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    console::log_1(&"App iniciada".into());
    html! {
        <div class="app">
            <Counter />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console::log_1(&"Iniciando aplicación".into());
    yew::Renderer::<App>::new().render();
}