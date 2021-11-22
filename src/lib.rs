mod model;
pub mod rules;

pub use model::Task;
pub use model::Recur;
pub use model::RecurState;
pub use rules::Deadline;
pub use rules::Until;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Duration;
    #[test]
    fn test_defer() {	
	let now = Local::now();
        let mut t: Task = Task::new(
	    "hewooo",
	    Box::new(Deadline::new(
		now + Duration::days(3),
		now + Duration::days(1),
	    )),
	);
	assert_eq!(t.date.current(), None);
	assert_eq!(t.date.active(), RecurState::Pending(now + Duration::days(1)));
	t.date.next();
	assert_eq!(t.date.active(), RecurState::Dead);
    }
}
