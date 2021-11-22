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

/// State of a task's blocking dependency rule
pub enum Dependency {
    /// Not blocked by anything
    Free,
    /// Unblocked only when other tasks are Dead
    Direct(Vec<Uuid>),
    /// Unblocked only when parent task is Dead
    Parent,
    /// Unblocked only when all children is Dead
    Children,
    /// Unblocked only when sibling task directly above is Dead
    Above,
    /// Unblocked only when sibling task directly below is Dead
    Below
}

/// A Task! ID and pointers to others identified by UUIDs
pub struct Task {
    /// A title for the task.
    pub title: String,
    /// A mutable pointer to a Recur by which this task subscribes to
    pub date: Box<dyn Recur>,
    /// A mutable pointer to a DependentRule
    // dependency: Dependency,
    /// Pointer to vector of immutable borrows of children UUIDs
    pub children: Vec<Uuid>,
    /// Metadata consisting of label: <serialized data> (@david in what format?)
    pub metadata: HashMap<String, String>,
    /// The dependency state of the task
    pub dependency: Dependency,
    /// ID
    id: Uuid,
}

impl Task {
    pub fn new(title: &str, date: Box<dyn Recur>) -> Task {
	Task {
	    title: String::from(title),
	    date,
	    children: Vec::new(),
	    metadata: HashMap::new(),
            dependency: Dependency::Free,
	    id: Uuid::new_v4(), 
	}
    }

    /// Returns the ID of the Task
    pub fn id(&self) -> String {
        String::from(self.id.to_string())
    }
}

