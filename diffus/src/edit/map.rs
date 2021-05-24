use crate::Diffable;

#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Edit<'a, T: Diffable<'a> + ?Sized> {
    Copy(&'a T),
    Insert(&'a T),
    Remove(&'a T),
    Change(T::Diff),
}

impl<'a, T: Diffable<'a> + ?Sized> Edit<'a, T> {
    //
    // Checks if the edit is an insert.
    //
    // # Examples
    //
    // ```
    // assert_eq!(Edit::Insert(&2).is_insert(), true);
    // assert_eq!(Edit::Remove.is_insert(), false);
    // ```
    pub fn is_insert(&self) -> bool {
        if let Self::Insert(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_remove(&self) -> bool {
        if let Self::Remove(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_copy(&self) -> bool {
        if let Self::Copy(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_change(&self) -> bool {
        if let Self::Change(_) = self {
            true
        } else {
            false
        }
    }
    pub fn insert(&self) -> Option<&'a T> {
        if let Self::Insert(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn remove(&self) -> Option<&'a T> {
        if let Self::Remove(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn change(&self) -> Option<&T::Diff> {
        if let Self::Change(value_diff) = self {
            Some(value_diff)
        } else {
            None
        }
    }
    pub fn copy(&self) -> Option<&'a T> {
        if let Self::Copy(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
