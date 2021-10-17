/// State of the date rule, returned on active()
pub enum DateRuleState {
    /// Dead, will not become active again
    Dead,
    /// Pending until specific date
    Pending(u32),
    /// Held until other, non-hald arbiturary dependencies
    Held,
    /// Actively running, could be completed again
    Active
}

pub trait DateRule {
    /// Returns an Option containing potentionally
    /// the next set of due dates
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

    /// 
    fn increment(&mut self) -> ();
    fn active(&self) -> bool;
}

pub struct Task<'a> {
    date: &'a mut dyn DateRule,
    pub timeblock: u32,
    pub estimated_duration: u16
}
