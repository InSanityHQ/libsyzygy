mod model;
pub mod rules;

pub use model::Task;
pub use model::Recur;
pub use model::RecurState;
pub use rules::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    fn task_title() {
        let mut t: Task = Task::new(
            "TestTask",
            Box::new(Blank {})
        );

        assert_eq!(t.title, "TestTask");

        t.title = String::from("testTask1");

        assert_eq!(t.title, "testTask1");
    }

    #[test]
    fn repeat_deadline() {	
	let now = Local::now();
        let mut t: Task = Task::new(
	    "TestTask",
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
