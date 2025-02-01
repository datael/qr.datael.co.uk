use leptos::prelude::*;

mod components;

#[component]
fn App() -> impl IntoView {
    let (url, set_url) = signal("https://qr.datael.co.uk/".to_string());

    let qr = move || {
        view! {
            <components::qr_code::QRCode url=url() />
        }
    };

    view! {
        <div>
            <h1>"QR Code Generator"</h1>
            <label>"URL to encode:"<br/>
            <input
                type="text"
                placeholder="URL to encode"
                prop:value=url
                on:input:target=move |ev| {
                    set_url(ev.target().value());
                }
            />
            </label>
            <br />
            <br />
            {qr}
        </div>
    }
}

pub fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    });
}
