//! Disjoint-set union, also called union-find.

/// A collection of disjoint sets of numbered items.
///
/// `find` uses path halving, so the trees stay short. `union` joins two sets
/// and reports whether they were separate.
#[derive(Debug, Clone)]
pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    sets: usize,
}

impl Dsu {
    /// Create `count` sets, each holding one item.
    pub fn new(count: usize) -> Self {
        Self {
            parent: (0..count).collect(),
            size: vec![1; count],
            sets: count,
        }
    }

    /// The representative of the set holding `item`.
    pub fn find(&mut self, mut item: usize) -> usize {
        while self.parent[item] != item {
            self.parent[item] = self.parent[self.parent[item]];
            item = self.parent[item];
        }
        item
    }

    /// Join the sets holding `a` and `b`.
    ///
    /// Returns `true` when they were separate, `false` when they already
    /// belonged to the same set.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
        self.sets -= 1;
        true
    }

    /// Whether two items belong to the same set.
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// The number of separate sets.
    pub fn count(&self) -> usize {
        self.sets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_one_set_per_item() {
        let dsu = Dsu::new(4);
        assert_eq!(dsu.count(), 4);
    }

    #[test]
    fn union_joins_two_items() {
        let mut dsu = Dsu::new(4);
        assert!(dsu.union(0, 1));
        assert!(dsu.connected(0, 1));
        assert_eq!(dsu.count(), 3);
    }

    #[test]
    fn union_twice_changes_nothing() {
        let mut dsu = Dsu::new(4);
        assert!(dsu.union(0, 1));
        assert!(!dsu.union(1, 0));
        assert_eq!(dsu.count(), 3);
    }

    #[test]
    fn union_is_transitive() {
        let mut dsu = Dsu::new(4);
        dsu.union(0, 1);
        dsu.union(1, 2);
        assert!(dsu.connected(0, 2));
        assert!(!dsu.connected(0, 3));
    }
}
