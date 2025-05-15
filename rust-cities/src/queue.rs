// static list of strings
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref QUEUE: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn add(val: &str) {
    let mut queue = QUEUE.lock().unwrap();
    queue.push(val.to_string());
}

pub fn get_queue() -> Vec<String> {
    let queue = QUEUE.lock().unwrap();
    queue.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear() {
        let mut queue = QUEUE.lock().unwrap();
        queue.clear();
    }

    #[test]
    fn test_add() {
        add("test");
        let queue = QUEUE.lock().unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0], "test");
        clear(); // Clear the queue after the test
    }

    #[test]
    fn test_add_multiple() {
        add("test1");
        add("test2");
        let queue = QUEUE.lock().unwrap();
        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0], "test1");
        assert_eq!(queue[1], "test2");
        clear(); // Clear the queue after the test
    }

    #[test]
    fn test_get_queue() {
        add("test1");
        add("test2");
        let queue = get_queue();
        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0], "test1");
        assert_eq!(queue[1], "test2");
        clear(); // Clear the queue after the test
    }

    #[test]
    fn test_get_queue_empty() {
        let queue = get_queue();
        assert_eq!(queue.len(), 0);
        clear(); // Clear the queue after the test
    }
}
