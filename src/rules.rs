use super::model::*;

/// Singleton dates (normal due + defer)
pub struct SingletonDateRule {
    /// The "due date" of the rule
    due: u32,
    /// The "defer date" of the rule
    defer: u32,
    /// Whether or not the date is done
    done: bool
}

impl DateRule for SingletonDateRule {
    fn next(&self) -> Option<(u32,u32)> {
        if !self.done { Some((self.due,
                               self.defer)) }
        else { None }
    }

    fn increment(&mut self) -> () {
        self.done = true;
    }

    fn active(&self) -> bool {
        return !self.done;
    }
}

impl SingletonDateRule {
    pub fn new(due:u32, defer:u32) -> Self {
        SingletonDateRule {
            due:due,
            defer:defer,
            done:false
        }
    }
}

