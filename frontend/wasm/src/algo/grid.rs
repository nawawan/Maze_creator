pub fn grid_edges(width: usize, height: usize, step: usize) -> Vec<(usize, usize)> {
    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(2 * width * height - width - height);
    for i in (0..height).step_by(step) {
        for j in (0..width).step_by(step) {
            add_adjacent_edge(&mut edges, i, j, width, height, step);
        }
    }
    edges
}

pub fn index_2d_to_1d(row: usize, col: usize, width: usize) -> usize {
    row * width + col
}

// row, columnという並びで返す。
pub fn index_1d_to_2d(idx: usize, width: usize) -> (usize, usize) {
    let row = idx / width;
    (row, idx - row * width)
}

// グリッドグラフにおいて、隣接マスへの辺が存在したらedgesに追加する
fn add_adjacent_edge(
    edges: &mut Vec<(usize, usize)>,
    row: usize,
    column: usize,
    width: usize,
    height: usize,
    step: usize,
) {
    if row + step < height {
        edges.push((row * width + column, (row + step) * width + column));
    }
    if column + step < width {
        edges.push((row * width + column, row * width + column + step))
    }
}

#[cfg(test)]
mod tests {}
