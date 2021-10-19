use super::model::*;
use chrono::prelude::*;
use chrono::Duration;

// Check if timestamp a is past timestamp b.
fn is_past(a: DateTime<Local>, b: DateTime<Local>) -> bool {
    if a.signed_duration_since(b) > Duration::seconds(0) {
	return true;
    } else {
	return false;
    }
}

/// Singleton dates (normal due + defer)
pub struct Deadline {
    /// The "due date" of the rule
    due: DateTime<Local>,
    /// The "defer date" of the rule
    defer: DateTime<Local>,
    /// Whether or not the date is done
    done: bool
}

impl Deadline {
    /// Create a new Singnleton Rule given due/defer dates
    ///
    /// # Arguments
    /// - `due`: due date, unix time, encoded in DateTime<Local>
    /// - `defer`: defer date, unix time, encoded in DateTime<Local>
    pub fn new(due: DateTime<Local>, defer: DateTime<Local>) -> Self {
        Deadline {
            due: due,
            defer: defer,
            done: false
        }
    }
}

/// See documentation for all Recurs
impl Recur for Deadline {
    fn current(&self) -> Option<DateTime<Local>> {
        if !self.done && is_past(Local::now(), self.defer) {
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
        } else if is_past(Local::now(), self.defer) {
	    return RecurState::Pending(self.defer);
	}	
        return RecurState::Active;
    }
}

pub struct Until {
    due: DateTime<Local>,
    repeat: Duration,
    defer: Duration, // Relative to repeat.
    end_date: DateTime<Local>,
    done: bool,
}

impl Until {
    pub fn new(
	due: DateTime<Local>,
	end_date: DateTime<Local>,
	repeat: Duration,
	defer: Duration,
    ) -> Self {
        Until {
            due,
            end_date,
	    repeat,
	    defer,
            done: false,
        }
    }
}

impl Recur for Until {
    fn current(&self) -> Option<DateTime<Local>> {
        if !self.done && is_past(Local::now(), self.due - self.defer) {
	    Some(self.due)
	} else {
	    None
	}
    }

    fn next(&mut self) -> () {
	if is_past(Local::now(), self.end_date) {
            self.done = true;
	} else {
	    self.due = self.due + self.repeat;
	}
    }

    fn active(&self) -> RecurState {
        if self.done {
            return RecurState::Dead;
        } else if is_past(Local::now(), self.due - self.defer) {
	    return RecurState::Pending(self.due - self.defer);
	}	
        return RecurState::Active;
    }
}
