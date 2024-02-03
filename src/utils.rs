use crate::{Card, CardState};

pub(crate) fn get_dummy_data() -> Vec<Card> {
    vec![
        Card {
            id: 0,
            name: "Groceries".to_string(),
            description: "Buy groceries for the week and also some other goodies".to_string(),
            state: CardState::Todo,
            is_editing: false,
        },
        Card {
            id: 1,
            name: "Laundry".to_string(),
            description: "Do laundry".to_string(),
            state: CardState::Todo,
            is_editing: false,
        },
        Card {
            id: 2,
            name: "Dishes".to_string(),
            description: "Do the dishes".to_string(),
            state: CardState::Todo,
            is_editing: false,
        },
        Card {
            id: 3,
            name: "Homework".to_string(),
            description: "Finish homework".to_string(),
            state: CardState::InProgress,
            is_editing: false,
        },
        Card {
            id: 4,
            name: "Project".to_string(),
            description: "Work on project".to_string(),
            state: CardState::InProgress,
            is_editing: false,
        },
        Card {
            id: 5,
            name: "Dinner".to_string(),
            description: "Make dinner".to_string(),
            state: CardState::Done,
            is_editing: false,
        },
        Card {
            id: 6,
            name: "Clean".to_string(),
            description: "Clean the house".to_string(),
            state: CardState::Done,
            is_editing: false,
        },
    ]
}
