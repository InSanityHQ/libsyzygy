use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use chrono::prelude::*;

/// State of the date rule, returned on active()
#[derive(PartialEq, Debug)]
pub enum RecurState {
    /// Dead, will not become active again
    Dead,
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
    fn available(&self, space: &Workspace, task: (Uuid, &Task)) -> Result<bool, TaskError>;
}

/// A Task! ID and pointers to others identified by UUIDs
pub struct Task {
    /// A title for the task.
    pub title: String,
    /// A mutable pointer to a Recur by which this task subscribes to
    pub date: Box<dyn Recur>,
    /// A vector of mutable pointers to a Dependency
    pub dependencies: Vec<Box<dyn Dependency>>,
    /// Pointer to vector of immutable borrows of children UUIDs
    pub children: Vec<Uuid>,
    /// Metadata usable in any form
    pub metadata: HashMap<String, String>,
}

pub struct Workspace {
    pub tasks: HashMap<Uuid, Task>,
}

// How does Rust error handling work!?
#[derive(Debug)]
pub enum TaskError {
    NonexistentError,
    NonexistentKeyError,
    DuplicateError,
    UnreachableError,
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    TaskError::DuplicateError => write!(f, "Task already inserted"),
	    TaskError::NonexistentError => write!(f, "Task doesn't exist"),
	    TaskError::NonexistentKeyError => write!(f, "Metadata key doesn't exist"),
	    TaskError::UnreachableError => write!(f, "You've somehow reached an unreachable state. Congrats?"),
	}
    }
}

impl std::error::Error for TaskError {}

impl Workspace {
    pub fn new() -> Workspace {
	Workspace {tasks: HashMap::new()}
    }

    pub fn add_task(&mut self, title: &str, date: Box<dyn Recur>, deps: Vec<Box<dyn Dependency>>) -> Uuid {
	let id = Uuid::new_v4();
	self.tasks.insert(id, Task {
	    title: String::from(title),
	    date,
	    children: Vec::new(),
	    metadata: HashMap::new(),
	    dependencies: deps,
	});
	id
    }

    pub fn task_available(&self, id: Uuid) -> Result<bool, TaskError> {
	let task = self.tasks.get(&id).ok_or(TaskError::NonexistentError)?;
	for dependency in &task.dependencies {
	    if !dependency.available(&self, (id, task))? {
		return Ok(false);
	    }
	}
	Ok(true)
    }

    pub fn task_done(&self, id: Uuid) -> Result<bool, TaskError> {
	let task = self.tasks.get(&id).ok_or(TaskError::NonexistentError)?;
	Ok(task.date.active() == RecurState::Dead)
    }
    
    pub fn task_complete(&mut self, id: Uuid) -> Result<(), TaskError> {
	let task = self.tasks.get_mut(&id).ok_or(TaskError::NonexistentError)?;
	task.date.next();
	Ok(())
    }

    pub fn task_add_child(&mut self, parent_id: Uuid, child_id: Uuid) -> Result<(), TaskError> {
	self.tasks.get(&child_id).ok_or(TaskError::NonexistentError)?;
	let task = self.tasks.get_mut(&parent_id).ok_or(TaskError::NonexistentError)?;
	for child in &task.children {
	    if *child == child_id { return Err(TaskError::DuplicateError) }
	}
	task.children.push(child_id);
	Ok(())
    }

    pub fn task_add_metadata(&mut self, id: Uuid, key: String, val: String) -> Result<(), TaskError> {
	let task = self.tasks.get_mut(&id).ok_or(TaskError::NonexistentError)?;
	task.metadata.insert(key, val);
	Ok(())
    }

    pub fn task_get_metadata(&mut self, id: Uuid, key: String) -> Result<Option<&String>, TaskError> {
	let task = self.tasks.get_mut(&id).ok_or(TaskError::NonexistentError)?;
	Ok(task.metadata.get(&key))
    }

    pub fn task_set_metadata(&mut self, id: Uuid, key: String, val: String) -> Result<(), TaskError> {
	let task = self.tasks.get_mut(&id).ok_or(TaskError::NonexistentError)?;
	let metadata = task.metadata.get_mut(&key).ok_or(TaskError::NonexistentKeyError)?;
	*metadata = val;
	Ok(())
    }

    pub fn task_get_parent(&self, id: Uuid) -> Result<Uuid, TaskError> {
	let mut stack: VecDeque<Uuid> = VecDeque::new();
	for task in self.tasks.iter() {
	    for i in &task.1.children {
		if *i == id { return Ok(*task.0); } 			
		stack.push_front(*i);
	    }
	}
	while !stack.is_empty() {
	    let current_id = stack.pop_front().ok_or(TaskError::UnreachableError)?;
	    let current_task = self.tasks.get(&current_id).ok_or(TaskError::NonexistentError)?;
	    for i in &current_task.children {
		if *i == id { return Ok(current_id); } 			
		stack.push_front(*i);
	    }	    
	}
	Err(TaskError::NonexistentError)
    }
}
