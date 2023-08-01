use leptos::*;

struct Card {
    name: String,
    description: String,
}

#[component]
fn CardView(cx: Scope, card: Card) -> impl IntoView {
    view! { cx,
        <div
            style="border: 1px solid #000; padding: 10px; margin: 10px;"
        >
            <h2>{card.name}</h2>
            <p>{card.description}</p>
        </div>
    }
}

#[component]
fn CardList(cx: Scope, title: String, cards: Vec<Card>) -> impl IntoView {
    view! { cx,
        <div
            style="display: flex; flex-direction: row; flex-wrap: wrap;"
        >
            <h1>{title}</h1>
            {cards
                .into_iter()
                .map(|card| view! { cx, <CardView card/> })
                .collect_view(cx)}
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let todo = vec![
        Card {
            name: "Groceries".to_string(),
            description: "Buy groceries for the week".to_string(),
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
        <div>
            <button
                on:click=move |_| {
                    set_count.update(|count| *count += 1)
                }
                style="background-color: #f00; color: #fff;"
            >
                "Click me: " {count}
            </button>
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
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> });
}
