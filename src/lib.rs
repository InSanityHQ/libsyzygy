mod model;
pub mod rules;

pub use model::Task;
pub use model::Workspace;
pub use model::Recur;
pub use model::RecurState;
pub use rules::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use chrono::Duration;    

    #[test]
    fn task_sanity() {
	let mut w = Workspace::new();
	let id = w.add_task(
	    "Test",
	    Blank::new(),
	    Vec::new()
	);
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Active);
	w.task_complete(id).unwrap();
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.title, "Test");	
	assert_eq!(task.date.active(), RecurState::Dead);
    }    
    
    #[test]
    fn deadline_recur() {
	let now = Local::now();
	let mut w = Workspace::new();
	let id = w.add_task(
	    "Test",
	    Deadline::new(now + Duration::days(4)),
	    Vec::new()
	);
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Active);
	assert_eq!(task.date.current().unwrap(), now + Duration::days(4));	
	w.task_complete(id).unwrap();
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Dead);
    }

    #[test]
    fn constant_recur() {
	let now = Local::now();
	let mut w = Workspace::new();
	let id = w.add_task(
	    "Test",
	    Constant::new(
		now + Duration::days(2),
		Some(now + Duration::days(4)),
		Duration::days(1),
	    ),
	    Vec::new()
	);
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Active);
	w.task_complete(id).unwrap();
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Active);
	assert_eq!(task.date.current().unwrap(), now + Duration::days(3));
	w.task_complete(id).unwrap();
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Active);
	assert_eq!(task.date.current().unwrap(), now + Duration::days(4));
	w.task_complete(id).unwrap();
	let task = w.tasks.get(&id).unwrap();
	assert_eq!(task.date.active(), RecurState::Dead);
    }

    #[test]
    fn date_dep() {	
	let now = Local::now();
	let mut w = Workspace::new();
        let id1 = w.add_task(
	    "TestTask",
	    Blank::new(),
	    vec![Date::new(now + Duration::days(4))],
	);
	assert_eq!(w.task_available(id1).unwrap(), false);
	let id2 = w.add_task(
	    "TestTask2",
	    Blank::new(),
	    vec![Date::new(now - Duration::days(4))],
	);
	assert_eq!(w.task_available(id2).unwrap(), true);
    }

    #[test]
    fn reldate_dep() {	
	let now = Local::now();
	let mut w = Workspace::new();
        let id1 = w.add_task(
	    "TestTask",
	    Deadline::new(now + Duration::days(4)),
	    vec![RelativeDate::new(Duration::days(2))]
	);
	assert_eq!(w.task_available(id1).unwrap(), false);
	let id2 = w.add_task(
	    "TestTask2",
	    Deadline::new(now + Duration::days(4)),
	    vec![RelativeDate::new(Duration::days(5))]
	);
	assert_eq!(w.task_available(id2).unwrap(), true);
    }
    
    #[test]
    fn direct_dep() {	
	let now = Local::now();
	let mut w = Workspace::new();
        let id1 = w.add_task(
	    "TestTask",
	    Blank::new(),
	    Vec::new()
	);
	let id2 = w.add_task(
	    "TestTask2",
	    Deadline::new(now + Duration::days(4)),
	    vec![Direct::new(id1)]
	);
	assert_eq!(w.task_available(id2).unwrap(), false);
	w.task_complete(id1).unwrap();
	assert_eq!(w.task_available(id2).unwrap(), true);
    }

    #[test]
    fn multiple_deps() {
	let now = Local::now();
	let mut w = Workspace::new();
        let id1 = w.add_task("TestTask", Blank::new(), Vec::new());
	let id2 = w.add_task(
	    "TestTask2",
	    Deadline::new(now + Duration::days(4)),
	    vec![Direct::new(id1), Date::new(now + Duration::days(1))]
	);
	assert_eq!(w.task_available(id2).unwrap(), false);
	w.task_complete(id1).unwrap();
	assert_eq!(w.task_available(id2).unwrap(), false);
	let id3 = w.add_task("TestTask", Blank::new(), Vec::new());
	let id4 = w.add_task(
	    "TestTask2",
	    Deadline::new(now + Duration::days(4)),
	    vec![Direct::new(id3), Date::new(now - Duration::days(1))]
	);
	assert_eq!(w.task_available(id4).unwrap(), false);
	w.task_complete(id3).unwrap();
	assert_eq!(w.task_available(id4).unwrap(), true);
    }
}
