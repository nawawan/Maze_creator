use rand::SeedableRng;
use rand::prelude::*;

use crate::algo::grid::grid_edges;
use crate::algo::unionfind::UnionFind;

pub enum KruskalResultEdge {
    Used,
    Unused,
}

// 縦heightマス・横widthマスのグリッドグラフについて、最小全域木を作成したときに使用しなかった辺を返す
pub fn extract_maze_edges_by_kruskal(
    width: usize,
    height: usize,
    step: usize, result: KruskalResultEdge
) -> Vec<(usize, usize)> {
    if width <= step && height <= step {
        return Vec::new();
    }
    let edges = arrange_random_edges(width, height, step);
    kruskal(width * height, edges, result)
}

// kruskal法によって最小全域木を作成し、使用しなかった辺を返す
fn kruskal(
    node_size: usize,
    edges: Vec<(usize, usize)>,
    edge_result: KruskalResultEdge,
) -> Vec<(usize, usize)> {
    let mut unionfind = UnionFind::new(node_size);
    let mut unused_edge: Vec<(usize, usize)> = Vec::new();
    let mut used_edge: Vec<(usize, usize)> = Vec::new();

    for (node_x, node_y) in edges {
        if !unionfind.same(node_x, node_y) {
            unionfind.merge(node_x, node_y);
            used_edge.push((node_x, node_y));
        } else {
            unused_edge.push((node_x, node_y));
        }
    }
    match edge_result {
        KruskalResultEdge::Used => used_edge,
        KruskalResultEdge::Unused => unused_edge,
    }
}

// ランダムな順番のgridグラフの辺を返す
fn arrange_random_edges(width: usize, height: usize, step: usize) -> Vec<(usize, usize)> {
    let mut edges = grid_edges(width, height, step);
    let mut random_bytes = [0u8; 8];
    getrandom::getrandom(&mut random_bytes).unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(u64::from_ne_bytes(random_bytes));
    edges.shuffle(&mut rng);
    edges
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn create_minimum_spanning_tree() {
        let node_size: usize = 5;
        let mut edges: Vec<(usize, usize)> = Vec::new();

        // create minimum spanning tree
        for i in 0..node_size {
            for j in i..node_size {
                edges.push((i, j));
            }
        }
        let unused_edges = kruskal(node_size, edges.clone(), KruskalResultEdge::Used);

        // fetch node in spanning tree
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        unused_edges.into_iter().for_each(|x| {
            set.insert(x);
        });
        let mut contains_integer: HashSet<usize> = HashSet::new();
        for (x, y) in edges {
            if set.contains(&(x, y)) {
                continue;
            }
            contains_integer.insert(x);
            contains_integer.insert(y);
        }

        assert_eq!(node_size, contains_integer.len());
    }

    #[test]
    fn return_just_right_unused_edges() {
        let node_size: usize = 5;
        let mut edges: Vec<(usize, usize)> = Vec::new();

        for i in 0..node_size {
            for j in i..node_size {
                edges.push((i, j));
            }
        }

        let unused_edges = kruskal(node_size, edges.clone(), KruskalResultEdge::Unused);

        // MSTに必要な辺の数はnode_size - 1
        assert_eq!(edges.len(), unused_edges.len() + node_size - 1);
    }

    #[rstest]
    #[case(10, 20, 1)]
    #[case(10, 20, 2)]
    #[case(1000, 1000, 13)]
    #[case(50, 100, 7)]
    #[case(5, 10, 7)]
    fn create_correct_minimum_spanning_tree_from_unused_edges_in_grid_graph(
        #[case] width: usize,
        #[case] height: usize,
        #[case] step: usize,
    ) {
        let nodes = create_spanning_tree_from_unused_edges(width, height, step);

        let w = (width - 1) / step + 1;
        let h = (height - 1) / step + 1;

        assert_eq!(w * h, nodes.len());
    }

    #[rstest]
    #[case(10, 20, 1)]
    #[case(10, 20, 2)]
    #[case(1000, 1000, 13)]
    #[case(50, 100, 7)]
    #[case(5, 10, 7)]
    fn create_correct_minimum_spanning_tree_from_used_edges_in_grid_graph(
        #[case] width: usize,
        #[case] height: usize,
        #[case] step: usize,
    ) {
        let nodes = create_spanning_tree_from_used_edges(width, height, step);

        let w = (width - 1) / step + 1;
        let h = (height - 1) / step + 1;

        assert_eq!(w * h, nodes.len());
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 10, 12)]
    #[case(100, 100, 101)]
    fn invalid_input_creates_empty_tree_from_unused_edges(
        #[case] width: usize,
        #[case] height: usize,
        #[case] step: usize,
    ) {
        let nodes = create_spanning_tree_from_unused_edges(width, height, step);
        assert_eq!(0, nodes.len());
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 10, 12)]
    #[case(100, 100, 101)]
    fn invalid_input_creates_empty_tree_from_used_edges(
        #[case] width: usize,
        #[case] height: usize,
        #[case] step: usize,
    ) {
        let nodes = create_spanning_tree_from_used_edges(width, height, step);
        assert_eq!(0, nodes.len());
    }

    fn create_spanning_tree_from_unused_edges(
        width: usize,
        height: usize,
        step: usize,
    ) -> HashSet<usize> {
        let unused_edges = extract_maze_edges_by_kruskal(width, height, step, KruskalResultEdge::Unused);

        // fetch node in spanning tree
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        unused_edges.into_iter().for_each(|x| {
            set.insert(x);
        });

        let mut nodes: HashSet<usize> = HashSet::new();
        for i in (0..height).step_by(step) {
            for j in (0..width).step_by(step) {
                let vertical = (i * width + j, (i + step) * width + j);
                let horizontal = (i * width + j, i * width + j + step);
                if i + step < height && !set.contains(&vertical) {
                    nodes.insert(i * width + j);
                    nodes.insert((i + step) * width + j);
                }
                if j + step < width && !set.contains(&horizontal) {
                    nodes.insert(i * width + j);
                    nodes.insert(i * width + j + step);
                }
            }
        }
        nodes
    }

    fn create_spanning_tree_from_used_edges(
        width: usize,
        height: usize,
        step: usize,
    ) -> HashSet<usize> {
        let used_edges = extract_maze_edges_by_kruskal(width, height, step, KruskalResultEdge::Used);

        // fetch node in spanning tree
        let mut nodes: HashSet<usize> = HashSet::new();
        for (node_left, node_right) in used_edges {
            nodes.insert(node_left);
            nodes.insert(node_right);
        }
        nodes
    }
}
