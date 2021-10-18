use uuid::Uuid;

/// State of the date rule, returned on active()
pub enum DateRuleState {
    /// Dead, will not become active again
    Dead,
    /// Pending until specific date
    Pending(u32),
    /// Date held manually/due to arbiturary non-date reasons
    Held,
    /// Actively running, could be completed again
    Active
}

pub trait DateRule {
    /// Returns an `Option` containing potentionally
    /// the next set of due and defer dates
    ///
    /// # Examples
    ///
    /// When the rule is uncompleted/still repeating/available:
    ///
    /// ```
    /// let rule = ImplDateRule::new();
    /// rule.next(); // => Some(1202919209,1204919209)
    /// ```
    fn next(&self) -> Option<(u32,u32)>;

    /// Increment the Date Rule. Similar to "completing" a task.
    fn increment(&mut self) -> ();

    /// Returns a `DateRuleState` containing potentionally
    /// the next set of due dates.
    ///
    /// # Possible States
    /// See documentation on `DateRuleState`
    fn active(&self) -> DateRuleState;
}

/// Collection of tasks, also the source of querying commands
pub struct Workspace {
}

/// Rules of dependency blocking
pub enum Dependency {
    /// No dependencies
    Free,
    /// Manual, direct dependents
    Direct(Vec<Uuid>),
    /// Direct parent
    Parent,
    /// Direct sibling above
    Above,
    /// Direct sibling below
    Below
}

/// A Task! ID and pointers to others identified by UUIDs
pub struct Task<'a> {
    /// A mutable pointer to a DateRule by which this task subscribes to
    date: &'a mut dyn DateRule,
    /// A mutable pointer to a DependentRule
    dependency: Dependency,
    /// Whether timeblocking is enabled
    timeblock_enabled: bool,
    /// An optional timeblock value
    timeblock: u32,
    /// Estimated duration of the task. If blocking,
    /// blocked time is timeblock + estimated_duration
    estimated_duration: u16,
    /// Pointer to vector of immutable borrows of children UUIDs
    children: Vec<Uuid>,
    /// ID
    id: Uuid,
}



