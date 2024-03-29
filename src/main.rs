use leptos::html::Div;
use leptos::*;
use leptos_use::*;
use uuid::Uuid;

mod mock_data;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum CardState {
    #[default]
    Todo,
    InProgress,
    Done,
}

#[derive(Clone, Default)]
struct Card {
    id: Uuid,
    name: String,
    description: String,
    state: CardState,
    is_editing: bool,
}

#[component]
fn CardWrapper(card: Card) -> impl IntoView {
    if card.is_editing {
        view! { <CardEdit card=card/>}
    } else {
        view! { <CardView card=card/>}
    }
}

#[component]
fn CardEdit(card: Card) -> impl IntoView {
    let name_ref = create_node_ref();
    let description_ref = create_node_ref();
    let CardsContext { cards_write, .. } = use_context::<CardsContext>().unwrap();

    view! {
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
                // Save changes button
                <button
                    class="bg-green-100 rounded p-1 mt-2"
                    on:click=move |_| {
                        update_card(cards_write, card.id, |c| {
                            c.name = name_ref().unwrap().value();
                            c.description = description_ref().unwrap().value();
                            c.is_editing = false;
                        });
                    }
                >
                    "️💾️"
                </button>
                // Discard changes button
                <button
                    class="bg-red-100 rounded p-1 mt-2"
                    on:click=move |_| {
                        update_card(cards_write, card.id, |c| {
                            c.is_editing = false;
                        });
                    }
                >
                    "️❌️"
                </button>
            </div>
        </div>
    }
}

#[component]
fn CardView(card: Card) -> impl IntoView {
    let node_ref = create_node_ref::<Div>();

    let (is_dragging, set_is_dragging) = create_signal(false);
    let CardsContext {
        cards_write,
        drop_card,
        ..
    } = use_context().unwrap();

    // Card dragging functionality using `leptos_use` crate, see docs for more info
    let UseDraggableReturn { style, .. } = use_draggable_with_options(
        node_ref,
        UseDraggableOptions::default()
            // We want to prevent dragging when clicking on "edit" and "delete" buttons
            .exact(true)
            .prevent_default(true)
            .on_move(move |_| {
                set_is_dragging(true);
            })
            .on_end(move |_| {
                drop_card(card.id);
                set_is_dragging(false);
            }),
    );

    // Set the position of the card to fixed when dragging
    let div_style = move || {
        if is_dragging.get() {
            format!("position: fixed; {}", style.get())
        } else {
            "".to_owned()
        }
    };

    // Add a shadow and pointer-events-none when dragging
    let div_class = move || {
        if is_dragging.get() {
            "bg-white rounded p-4 mb-4 shadow-lg pointer-events-none max-w-xs min-w-[20%]"
        } else {
            "bg-white rounded p-4 mb-4"
        }
    };

    view! {
        <div node_ref=node_ref class=div_class style=div_style>
            <div class="pointer-events-none">
                <h2 class="text-lg font-bold">{card.name.clone()}</h2>
                <p class="text-gray-600">{card.description.clone()}</p>
            </div>

            <div class="flex justify-end gap-2">
                // Edit card
                <button
                    class="bg-blue-100 rounded p-1 mt-2"
                    on:click=move |_| {
                        update_card(cards_write, card.id, |c| {
                            c.is_editing = true;
                        });
                    }
                >
                    "️✏️"
                </button>

                // Delete card
                <button
                    class="bg-red-100 rounded p-1 mt-2"
                    on:click=move |_| {
                        cards_write.update(move |cards| {
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
fn NewCardPlaceholder(card_state: CardState) -> impl IntoView {
    let CardsContext { cards_write, .. } = use_context::<CardsContext>().unwrap();
    view! {
        // Add new card button
        <button
            class="border border-dashed border-gray-400 rounded hover:bg-gray-200 min-w-full"
            on:click=move |_| {
                cards_write.update(move |cards| {
                    cards.push(Card {
                        id: Uuid::new_v4(),
                        state: card_state,
                        is_editing: true,
                        ..Default::default()
                    });
                });
            }
        >
            "➕"
        </button>
    }
}

#[component]
fn CardList(card_state: CardState, node_ref: NodeRef<Div>) -> impl IntoView {
    let CardsContext { cards, .. } = use_context::<CardsContext>().unwrap();

    view! {
        <div class="flex-1 max-w-sm bg-gray-100 rounded p-4" node_ref=node_ref>
            <h1 class="text-xl font-bold mb-4 text-blue-600">
                {format!("{:?}", card_state)}
            </h1>
            // Individual cards belonging to this list (filtered by state)
            {move || cards.get()
                .into_iter()
                .filter(|card| card.state == card_state)
                .map(|card| view! { <CardWrapper card/> })
                .collect_view()}

            // Card placeholder for adding new cards to this list
            <NewCardPlaceholder card_state/>
        </div>
    }
}

#[derive(Clone)]
struct CardsContext {
    cards: ReadSignal<Vec<Card>>,
    cards_write: WriteSignal<Vec<Card>>,
    drop_card: Callback<Uuid>,
}

fn update_card(cards_write: WriteSignal<Vec<Card>>, id: Uuid, f: impl Fn(&mut Card)) {
    cards_write.update(move |cards| {
        let card = cards.iter_mut().find(|card| card.id == id);
        if let Some(card) = card {
            f(card);
        }
    });
}

#[component]
fn App() -> impl IntoView {
    // Main signal containing all cards
    let (cards, cards_write) = create_signal(mock_data::get_mock_data());

    let card_lists = [CardState::Todo, CardState::InProgress, CardState::Done];

    // These node refs are needed for the drag and drop effect
    let card_list_refs = card_lists
        .iter()
        .map(|state| (*state, create_node_ref::<Div>()))
        .collect::<Vec<_>>();

    // These signals are used to check if a card is being hovered over a drop zone
    let drop_zone_signals = card_list_refs
        .iter()
        .map(|(state, node_ref)| (*state, use_element_hover(*node_ref)))
        .collect::<Vec<_>>();

    let drop_card = Callback::new(move |id: Uuid| {
        let dropped_to = drop_zone_signals
            .iter()
            .find(|(_, is_hovered)| is_hovered.get())
            .map(|(state, _)| *state);

        if let Some(new_state) = dropped_to {
            cards_write.update(move |cards| {
                let card = cards.iter_mut().find(|card| card.id == id).unwrap();
                card.state = new_state;
            });
        }
    });

    let context = CardsContext {
        cards,
        cards_write,
        drop_card,
    };

    view! {
        <Provider value={context}>
            <div class="container mx-auto px-4 py-8">
                <div class="flex gap-4">
                    {card_list_refs.into_iter()
                        .map(|(card_state, node_ref)| view! { <CardList card_state node_ref/> })
                        .collect_view()}
                </div>
            </div>
        </Provider>
    }
}

fn main() {
    mount_to_body(App);
}
