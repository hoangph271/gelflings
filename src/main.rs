use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let (_, items) = use_state(&cx, || Vec::<String>::new());

    cx.render(rsx!{
        button {
            onclick: move |_| {
                items.modify(|items| {
                    [&items[..], &[format!("{}", items.len())]].concat()
                })
            },
            "Add one...!"
        },
        code {
            style: "color: red;",
            [format_args!("Items: {}", items.current().len())]
        }
        ul {
            items.current().iter().map(|val| rsx!(li { "Item: #{val}" }))
        }
    })
}
