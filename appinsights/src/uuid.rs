pub use imp::*;
pub use uuid::Uuid;

#[cfg(not(test))]
mod imp {
    use uuid::Uuid;

    /// Generates a new instance of unique identifier.
    pub fn new() -> Uuid {
        Uuid::new_v4()
    }
}

#[cfg(test)]
mod imp {
    use std::cell::RefCell;

    use uuid::Uuid;

    thread_local!(static ID: RefCell<Option<Uuid>> = RefCell::new(None));

    /// Generates a new instance of unique identifier or predefined value to test against it.
    pub fn new() -> Uuid {
        ID.with(|is| {
            if let Some(id) = *is.borrow() {
                id
            } else {
                Uuid::new_v4()
            }
        })
    }

    /// Sets known Uuid value as now to assert test against it.
    pub fn set(uuid: Uuid) {
        ID.with(|is| *is.borrow_mut() = Some(uuid))
    }

    /// Resets pre-defined Uuid value to use Uuid::new_v4() instead.
    pub fn reset() {
        ID.with(|is| *is.borrow_mut() = None)
    }
}
