// todo remove deprecated
#![allow(non_snake_case, deprecated)]

use chrono::prelude::*;
use dioxus::events::{KeyCode, KeyboardEvent};
use dioxus::html::br;
use dioxus::prelude::*;
use dioxus_tui::TuiContext;

fn main() {
    dioxus_tui::launch_cfg(
        App,
        dioxus_tui::Config::new()
            // .without_ctrl_c_quit()
            // Some older terminals only support 16 colors or ANSI colors
            // If your terminal is one of these, change this to BaseColors or ANSI
            .with_rendering_mode(dioxus_tui::RenderingMode::Rgb),
    );
}

fn App(cx: Scope) -> Element {
    let tui_ctx: TuiContext = cx.consume_context().unwrap();

    let time = chrono::Utc::now();
    let epoch = time.timestamp() as f64;
    let stardate: f64 = (1000.0 * ((epoch / 365.25) + 25.0)) + epoch / 86400.0;

    let bg_status = use_state(cx, || false);

    cx.render(rsx! {
        div {
            width: "100%",
            height: "100%",
            justify_content: "center",
            align_items: "center",
            background_color: if *bg_status.get() { "green" } else { "black" },
            onkeydown: move |k: KeyboardEvent| if let KeyCode::Q = k.key_code {
                tui_ctx.quit();
            },
            div {
                // width: "100%",
                // height: "10px",
                display: "flex",
                flex_direction: "column",
                // background_color: "",
                p{
                    font_size: "20px",
                    "Captain's Log",
                }
                br {},
                div {
                    background_color: "blue",
                    // width: "50%",
                    // height: "10px",
                    hr {},
                    p {
                        "Stardate: {stardate}"
                    },
                },
                input {
                oninput: |data| {
                    // println!("input: {:?}", data);
                },
                r#type: "text",
                width: "100%",
                height: "70%",
                placeholder: "Enter your log here",
                required: true,
                min: 10,
            }
                input {
                    oninput: |_| {
                        bg_status.set(true)
                    },
                    r#type: "button",
                    value: "Submit to Starfleet",
                    width: "100%",
                    height: "30%",
                }
            }
        }
    })
}
