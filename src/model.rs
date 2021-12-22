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

pub trait Dependency {
    /// Returns a boolean representing if the task is available.
    fn available(&self, space: &Workspace) -> bool;
}

/// A Task! ID and pointers to others identified by UUIDs
pub struct Task {
    /// A title for the task.
    pub title: String,
    /// A mutable pointer to a Recur by which this task subscribes to
    pub date: Box<dyn Recur>,
    /// A mutable pointer to a Dependency
    pub dependency: Option<Box<dyn Dependency>>,
    /// Pointer to vector of immutable borrows of children UUIDs
    pub children: Vec<Uuid>,
    /// Metadata usable in any form
    pub metadata: HashMap<String, String>,
}

pub struct Workspace {
    
    pub tasks: HashMap<Uuid, Task>,
}

impl Workspace {
    pub fn new() -> Workspace {
	Workspace {tasks: HashMap::new()}
    }
    
    pub fn add_task(&mut self, title: &str, date: Box<dyn Recur>, dep: Option<Box<dyn Dependency>>) -> Uuid{
	let id = Uuid::new_v4();
	self.tasks.insert(id,  Task {
	    title: String::from(title),
	    date,
	    children: Vec::new(),
	    metadata: HashMap::new(),
            dependency: dep,
	});
	id
    }

    pub fn task_available(&self, id: Uuid) -> bool {
	self.tasks.get(&id).unwrap().dependency.as_ref().unwrap().available(&self)
    }

    pub fn task_complete(&mut self, id: Uuid) {
	self.tasks.get_mut(&id).unwrap().date.next();
    }
}

