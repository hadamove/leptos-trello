use leptos::*;

struct Card {
    name: String,
    description: String,
}

#[component]
fn CardView(cx: Scope, card: Card) -> impl IntoView {
    view! { cx,
        <div class="bg-white rounded p-4 mb-4">
            <h2 class="text-lg font-bold">{card.name}</h2>
            <p class="text-gray-600">{card.description}</p>
        </div>
    }
}

#[component]
fn CardList(cx: Scope, title: String, cards: Vec<Card>) -> impl IntoView {
    view! { cx,
        <div class="flex-1 max-w-sm bg-gray-100 rounded p-4">
            <h1 class="text-xl font-bold mb-4 text-blue-600">{title}</h1>
            {cards
                .into_iter()
                .map(|card| view! { cx, <CardView card/> })
                .collect_view(cx)}
        </div>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
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
