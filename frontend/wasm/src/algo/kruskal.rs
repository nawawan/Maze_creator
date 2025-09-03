use crate::algo::unionfind::UnionFind;

// 縦heightマス・横widthマスのグリッドグラフについて、最小全域木を作成したときに使用しなかった辺を返す
pub fn extract_unused_maze_edges_by_kruskal(width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            add_adjacent_edge(&mut edges, i, j, width, height);
        }
    }
    kruskal(width * height, edges)
}

// グリッドグラフにおいて、隣接マスへの辺が存在したらedgesに追加する
fn add_adjacent_edge(edges: &mut Vec<(usize, usize)>, row: usize, column: usize, width: usize, height: usize) {
    if row + 1 < height {
        edges.push((row * width + column, (row + 1) * width + column));
    }
    if column + 1 < width {
        edges.push((row * width + column, row * width + column + 1))
    }
}

// kruskal法によって最小全域木を作成し、使用しなかった辺を返す
fn kruskal(node_size: usize, edges: Vec<(usize, usize)>) -> Vec<(usize, usize)>{
    let mut unionfind = UnionFind::new(node_size);
    let mut unused_edge: Vec<(usize, usize)> = Vec::new();

    for (node_x, node_y) in edges {
        if !unionfind.same(node_x, node_y) {
            unionfind.merge(node_x, node_y);
        }
        else {
            unused_edge.push((node_x, node_y));
        }
    }
    unused_edge
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn create_minimum_spanning_tree(){
        let node_size: usize = 5;
        let mut edges: Vec<(usize, usize)> = Vec::new();

        // create minimum spanning tree
        for i in 0..node_size {
            for j in i..node_size {
                edges.push((i, j));
            }
        } 
        let unused_edges = kruskal(node_size, edges.clone());

        // fetch node in spanning tree
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        unused_edges.into_iter().for_each(|x| {set.insert(x);});
        let mut contains_integer : HashSet<usize> = HashSet::new();
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
    fn return_just_right_unused_edges(){
        let node_size: usize = 5;
        let mut edges: Vec<(usize, usize)> = Vec::new();

        for i in 0..node_size {
            for j in i..node_size {
                edges.push((i, j));
            }
        } 

        let unused_edges = kruskal(node_size, edges.clone());

        // MSTに必要な辺の数はnode_size - 1
        assert_eq!(edges.len(), unused_edges.len() + node_size - 1);
    }

    #[test]
    fn create_correct_minimum_spanning_tree_in_grid_graph(){
        let width = 10;
        let height = 20;
        let unused_edges = extract_unused_maze_edges_by_kruskal(width, height);

        // fetch node in spanning tree
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        unused_edges.into_iter().for_each(|x| {set.insert(x);});

        let mut nodes: HashSet<usize> = HashSet::new();
        for i in 0..height {
            for j in 0..width {
                let vertical = (i * width + j, (i + 1) * width + j);
                let horizontal = (i * width + j, i * width + j + 1);
                if i + 1 < height && !set.contains(&vertical) {
                    nodes.insert(i * width + j);
                    nodes.insert((i + 1) * width + j);
                }
                if j + 1 < width && !set.contains(&horizontal){
                    nodes.insert(i * width + j);
                    nodes.insert(i * width + j + 1);
                }
            }
        }

        assert_eq!(width * height, nodes.len());
    }

}