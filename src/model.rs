use std::io::Stdin;
use uuid::Uuid;
use std::collections::HashMap;
use chrono::prelude::*;

/// State of the date rule, returned on active()
pub enum RecurState {
    /// Dead, will not become active again
    Dead,
    /// Pending until specific date
    Pending(DateTime<Local>),
    /// Date held manually/due to arbiturary non-date reasons
    Held,
    /// Actively running, could be completed again
    Active
}

pub trait Recur {
    /// Returns an `Option` containing potentionally
    /// the next set of due and defer dates
    ///
    /// # Examples
    ///
    /// When the rule is uncompleted/still repeating/available:
    ///
    /// ```
    /// let rule = ImplRecur::new();
    /// rule.next(); // => Some(DateTime)
    /// ```
    fn current(&self) -> Option<DateTime<Local>>;

    /// Increment the Date Rule. Similar to "completing" a task.
    fn next(&mut self) -> ();

    /// Returns a `RecurState` containing potentionally
    /// the next set of due dates.
    ///
    /// # Possible States
    /// See documentation on `RecurState`
    fn active(&self) -> RecurState;
}

/// A Task! ID and pointers to others identified by UUIDs
pub struct Task<'a> {
    /// A mutable pointer to a Recur by which this task subscribes to
    date: &'a mut dyn Recur,
    /// A mutable pointer to a DependentRule
    // dependency: Dependency,
    /// Pointer to vector of immutable borrows of children UUIDs
    children: Vec<Uuid>,
    /// Metadata consisting of label: <serialized data>
    metadata: HashMap<String, String>,
    /// ID
    id: Uuid,
}



