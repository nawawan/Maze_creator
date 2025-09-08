
pub fn grid_edges(width: usize, height: usize) -> Vec<(usize, usize)>{
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            add_adjacent_edge(&mut edges, i, j, width, height);
        }
    }
    edges
}

pub fn index_2d_to_1d(row: usize, col: usize, width: usize) -> usize{
    row * width + col
}

// row, columnという並びで返す。
pub fn index_1d_to_2d(idx: usize, width: usize) -> (usize, usize) {
    let row = idx / width;
    (row, idx - row * width)
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

