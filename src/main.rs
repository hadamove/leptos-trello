use leptos::html::Div;
use leptos::*;
use leptos_use::*;

mod utils;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum CardState {
    #[default]
    Todo,
    InProgress,
    Done,
}

#[derive(Clone, Default)]
struct Card {
    id: usize,
    name: String,
    description: String,
    state: CardState,
}

#[component]
fn CardWrapper(cx: Scope, card: Card) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(cx, false);
    // This is a dirty workaround, do not do this in production
    let is_new = card.description.is_empty() && card.name.is_empty();

    view! { cx,
        {move || match is_editing.get() || is_new {
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
                // Save changes button
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
                // Discard changes button
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
    let node_ref = create_node_ref::<Div>(cx);
    let (is_dragging, set_is_dragging) = create_signal(cx, false);
    let DragAndDropContext { set_dropped_card } =
        use_context(cx).expect("HoveringOverContext not found");

    // Card dragging functionality using `leptos_use` crate, see docs for more info
    let UseDraggableReturn { style, .. } = use_draggable_with_options(
        cx,
        node_ref,
        UseDraggableOptions::default()
            // We want to prevent dragging when clicking on "edit" and "delete" buttons
            .exact(true)
            .prevent_default(true)
            .on_move(move |_| {
                set_is_dragging(true);
            })
            .on_end(move |_| {
                set_dropped_card(Some(card.id));
                set_is_dragging(false);
            }),
    );

    let CardsContext { set_cards, .. } = use_context::<CardsContext>(cx).expect("Cards not found");

    let div_class = move || {
        format!(
            "bg-white rounded p-4 mb-4 {}",
            match is_dragging.get() {
                true => "shadow-lg pointer-events-none max-w-xs min-w-[20%]",
                false => "",
            }
        )
    };

    // Set the position of the card to fixed when dragging
    let div_style = move || match is_dragging.get() {
        true => format!("position: fixed; {}", style.get()),
        false => "".to_owned(),
    };

    view! { cx,
        <div node_ref=node_ref class=div_class style=div_style>
            <div class="pointer-events-none">
                <h2 class="text-lg font-bold">{card.name.clone()}</h2>
                <p class="text-gray-600">{card.description.clone()}</p>
            </div>

            <div class="flex justify-end gap-2">
                // Edit card
                <button
                    class="bg-blue-100 rounded p-1 mt-2"
                    on:click=move |_| set_is_editing(true)
                >
                    "️✏️"
                </button>

                // Delete card
                <button
                    class="bg-red-100 rounded p-1 mt-2"
                    on:click=move |_| {
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
fn NewCardPlaceholder(cx: Scope, card_state: CardState) -> impl IntoView {
    let CardsContext { set_cards, .. } = use_context::<CardsContext>(cx).expect("Cards not found");
    view! { cx,
        // Add new card button
        <button
            class="border border-dashed border-gray-400 rounded hover:bg-gray-200 min-w-full"
            on:click=move |_| {
                set_cards.update(move |cards| {
                    // Find the highest id and add 1 to it, do not do this in production
                    let next_id = cards.iter().map(|card| card.id).max().unwrap_or(0) + 1;
                    cards.push(Card {
                        id: next_id,
                        state: card_state,
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
fn CardList(cx: Scope, card_state: CardState, node_ref: NodeRef<Div>) -> impl IntoView {
    let CardsContext { cards, .. } = use_context::<CardsContext>(cx).expect("Cards not found");

    view! { cx,
        <div class="flex-1 max-w-sm bg-gray-100 rounded p-4" node_ref=node_ref>
            <h1 class="text-xl font-bold mb-4 text-blue-600">
                {format!("{:?}", card_state)}
            </h1>
            // Individual cards belonging to this list (filtered by state)
            {move || cards.get()
                .into_iter()
                .filter(|card| card.state == card_state)
                .map(|card| view! { cx, <CardWrapper card/> })
                .collect_view(cx)}

            // Card placeholder for adding new cards to this list
            <NewCardPlaceholder card_state/>
        </div>
    }
}

#[derive(Clone)]
struct CardsContext {
    cards: ReadSignal<Vec<Card>>,
    set_cards: WriteSignal<Vec<Card>>,
}

#[derive(Clone)]
struct DragAndDropContext {
    set_dropped_card: WriteSignal<Option<usize>>,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // Main signal containing all cards
    let (cards, set_cards) = create_signal(cx, utils::get_dummy_data());
    // Signal for the card that is currently being dropped to a new list
    let (dropped_card, set_dropped_card) = create_signal(cx, Option::<usize>::None);

    provide_context(cx, CardsContext { cards, set_cards });
    provide_context(cx, DragAndDropContext { set_dropped_card });

    // Each list has a state (Todo, InProgress, Done)
    let card_lists = [CardState::Todo, CardState::InProgress, CardState::Done];

    // Node refs for each card list to get the hover state
    let card_list_refs = card_lists
        .iter()
        .map(|state| (*state, create_node_ref::<Div>(cx)))
        .collect::<Vec<_>>();

    // Hover state for each card list
    let card_list_hover = card_list_refs
        .iter()
        .map(|(state, node_ref)| (*state, use_element_hover(cx, *node_ref)))
        .collect::<Vec<_>>();

    // Drag and drop effect
    create_effect(cx, move |_| {
        if let Some(dropped_card) = dropped_card.get() {
            // Find the list that the card was dropped to
            let dropped_to = card_list_hover
                .iter()
                .find(|(_, is_hovered)| is_hovered.get())
                .map(|(state, _)| *state);

            if let Some(new_state) = dropped_to {
                // Update the card state
                set_cards.update(move |cards| {
                    let card = cards
                        .iter_mut()
                        .find(|card| card.id == dropped_card)
                        .unwrap();

                    card.state = new_state;
                });
            }
            set_dropped_card(None);
        }
    });

    view! { cx,
        <div class="container mx-auto px-4 py-8">
            <div class="flex gap-4">
                // Card lists
                {card_list_refs.into_iter()
                    .map(|(card_state, node_ref)| view! { cx, <CardList card_state node_ref/> })
                    .collect_view(cx)}
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> });
}
