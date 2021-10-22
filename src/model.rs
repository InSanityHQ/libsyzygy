use std::io::Stdin;
use uuid::Uuid;
use std::collections::{HashMap};
use chrono::prelude::*;

/// State of the date rule, returned on active()
#[derive(PartialEq, Debug)]
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
    fn current(&self) -> Option<DateTime<Local>>;

    /**
    Increment the Date Rule. Similar to "completing" a task.
    */
    fn next(&mut self) -> ();

    /// Returns a `RecurState` containing potentionally
    /// the next set of due dates.
    ///
    /// # Possible States
    /// See documentation on `RecurState`
    fn active(&self) -> RecurState;
}

/// A Task! ID and pointers to others identified by UUIDs
pub struct Task {
    /// A title for the task.
    pub title: String,
    /// A mutable pointer to a Recur by which this task subscribes to
    pub date: Box<Recur>,
    /// A mutable pointer to a DependentRule
    // dependency: Dependency,
    /// Pointer to vector of immutable borrows of children UUIDs
    pub children: Vec<Uuid>,
    /// Metadata consisting of label: <serialized data>
    pub metadata: HashMap<String, String>,
    /// ID
    id: Uuid,
}

impl Task {
    pub fn new(title: String, date: Box<dyn Recur>) -> Task {
	Task {
	    title,
	    date,
	    children: Vec::new(),
	    metadata: HashMap::new(),
	    id: Uuid::new_v4(), 
	}
    }
}



