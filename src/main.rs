use leptos::html::Div;
use leptos::leptos_dom::console_log;
use leptos::*;
use leptos_use::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CardState {
    Todo,
    InProgress,
    Done,
}

#[derive(Clone)]
struct Card {
    id: usize,
    name: String,
    description: String,
    state: CardState,
}

#[component]
fn CardWrapper(cx: Scope, card: Card) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(cx, false);

    view! { cx,
        {move || match is_editing.get() {
            true => view! {cx, <CardEdit card=card.clone() set_is_editing/>},
            false => view! {cx, <CardView card=card.clone() set_is_editing/>}
        }}
    }
}

#[component]
fn CardEdit(cx: Scope, card: Card, set_is_editing: WriteSignal<bool>) -> impl IntoView {
    let name_ref = create_node_ref(cx);
    let description_ref = create_node_ref(cx);
    let CardsContext { set_cards, .. } = use_context::<CardsContext>(cx).expect("Cards not found");

    view! { cx,
        <div class="flex flex-col gap-2 bg-white rounded p-4 mb-4">

            <input
                class="border rounded px-2 text-lg font-bold"
                value=card.name
                node_ref=name_ref
                size=1
            />
            <textarea
                class="border rounded px-2 text-grey-600"
                node_ref=description_ref
                cols=1
            >
                {card.description}
            </textarea>
            <div class="flex justify-end gap-2">
                // Save
                <button
                    class="bg-green-100 rounded p-1 mt-2"
                    on:click=move |_| {
                        let name = name_ref().unwrap().value();
                        let description = description_ref().unwrap().value();

                        let changed_card = Card {
                            id: card.id,
                            name,
                            description,
                            state: card.state,
                        };

                        set_cards.update(move |cards| {
                            let card = cards.iter_mut().find(|card| card.id == changed_card.id).unwrap();
                            *card = changed_card;
                        });

                        set_is_editing(false);
                    }
                >
                    "️💾️"
                </button>
                // Discard
                <button
                    class="bg-red-100 rounded p-1 mt-2"
                    on:click=move |_| set_is_editing(false)
                >
                    "️❌️"
                </button>
            </div>
        </div>
    }
}

#[component]
fn CardView(cx: Scope, card: Card, set_is_editing: WriteSignal<bool>) -> impl IntoView {
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

    let CardsContext { set_cards, .. } = use_context::<CardsContext>(cx).expect("Cards not found");

    view! { cx,
        <div
            node_ref=el
            class=move || format!("bg-white rounded p-4 mb-4 {}",
                if is_dragging.get() { "shadow-lg pointer-events-none max-w-xs min-w-[20%]" } else { "" }
            )
            style=move || if is_dragging.get() { format!("position: fixed; {}", style.get()) } else {"".to_owned()}
        >
            <h2 class="text-lg font-bold">{card.name.clone()}</h2>
            <p class="text-gray-600">{card.description.clone()}</p>

            <div class="flex justify-end gap-2">
                // Edit
                <button
                    class="bg-blue-100 rounded p-1 mt-2"
                    on:click=move |_| set_is_editing(true)
                >
                    "️✏️"
                </button>

                // Delete
                <button
                    class="bg-red-100 rounded p-1 mt-2"
                    on:click=move |_| {
                        console_log("Deleting card");
                        set_cards.update(move |cards| {
                            cards.retain(|c| c.id != card.id);
                        });
                    }
                >
                    "️🔥"
                </button>
            </div>
        </div>
    }
}

#[component]
fn CardList(cx: Scope, card_state: CardState) -> impl IntoView {
    let el = create_node_ref::<Div>(cx);
    let is_hovered = use_element_hover(cx, el);
    let CardsContext { cards, .. } = use_context::<CardsContext>(cx).expect("Cards not found");

    view! { cx,
        <div class="flex-1 max-w-sm bg-gray-100 rounded p-4" node_ref=el>
            <h1 class="text-xl font-bold mb-4 text-blue-600">
                {format!("{:?}", card_state)}
            </h1>
            {move || cards.get()
                .into_iter()
                .filter(|card| card.state == card_state)
                .map(|card| view! { cx, <CardWrapper card/> })
                .collect_view(cx)}

            { move || is_hovered.get().then(|| view! { cx, <button class="bg-blue-600 text-white rounded p-2">Add</button> }) }
        </div>
    }
}

#[derive(Clone)]
struct CardsContext {
    cards: ReadSignal<Vec<Card>>,
    set_cards: WriteSignal<Vec<Card>>,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (cards, set_cards) = create_signal(
        cx,
        vec![
            Card {
                id: 0,
                name: "Groceries".to_string(),
                description: "Buy groceries for the week and also some other goodies".to_string(),
                state: CardState::Todo,
            },
            Card {
                id: 1,
                name: "Laundry".to_string(),
                description: "Do laundry".to_string(),
                state: CardState::Todo,
            },
            Card {
                id: 2,
                name: "Dishes".to_string(),
                description: "Do the dishes".to_string(),
                state: CardState::Todo,
            },
            Card {
                id: 3,
                name: "Homework".to_string(),
                description: "Finish homework".to_string(),
                state: CardState::InProgress,
            },
            Card {
                id: 4,
                name: "Project".to_string(),
                description: "Work on project".to_string(),
                state: CardState::InProgress,
            },
            Card {
                id: 5,
                name: "Dinner".to_string(),
                description: "Make dinner".to_string(),
                state: CardState::Done,
            },
            Card {
                id: 6,
                name: "Clean".to_string(),
                description: "Clean the house".to_string(),
                state: CardState::Done,
            },
        ],
    );
    provide_context(cx, CardsContext { cards, set_cards });

    // let add_card = move |card: Card| {
    //     let mut cards = cards.get();
    //     cards.push(card);
    //     set_cards(cards);
    // };

    // let update_card = move |id: usize, changed_card: Card| {
    //     let mut cards = cards.get();
    //     let card = cards.iter_mut().find(|card| card.id == id).unwrap();
    //     *card = changed_card;
    //     set_cards(cards);
    // };

    view! { cx,
        <div class="container mx-auto px-4 py-8">
            <div class="flex gap-4">
                {[CardState::Todo, CardState::InProgress, CardState::Done].into_iter()
                    .map(|state| view! { cx, <CardList card_state=state/> })
                    .collect_view(cx)}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> });
}
