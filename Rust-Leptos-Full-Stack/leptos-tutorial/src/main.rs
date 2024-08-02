use leptos::*;
// use leptos_dom::log;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    // Dynamic Styles
    // let (x, set_x) = create_signal(0);
    view! {
            <button
                on:click=move |_| {
                    // set_x.update(|n| *n += 1);
                    set_count.update(|n| *n += 1);
                }

                class:red=move || count() % 2 == 1
            >


                "Click me"
            </button>

            <br/>

            <progress
                max="50"
                value=count
            >

            </progress>
            <br/>

            <progress
                max="50"
                value=double_count
            >
            </progress>
            <p>"Count: " {count}</p>
            <p>"Double Count: " {double_count}</p>


    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    // mount_to_body(|| view! { <p>"Hello, new world!"</p>})

    leptos::mount_to_body(App)
}

// view! {
//     <button
//         on:click=move |_| {
//             // set_count(3);
//             set_count.update(|n| *n += 1);
//         }

//         class:red = move|| count() % 2 == 1
//         // class=("button-20", move || count() % 2 == 1)
//     >
//         "Click me: "
//         {move || count()}
//         </button>
// }

// set the 'style'
// style = "position: absolute"
// // and toggle individual CSS properties with 'style:'
// style:left=move || format!("{}px", x() + 100)
// style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
// style:max-width="400px"
// // Set a CSS variable for stylesheet use
// style=("--columns", x)
// >
//     "Click to Move"
//     </button>

//     <progress
//         max="50"
//         value=move || count() * 2
//         />
