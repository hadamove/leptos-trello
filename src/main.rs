use leptos::html::Div;
use leptos::*;
use leptos_use::*;

struct Card {
    name: String,
    description: String,
}

#[component]
fn CardView(cx: Scope, card: Card) -> impl IntoView {
    let el = create_node_ref::<Div>(cx);
    let (is_dragging, set_is_dragging) = create_signal(cx, false);

    let UseDraggableReturn { style, .. } = use_draggable_with_options(
        cx,
        el,
        UseDraggableOptions::default()
            .prevent_default(true)
            .on_move(move |_| {
                set_is_dragging(true);
            })
            .on_end(move |_| {
                set_is_dragging(false);
            }),
    );

    view! { cx,
        <div
            node_ref=el
            class=move || format!("bg-white rounded p-4 mb-4 {}", if is_dragging.get() { "shadow-lg pointer-events-none max-w-xs" } else { "" })
            style=move || if is_dragging.get() { format!("position: fixed; {}", style.get()) } else {"".to_owned()}
        >
            <h2 class="text-lg font-bold">{card.name}</h2>
            <p class="text-gray-600">{card.description}</p>
        </div>
    }
}

#[component]
fn CardList(cx: Scope, title: String, cards: Vec<Card>) -> impl IntoView {
    let el = create_node_ref::<Div>(cx);
    let is_hovered = use_element_hover(cx, el);

    view! { cx,
        <div class="flex-1 max-w-sm bg-gray-100 rounded p-4" node_ref=el>
            <h1 class="text-xl font-bold mb-4 text-blue-600">{title}</h1>
            {cards
                .into_iter()
                .map(|card| view! { cx, <CardView card/> })
                .collect_view(cx)}

            { move || is_hovered.get().then(|| view! { cx, <button class="bg-blue-600 text-white rounded p-2">Add</button> }) }
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let todo = vec![
        Card {
            name: "Groceries".to_string(),
            description: "Buy groceries for the week and also some other goodies".to_string(),
        },
        Card {
            name: "Laundry".to_string(),
            description: "Do laundry".to_string(),
        },
        Card {
            name: "Dishes".to_string(),
            description: "Do the dishes".to_string(),
        },
    ];

    let in_progress = vec![
        Card {
            name: "Homework".to_string(),
            description: "Finish homework".to_string(),
        },
        Card {
            name: "Project".to_string(),
            description: "Work on project".to_string(),
        },
    ];

    let done = vec![
        Card {
            name: "Dinner".to_string(),
            description: "Make dinner".to_string(),
        },
        Card {
            name: "Clean".to_string(),
            description: "Clean the house".to_string(),
        },
    ];

    view! { cx,
        <div class="container mx-auto px-4 py-8">
            <div class="flex gap-4">
                <CardList
                    title="To Do".to_string()
                    cards=todo
                />
                <CardList
                    title="In Progress".to_string()
                    cards=in_progress
                />
                <CardList
                    title="Done".to_string()
                    cards=done
                />
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> });
}
