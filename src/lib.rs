mod model;
pub mod rules;

pub use model::*;
pub use rules::Date;
pub use rules::*;

#[cfg(test)]
mod tests {
    use super::*;    
    use chrono::Local;
    use chrono::Duration;    

    fn make_blank_task(w: &mut Workspace) -> uuid::Uuid {
	w.add_task("", Blank::new(), Vec::new())
    }
    
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

    #[test]
    fn hierarchical_tasks() {	
	let mut w = Workspace::new();
	let id1 = make_blank_task(&mut w);
	let id2 = make_blank_task(&mut w);
	let _id3 = make_blank_task(&mut w);
	let id4 = make_blank_task(&mut w);
	let id5 = make_blank_task(&mut w);
	w.task_add_child(id2, id4).unwrap();
	w.task_add_child(id2, id5).unwrap();
	w.task_add_child(id5, id1).unwrap();
	assert_eq!(w.task_get_parent(id4).unwrap(), id2);
	assert_eq!(w.task_get_parent(id5).unwrap(), id2);
	assert_eq!(w.task_get_parent(id1).unwrap(), id5);
    }

    #[test]
    fn metadata_sanity() {
	let mut w = Workspace::new();
	let id1 = make_blank_task(&mut w);
	let now = Local::now();
	w.task_add_metadata(id1, "Last Modified".to_string(), now.to_rfc3339()).unwrap();
	assert_eq!(
	    *w.task_get_metadata(id1, "Last Modified".to_string()).unwrap().unwrap(),
	    now.to_rfc3339()
	);
	w.task_set_metadata(id1, "Last Modified".to_string(), (now+Duration::days(1)).to_rfc3339()).unwrap();
	assert_eq!(
	    *w.task_get_metadata(id1, "Last Modified".to_string()).unwrap().unwrap(),
	    (now+Duration::days(1)).to_rfc3339()
	);
    }

    #[test]
    fn hierarchical_deps() {
	let mut w = Workspace::new();
	let id1 = w.add_task("", Blank::new(), vec![Parent::new()]);
	let id2 = w.add_task("", Blank::new(), vec![Children::new()]);	
	let _id3 = make_blank_task(&mut w);
	let id4 = make_blank_task(&mut w);
	let id5 = make_blank_task(&mut w);
	w.task_add_child(id2, id4).unwrap();
	w.task_add_child(id2, id5).unwrap();
	w.task_add_child(id5, id1).unwrap();
	assert_eq!(w.task_available(id4).unwrap(), true);
	assert_eq!(w.task_available(id5).unwrap(), true);
	assert_eq!(w.task_available(id2).unwrap(), false);
	assert_eq!(w.task_available(id1).unwrap(), false);
	w.task_complete(id5).unwrap();
	assert_eq!(w.task_available(id1).unwrap(), true);
	assert_eq!(w.task_available(id2).unwrap(), false);
	w.task_complete(id4).unwrap();
	assert_eq!(w.task_available(id2).unwrap(), true);
    }

    #[test]
    fn serialization() {
	let now = Local::now();
	let mut w = Workspace::new();
        let id1 = w.add_task("TestTask", Blank::new(), Vec::new());
	let id2 = w.add_task(
	    "TestTask2",
	    Deadline::new(now + Duration::days(4)),
	    vec![Direct::new(id1), Date::new(now + Duration::days(1))]
	);
	let s = serde_json::to_string(&w).unwrap();
	let w2: Workspace = serde_json::from_str(&s).unwrap();
	assert_eq!(w2.task_available(id1).unwrap(), true);
	assert_eq!(w2.task_available(id2).unwrap(), false);
	w.task_complete(id1).unwrap();
	assert_eq!(w2.task_available(id2).unwrap(), false);
	assert_eq!(w2.tasks.get(&id2).unwrap().date.current().unwrap(), now + Duration::days(4));
	assert_eq!(w2.tasks.get(&id2).unwrap().title, "TestTask2");
    }
}
