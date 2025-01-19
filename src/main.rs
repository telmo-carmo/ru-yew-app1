/*

rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli

trunk serve

--

wasm-pack build --target web

python3 -m http.server


 */

 use yew::prelude::*;
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
 
 #[function_component(App)]
 fn app() -> Html {
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
        Callback::from(move |_| {
            let value = *counter + 1;
            web_sys::console::log_1(&format!("Button clicked {value}!").into());
            counter.set(value);
        })
     };
 
     html! {
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
                     html! { <p style="color: green;">{"Welcome, Yew!"}</p> }
                 } else {
                     html! { <p style="color: blue;">{"You are not Yew."}</p> }
                 }
             }
 
             <h2>{"Handling Events"}</h2>
             <p>{ *counter }</p>
             <button onclick={on_bt1}>{"Click Me"}</button>
         </div>
     }
 }
 
 fn main() {
    //wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
 }
