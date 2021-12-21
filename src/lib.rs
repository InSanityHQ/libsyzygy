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
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    fn direct_dep() {	
	let now = Local::now();
	let mut w = Workspace::new();
        w.add_task(
	    "TestTask",
	    Box::new(Deadline::new(now + Duration::days(3))),
	    None
	);
	w.add_task(
	    "TestTask2",
	    Box::new(Deadline::new(now + Duration::days(4))),
	    Some(Box::new(Direct::new(&w, *w.tasks.keys().next().unwrap())))
	);	
	assert_eq!(w.tasks.values().nth(1).unwrap().dependency.as_ref().unwrap().available(), false);
	w.tasks.values().next().unwrap().date.next();
	assert_eq!(w.tasks.values().nth(1).unwrap().dependency.as_ref().unwrap().available(), true);
    }

    // #[test]
    // fn repeat_constant() {	
    //     let now = Local::now();
    //     let mut t: Task = Task::new(
    //         "TestTask",
    //         Box::new(Constant::new(

    //             , end_date: Option<DateTime<Local>>, repeat: Duration, defer: Duration, ))
    //         // Box::new(Deadline::new(
    //         //     now + Duration::days(3),
    //         //     now + Duration::days(1),
    //         // )),
    //     );
    //     assert_eq!(t.date.current(), None);
    //     assert_eq!(t.date.active(), RecurState::Pending(now + Duration::days(1)));
    //     t.date.next();
    //     assert_eq!(t.date.active(), RecurState::Dead);
    // }
    // TODO
}
