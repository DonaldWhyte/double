use std::cell::RefCell;
use std::sync::{Arc, Mutex};

type ThreadSafeRef<T> = Arc<Mutex<RefCell<T>>>;
type OptionalThreadSafeRef<T> = ThreadSafeRef<Option<T>>;

fn create_thread_safe_ref<T>(underlying_object: T) -> ThreadSafeRef<T> {
    Arc::new(Mutex::new(RefCell::new(underlying_object)))
}

fn use_locked_ref<T, RetType>(thread_safe_ref: &ThreadSafeRef<T>,
                              func: dyn Fn(&T) -> TReturn) -> TReturn
{
    match thread_safe_ref.lock() {
        Ok(guard) => {
            let ref cell_containing_value = *guard;
            func(&cell_containing_value.borrow())
        },
        Err(p_err) => {
            panic!(
                "Could not acquire lock on double mutex due to error: {}",
                p_err);
        }
    }
}

fn use_locked_ref_mut<T, RetType>(thread_safe_ref: &ThreadSafeRef<T>,
                                  func: dyn Fn(&mut T) -> TReturn) -> TReturn
{
    match thread_safe_ref.lock() {
        Ok(guard) => {
            let ref cell_containing_value = *guard;
            func(&mut cell_containing_value.borrow_mut())
        },
        Err(p_err) => {
            panic!(
                "Could not acquire lock on double mutex due to error: {}",
                p_err);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_thread_safe_ref() {
        let safe_ref = ThreadSafeRef<String>("");
    }
}
