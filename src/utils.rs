use uuid::Uuid;

use crate::{Card, CardState};

pub(crate) fn get_dummy_data() -> Vec<Card> {
    vec![
        Card {
            id: Uuid::new_v4(),
            name: "Groceries".to_string(),
            description: "Buy groceries for the week and also some other goodies".to_string(),
            state: CardState::Todo,
            is_editing: false,
        },
        Card {
            id: Uuid::new_v4(),
            name: "Laundry".to_string(),
            description: "Do laundry".to_string(),
            state: CardState::Todo,
            is_editing: false,
        },
        Card {
            id: Uuid::new_v4(),
            name: "Dishes".to_string(),
            description: "Do the dishes".to_string(),
            state: CardState::Todo,
            is_editing: false,
        },
        Card {
            id: Uuid::new_v4(),
            name: "Homework".to_string(),
            description: "Finish homework".to_string(),
            state: CardState::InProgress,
            is_editing: false,
        },
        Card {
            id: Uuid::new_v4(),
            name: "Project".to_string(),
            description: "Work on project".to_string(),
            state: CardState::InProgress,
            is_editing: false,
        },
        Card {
            id: Uuid::new_v4(),
            name: "Dinner".to_string(),
            description: "Make dinner".to_string(),
            state: CardState::Done,
            is_editing: false,
        },
        Card {
            id: Uuid::new_v4(),
            name: "Clean".to_string(),
            description: "Clean the house".to_string(),
            state: CardState::Done,
            is_editing: false,
        },
    ]
}
