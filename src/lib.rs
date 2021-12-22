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
        let id1 = w.add_task(
	    "TestTask",
	    Deadline::new(now + Duration::days(3)),
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
