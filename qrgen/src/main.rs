pub mod generator;

use leptos::{*, ev::SubmitEvent};
use generator::QrSVG;
use rand::Rng;

#[component]
fn QrInput (cx: Scope) -> impl IntoView {

    use leptos::html::Input;
    let (message, set_message) = create_signal(cx,"Enter string to be encoded".to_string());
    let (filename, set_filename) = create_signal(cx, "qrcode.svg");
    let input_element: NodeRef<Input> = create_node_ref(cx);
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_message(value);
        let qid: String = format!("qrcode{}.svg", rand::thread_rng().gen_range(100..1000)); // generate random filename to update with
        
        set_filename.update(move|n| *n = &qid); // weird ass closure lifetime bug
        let _literal = move |value: String, qid: String|{QrSVG::new(value.clone()).to_svg(&qid,400)};
        
    };
    view! { cx,
        <form on:submit=on_submit>
            <input type="text"
                value = message
                node_ref=input_element
            />
            <input type="submit" value="Generate"/>
        </form>
        <p>"Generating: "{message}</p>
        <div>
            <img src=move || filename() alt="qrcode"/>
        </div>

    }
}


#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <p>"Hello world"</p>
        <QrInput/>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
