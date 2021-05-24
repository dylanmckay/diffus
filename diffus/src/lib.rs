pub mod diffable_impls;
pub mod edit;
mod lcs;
pub mod same;
mod twodvec;

pub trait Diffable<'a> {
    type Diff: 'a + Clone;

    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self>;
}

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use diffus_derive::*;
