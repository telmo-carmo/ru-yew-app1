/*

see: https://github.com/jetli/awesome-yew#component-libraries

--

rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

trunk serve

--
OR:
wasm-pack build --target web
python3 -m http.server


*/

use yew::prelude::*;
use yew_router::prelude::*;
//use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(Greeting)]
fn greeting(props: &Props) -> Html {
    html! {
        <p>{ format!("Hello, {}!", props.name) }</p>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/form1")]
    Form1,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_routes(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Form1 => html! { <Form1 /> },
        Route::NotFound => html! { <div>
            <h1>{"404 Not Found"}</h1>
            <Link<Route> to={Route::Home}>{ "Go to Home Page" }</Link<Route>>
        </div> },
    }
}

#[function_component(Form1)]
fn form1() -> Html {
    let on_bt1 = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&Route::About);
        })
    };

    html! {
      <div>
        <NavBar />
        <div class="container">
            <h1 class="h1_title">{"Form1 Page"}</h1>
            <Link<Route> to={Route::Home}>{ "Go to Home Page" }</Link<Route>>
            <button onclick={on_bt1} class="mui-btn mui-btn--primary">{"Go About"}</button>
        </div>
      </div>
    }
}

#[function_component(About)]
fn about() -> Html {
    html! {
      <div>
        <NavBar />
        <div class="container">
            <h1 class="h1_title">{"About Page"}</h1>
            <Link<Route> to={Route::Home}>{ "Go to Home Page" }</Link<Route>>
        </div>
      </div>
    }
}

#[function_component(NavBar)]
fn navbar() -> Html {
    html! {
        <nav style="display: flex; justify-content: space-around; background-color: #f0f0f0; padding: 0.5rem;">
            <Link<Route> to={Route::Home} classes="link-style">
                {"Home"}
            </Link<Route>>
            <Link<Route> to={Route::Form1} classes="link-style">{"Form1"}</Link<Route>>
            <Link<Route> to={Route::About} classes="link-style">{"About"}</Link<Route>>
        </nav>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let counter = use_state(|| 0);
    let name = use_state(|| String::from("World")); // State for the name
    let name_input_ref = NodeRef::default(); // Ref for the input element

    let update_name = {
        let name = name.clone();
        let name_input_ref = name_input_ref.clone(); // Clone for use in the closure
        Callback::from(move |_| {
            if let Some(input) = name_input_ref.cast::<web_sys::HtmlInputElement>() {
                name.set(input.value());
            }
        })
    };

    let on_bt1 = {
        let counter = counter.clone();
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            let value = *counter + 1;
            web_sys::console::log_1(&format!("Button clicked {value}!").into());
            if value == 5 {
                navigator.push(&Route::About);
            } else {
                counter.set(value);
            }
        })
    };

    html! {
        <div>
          <NavBar />
          <div>
            <h1>{ "Yew App 1 Main" }</h1>
            <input type="text" ref={name_input_ref} placeholder="Enter your name" onchange={update_name}/>
            <Greeting name={(*name).clone()} />

            <h2>{"List Example"}</h2>
            <ul>
                {
                    (1..=5).map(|i| {
                        html! { <li>{ format!("Item {}", i) }</li> }
                    }).collect::<Html>()
                }
            </ul>

            <h2>{"Conditional Rendering Example"}</h2>
            {
                if *name == "Yew" {
                    html! {
                      <div>
                        <p style="color: green;">{"Welcome, Yew!"}</p>
                        <Link<Route> to={Route::About}>{ "Go to About Page" }</Link<Route>>
                      </div>
                    }
                } else {
                    html! { <p style="color: blue;">{"You are not Yew."}</p> }
                }
            }

            <h2>{"Handling Events"}</h2>
            <p>{ *counter }</p>
            <button onclick={on_bt1}>{"Click Me"}</button>
          </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch_routes} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    //wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
