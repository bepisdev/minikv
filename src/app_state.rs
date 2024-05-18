use actix_web::web;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub store: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> web::Data<Self> {
        let state = web::Data::new(AppState {
            store: Mutex::new(HashMap::new()),
        });
        return state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        let store = state.store.lock().unwrap();
        assert!(store.is_empty(), "Initial store should be empty");
    }

    #[test]
    fn test_insert_into_store() {
        let state = AppState::new();
        {
            let mut store = state.store.lock().unwrap();
            store.insert("key".to_string(), "value".to_string());
        }

        {
            let store = state.store.lock().unwrap();
            assert_eq!(
                store.get("key"),
                Some(&"value".to_string()),
                "Value should be 'value'"
            );
        }
    }

    #[test]
    fn test_update_store() {
        let state = AppState::new();
        {
            let mut store = state.store.lock().unwrap();
            store.insert("key".to_string(), "value".to_string());
        }

        {
            let mut store = state.store.lock().unwrap();
            store.insert("key".to_string(), "new_value".to_string());
        }

        {
            let store = state.store.lock().unwrap();
            assert_eq!(
                store.get("key"),
                Some(&"new_value".to_string()),
                "Value should be 'new_value'"
            );
        }
    }

    #[test]
    fn test_remove_from_store() {
        let state = AppState::new();
        {
            let mut store = state.store.lock().unwrap();
            store.insert("key".to_string(), "value".to_string());
        }

        {
            let mut store = state.store.lock().unwrap();
            store.remove("key");
        }

        {
            let store = state.store.lock().unwrap();
            assert!(store.get("key").is_none(), "Key should be removed");
        }
    }
}
