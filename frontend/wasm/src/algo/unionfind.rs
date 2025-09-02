
// union by size
struct UnionFind {
    size: Vec<i32>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind{
            size: vec![-1; n],
            n: n,
        }
    }

    fn root(&mut self, node: usize) -> usize {
        assert!(node < self.n);
        if let Ok(parent) = self.size[node].try_into() {
            let root_node = self.root(parent);
            self.size[node] = root_node as i32;
            return root_node;
        }
        return node;
    }

    fn size(&mut self, node: usize) -> i32 {
        let root_node = self.root(node);
        -self.size[root_node]
    }

    fn merge(&mut self, left_node: usize, right_node: usize) {
        let mut root_left = self.root(left_node);
        let mut root_right = self.root(right_node);

        // union into larger node
        if self.size(root_right) > self.size(root_left) {
            (root_left, root_right) = (root_right, root_left);
        }
        self.size[root_left] += self.size[root_right];
        self.size[root_right] = root_left as i32;
    }
}

