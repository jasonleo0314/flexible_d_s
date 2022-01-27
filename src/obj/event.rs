//事件

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Default, Clone)]
pub struct Event {
    id: usize,
    child: HashSet<usize>,
}

impl Event {
    pub fn diff_set(&self, other: &HashSet<usize>) -> HashSet<usize> {
        self.child
            .difference(other)
            .map(|&a| a)
            .collect::<HashSet<usize>>()
    }

    pub fn cover(&self, other: &Event) -> bool {
        other.belong(self)
    }

    pub fn is_superset(&self, other: &HashSet<usize>) -> bool {
        other.is_subset(self.child())
    }

    pub fn belong(&self, other: &Event) -> bool {
        self.child().is_subset(other.child())
    }

    pub fn is_subset(&self, other: &HashSet<usize>) -> bool {
        self.child().is_subset(other)
    }

    pub fn union_event(&self, other: &Event) -> Event {
        Event::new_with_child(0, self.union(other))
    }

    pub fn union(&self, other: &Event) -> HashSet<usize> {
        self.child
            .union(other.child())
            .map(|&a| a)
            .collect::<HashSet<usize>>()
    }

    pub fn inter_event(&self, other: Event) -> Event {
        Event::new_with_child(0, self.intersect(&other))
    }

    pub fn intersect(&self, other: &Event) -> HashSet<usize> {
        self.child
            .intersection(other.child())
            .map(|&a| a)
            .collect::<HashSet<usize>>()
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.child.len().cmp(&other.child.len())
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.child == other.child
    }
}
impl Eq for Event {}

//构造
impl Event {
    pub fn new_with_child(id: usize, child: HashSet<usize>) -> Self {
        Event { id, child }
    }
    pub fn new(id: usize) -> Self {
        Event {
            id,
            child: HashSet::new(),
        }
    }
}
//Getter
impl Event {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn child(&self) -> &HashSet<usize> {
        &self.child
    }
}
