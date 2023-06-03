pub mod generator;

use sycamore::prelude::*;
use generator::QrSVG;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());

    let displayed_name = || {
        if name.get().is_empty() {
            "https://www.google.com".to_string()
        } else {
            name.get().as_ref().clone()
        }
    };

    let generate_qr = move |_| {
        
        if name.get().is_empty() {
            let qr_code = QrSVG::new("https://www.google.com".to_string());
            qr_code.to_svg("qrcode.svg", 500).unwrap();
            
        } else {
            let qr_code = QrSVG::new(name.get().as_ref().clone());
            qr_code.to_svg("qrcode.svg", 500).unwrap();
            
        }
        
    };

    view! { cx,
        div {
            h1 {
                "QR Code Generated from "
                (displayed_name())
                "!"
            }
            div {
                button(type="button", on:click = generate_qr){
                    "Generate QR Code"
                }
                img(src="public/qrcode.svg", alt = "QR Code")

            }
            input(placeholder="Enter text to encode", bind:value=name)
        }
    }
}

// trunk serve to run
fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| view! { cx, App {} });
}