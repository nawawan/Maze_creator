// union by size
pub struct UnionFind {
    size: Vec<i32>,
    n: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            size: vec![-1; n],
            n: n,
        }
    }

    fn root(&mut self, node: usize) -> usize {
        assert!(node < self.n);
        if let Ok(parent) = self.size[node].try_into() {
            let root_node: usize = self.root(parent);
            self.size[node] = root_node as i32;
            return root_node;
        }
        node
    }

    pub fn size(&mut self, node: usize) -> i32 {
        let root_node = self.root(node);
        -self.size[root_node]
    }

    pub fn same(&mut self, left_node: usize, right_node: usize) -> bool {
        self.root(left_node) == self.root(right_node)
    }

    pub fn merge(&mut self, left_node: usize, right_node: usize) {
        let mut root_left = self.root(left_node);
        let mut root_right = self.root(right_node);

        if root_left == root_right {
            return;
        }

        // union into larger node
        if self.size(root_right) > self.size(root_left) {
            (root_left, root_right) = (root_right, root_left);
        }
        self.size[root_left] += self.size[root_right];
        self.size[root_right] = root_left as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_correct_root() {
        let mut unionfind = UnionFind::new(10);
        unionfind.merge(2, 5);

        assert_eq!(2, unionfind.root(2));
        assert_eq!(2, unionfind.root(5));
        assert_eq!(1, unionfind.root(1));
    }

    #[test]
    fn union_into_larger_subtree() {
        let mut unionfind = UnionFind::new(10);
        unionfind.merge(2, 5);
        unionfind.merge(1, 5);

        assert_eq!(2, unionfind.root(1));
        assert_eq!(2, unionfind.root(5));
        assert_eq!(2, unionfind.root(2));
    }

    #[test]
    fn cancel_merge_when_root_is_same() {
        let mut unionfind = UnionFind::new(10);
        unionfind.merge(1, 3);
        unionfind.merge(1, 3);

        assert_eq!(2, unionfind.size(1));
    }

    #[test]
    fn identify_differences_of_trees_nodes_belong_to() {
        let mut unionfind = UnionFind::new(10);
        unionfind.merge(1, 3);
        unionfind.merge(4, 5);

        assert!(unionfind.same(1, 3));
        assert!(unionfind.same(4, 5));
        assert!(!unionfind.same(1, 4));

        unionfind.merge(4, 1);

        assert!(unionfind.same(1, 4));
    }

    #[test]
    fn fetch_correct_size() {
        let mut unionfind = UnionFind::new(10);
        unionfind.merge(5, 6);
        unionfind.merge(1, 5);

        unionfind.merge(2, 9);

        assert_eq!(3, unionfind.size(1));
        assert_eq!(3, unionfind.size(5));
        assert_eq!(2, unionfind.size(9));
        assert_eq!(1, unionfind.size(3));
    }
}
