use super::model::*;
use chrono::prelude::*;
use chrono::Duration;
use uuid::Uuid;

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

/// Null date rule that does nothing and is always available
pub struct Blank {}

impl Recur for Blank {
    fn current(&self) -> Option<DateTime<Local>> {
	return None;
    }

    fn next(&mut self) -> () {}

    fn active(&self) -> RecurState {
	RecurState::Active
    }
}

/// Singleton dates (normal due + defer)
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
    pub fn new(due: DateTime<Local>) -> Self {
	Deadline {
	    due,
	    done: false
	}
    }
}

/// See documentation for all Recurs
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
pub struct Constant {
    // the "due date" of a task
    pub due: DateTime<Local>,
    // the repeat interval
    pub repeat: Duration,
    // relative to last repeat date
    // so "available" is defined as due date - defer
    pub defer: Duration, // Relative to repeat.
    // dead if due date is after this end date
    pub end_date: Option<DateTime<Local>>
}

impl Constant {
    pub fn new(
	due: DateTime<Local>,
	end_date: Option<DateTime<Local>>,
	repeat: Duration,
	defer: Duration,
    ) -> Self {
	Constant {
	    due,
	    end_date,
	    repeat,
	    defer
	}
    }
}

/// See documentation for all Recurs
impl Recur for Constant {
    fn current(&self) -> Option<DateTime<Local>> {
	// If there is an end date, return None if the end date is passed
	if let Some(end_date) = self.end_date {
	    if is_past(self.due, end_date) {
		return None;
	    }
	}

	// If not, if current time is past start time, return due date
	// else, return none
	if is_past(Local::now(), self.due - self.defer) {
	    Some(self.due)
	} else {
	    None
	}
    }

    fn next(&mut self) -> () {
	// Next date is just the increment
	self.due = self.due + self.repeat;
    }

    fn active(&self) -> RecurState {
	if let Some(end_date) = self.end_date {
	    if is_past(self.due, end_date) {
		return RecurState::Dead
	    }
	}

	if is_past(Local::now(), self.due - self.defer) {
	    RecurState::Pending(self.due - self.defer)
	} else {
	    RecurState::Active
	}
    }
}

pub struct Date {
    pub date: DateTime<Local>
}

impl Dependency for Date {
    fn available(&self) -> bool {
	is_past(self.date, Local::now())
    }
}

pub struct Direct<'a> {
    pub task: &'a Task
}

impl Direct<'_> {
    pub fn new<'a>(space: &Workspace, id: Uuid) -> Direct {
	Direct { task: &space.tasks.get(&id).unwrap() }
    }
}

impl Dependency for Direct<'_> {
    fn available(&self) -> bool {
	self.task.date.active() == RecurState::Dead
    }
}
