use super::model::*;
use chrono::prelude::*;
use chrono::Duration;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Check if timestamp a is past timestamp b.
/// i.e. that timestamp "a" at a later date than "b"
///
/// # Arguments
/// - `a`: the time that should be "later" DateTime<Local>
/// - `b`: the time that sohuld be "earlier" DateTime<Local>
fn is_past(a: DateTime<Local>, b: DateTime<Local>) -> bool {
    if a.signed_duration_since(b) > Duration::seconds(0) {
	return true;
    } else {
	return false;
    }
}

/// Null date rule that does nothing special and is always available
#[derive(Serialize, Deserialize)]
pub struct Blank {done: bool}
impl Blank { pub fn new() -> Box<Blank> { Box::new(Blank{done: false}) } }
#[typetag::serde]
impl Recur for Blank {
    fn current(&self) -> Option<DateTime<Local>> {
	return None;
    }

    fn next(&mut self) -> () {
	self.done = true;
    }

    fn active(&self) -> RecurState {
	if self.done {RecurState::Dead} else {RecurState::Active}
    }
}

/// Singleton dates
#[derive(Serialize, Deserialize)]
pub struct Deadline {
    // The "due date" of the rule
    pub due: DateTime<Local>,
    // Whether or not the date is done
    done: bool
}

impl Deadline {
    /// Create a new Singnleton Rule given due/defer dates
    ///
    /// # Arguments
    /// - `due`: due date, unix time, encoded in DateTime<Local>
    pub fn new(due: DateTime<Local>) -> Box<Deadline> {
	Box::new(Deadline {
	    due,
	    done: false
	})
    }
}

/// See documentation for all Recurs
#[typetag::serde]
impl Recur for Deadline {
    fn current(&self) -> Option<DateTime<Local>> {
	if !self.done {
	    Some(self.due)
	} else {
	    None
	}
    }

    fn next(&mut self) -> () {
	self.done = true;
    }

    fn active(&self) -> RecurState {
	if self.done {
	    return RecurState::Dead;
	} else {
	    return RecurState::Active;
	}
    }
}

/// Due date that repeats every X time interval consistently
/// i.e. 1 second/minute/day/(exactly one) month/year etc.
#[derive(Serialize, Deserialize)]
pub struct Constant {
    // the "due date" of a task
    pub due: DateTime<Local>,
    // the repeat interval
    pub repeat: std::time::Duration,
    // dead if due date is after this end date
    pub end_date: Option<DateTime<Local>>
}

impl Constant {
    pub fn new(
	due: DateTime<Local>,
	end_date: Option<DateTime<Local>>,
	repeat: Duration,
    ) -> Box<Constant> {
	Box::new(Constant {
	    due,
	    end_date,
	    repeat: repeat.to_std().unwrap(), // TODO Remove the unwrap.
	})
    }
}

/// See documentation for all Recurs
#[typetag::serde]
impl Recur for Constant {
    fn current(&self) -> Option<DateTime<Local>> {
	// If there is an end date, return None if the end date is passed
	if let Some(end_date) = self.end_date {
	    if is_past(self.due, end_date) {
		return None;
	    }
	}
	Some(self.due)	
    }

    fn next(&mut self) -> () {
	// Next date is just the increment
	self.due = self.due + Duration::from_std(self.repeat).unwrap(); // TODO handle errors.
    }

    fn active(&self) -> RecurState {
	if let Some(end_date) = self.end_date {
	    if is_past(self.due, end_date) {
		return RecurState::Dead
	    }
	}
	
	RecurState::Active
    }
}

#[derive(Serialize, Deserialize)]
pub struct Date { pub date: DateTime<Local> }
impl Date { pub fn new(date: DateTime<Local>) -> Box<Date> { Box::new(Date { date }) } }
#[typetag::serde]
impl Dependency for Date {
    fn available(&self, _space: &Workspace, _task: (Uuid, &Task)) -> Result<bool, TaskError> {	
	Ok(is_past(Local::now(), self.date))
    }
}

#[derive(Serialize, Deserialize)]
pub struct RelativeDate { pub off: std::time::Duration }
impl RelativeDate { pub fn new(off: Duration) -> Box<RelativeDate> { Box::new(RelativeDate { off: off.to_std().unwrap() }) } } // TODO handle error
#[typetag::serde]
impl Dependency for RelativeDate {
    fn available(&self, _space: &Workspace, task: (Uuid, &Task)) -> Result<bool, TaskError> {
	let date = task.1.date.current().ok_or(TaskError::NonexistentError)?; // FIXME sketchy
	Ok(is_past(Local::now(), date - Duration::from_std(self.off).unwrap())) // TODO handle error
    }
}

#[derive(Serialize, Deserialize)]
pub struct Direct { pub id: Uuid }
impl Direct { pub fn new(id: Uuid) -> Box<Direct> { Box::new(Direct { id }) } }
#[typetag::serde]
impl Dependency for Direct {
    fn available(&self, space: &Workspace, _task: (Uuid, &Task)) -> Result<bool, TaskError> {
	space.task_done(self.id)	
    }
}

#[derive(Serialize, Deserialize)]
pub struct Children {}
impl Children { pub fn new() -> Box<Children> { Box::new(Children{}) } }
#[typetag::serde]
impl Dependency for Children {
    fn available(&self, space: &Workspace, task: (Uuid, &Task)) -> Result<bool, TaskError> {
	for i in &task.1.children {
	    let current_task = space.tasks.get(i).ok_or(TaskError::UnreachableError)?;
	    if current_task.date.active() == RecurState::Active {
		return Ok(false);
	    }
	}
	Ok(true)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Parent {}
impl Parent { pub fn new() -> Box<Parent> { Box::new(Parent{}) } }
#[typetag::serde]
impl Dependency for Parent {
    fn available(&self, space: &Workspace, task: (Uuid, &Task)) -> Result<bool, TaskError> {
	let parent = space.task_get_parent(task.0)?;
	let out = space.task_done(parent)?;
	Ok(out)
    }
}


