use super::error::TDError;

/// Represents any completable objects
///
/// Reduces specialisation so that Projects/Tasks, etc.
/// could have the same level of features w/o much work.
pub trait Completable {
    fn complete(_: Self) -> bool;
    fn rename(&self) -> Result<(), TDError>;
}

