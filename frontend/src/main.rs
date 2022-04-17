use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::{
    events::Event,
    prelude::{function_component, html},
    use_state_eq, Callback, TargetCast,
};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeResolve, catch)]
    pub async fn resolve(hostname: String) -> Result<JsValue, JsValue>;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}

#[function_component(App)]
fn effect() -> Html {
    let resolver_output = use_state_eq(|| "resolver output".to_string());
    let name = use_state_eq(|| "blog.adamchalmers.com".to_string());

    let onchange = {
        let message = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            message.set(value);
        })
    };

    let onclick = {
        let resolver_output = resolver_output.clone();
        let hostname = name.to_string();
        let task = move |_| {
            let resolver_output = resolver_output.clone();
            let hostname = hostname.to_string();
            spawn_local(async move {
                let hostname = hostname.clone();
                // This will call our glue code all the way through to the tauri
                // back-end command and return the `Result<String, String>` as
                // `Result<JsValue, JsValue>`.
                let resp = resolve(if hostname.ends_with('.') {
                    hostname
                } else {
                    format!("{hostname}.")
                })
                .await;
                match resp {
                    Ok(message) => {
                        let out = message.as_string().unwrap();
                        resolver_output.set(out);
                    }
                    Err(e) => {
                        let window = window().unwrap();
                        window
                            .alert_with_message(&format!("Error: {:?}", e))
                            .unwrap();
                    }
                }
            })
        };
        // let out = format!("IP address for {hostname}");
        Callback::from(task)
    };

    let resolver_message = (*resolver_output).clone();

    html! {
        <div>
            <input {onchange} value={(*name).clone()} />
            <button {onclick}>{"Resolve"}</button>
            <h1>{ resolver_message }</h1>
            </div>
    }
}
