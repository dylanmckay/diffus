use crate::{
    edit::{map, Edit},
    Diffable,
};

macro_rules! map_impl {
    ($(($typ:ident, $key_constraint:ident)),*) => {
        $(
            impl<'a, K: Eq + $key_constraint + 'a, V: Diffable<'a> + 'a> Diffable<'a> for $typ<K, V>
                where V: Clone {
                type Diff = $typ<&'a K, map::Edit<'a, V>>;

                fn diff(&'a self, other: &'a Self) -> Edit<Self> {
                    let intersection = self
                        .iter()
                        .filter_map(|(k, v)| Some((k, (v, other.get(k)?))));

                    let unique_self = self.iter().filter(|(k, _)| !other.contains_key(*k));

                    let unique_other = other.iter().filter(|(k, _)| !self.contains_key(*k));

                    let value_diffs = unique_other
                        .map(|(k, v)| (k, map::Edit::Insert(v)))
                        .chain(unique_self.map(|(k, v)| (k, map::Edit::Remove(v))))
                        .chain(intersection.map(|(k, (self_v, other_v))| (k, self_v.diff(other_v).into())))
                        .collect::<$typ<_, _>>();

                    if value_diffs.values().any(|v| !v.is_copy()) {
                        Edit::Change(value_diffs)
                    } else {
                        Edit::Copy(self)
                    }
                }
            }
        )*
    }
}

use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};
map_impl! {
    (BTreeMap, Ord),
    (HashMap, Hash)
}

#[cfg(feature = "indexmap-impl")]
use indexmap::IndexMap;
#[cfg(feature = "indexmap-impl")]
map_impl! { (IndexMap, Hash) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let unity: std::collections::HashMap<_, _> =
            [(1, 1), (2, 2), (3, 3)].iter().cloned().collect();
        let not_unity: std::collections::HashMap<_, _> =
            [(1, 1), (2, 3), (4, 4)].iter().cloned().collect();

        if let Edit::Change(diff) = unity.diff(&not_unity) {
            assert!(diff[&1].is_copy());
            assert_eq!(diff[&2].change().unwrap(), &(&2, &3));
            assert!(diff[&3].is_remove());
            assert_eq!(diff[&4].insert().unwrap(), &4);
        } else {
            unreachable!()
        }
    }
}
