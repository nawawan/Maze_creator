use std::{collections::VecDeque, mem::swap};

use rand::SeedableRng;

use crate::algo::kruskal;

enum Offset {
    One,
    Zero,
}

pub fn single_stroke_maze(mut width: usize, mut height: usize) -> Vec<(usize, usize)> {
    width -= 1;
    height -= 1;
    if width % 2 == 1 && height % 2 == 1 {
        return Vec::new();
    }

    let step = 2;
    let mut used_grid_line =
        kruskal::extract_used_maze_edges_by_kruskal(width / 2 * 2, height / 2 * 2, step);

    let mut used_grid_edges = divide_edges(&mut used_grid_line, step);

    let mut random_bytes = [0u8; 1];
    getrandom::getrandom(&mut random_bytes).unwrap();
    let offset = match u8::from_ne_bytes(random_bytes) % 2 {
        0 => Offset::Zero,
        1 => Offset::One,
        _ => panic!("impossible value"),
    };
    if width % 2 == 1 {
        shift_horizontal(&mut used_grid_edges, width, height, step, offset);
    } else if height % 2 == 1 {
        shift_vertical(&mut used_grid_edges, width, height, step, offset);
    }

    let mut edges: Vec<(usize, usize)> = used_grid_edges
        .iter()
        .map(|(start, end)| (start + width + 1, end + width + 1))
        .collect();

    let mut used_grid = vec![false; width * height];
    let size = width * height;
    for (start, end) in edges.iter() {
        assert!(*start < size && *end < size);
        used_grid[*start] = true;
        used_grid[*end] = true;
    }
    let mut queue = VecDeque::new();
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    queue.push_front(0);
    while let Some(v) = queue.pop_front() {
        if used_grid[v] {
            continue;
        }
        used_grid[v] = true;
        let x = v / width;
        let y = v % width;
        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];
            if 0 <= nx
                && nx < height.try_into().unwrap()
                && 0 <= ny
                && ny < width.try_into().unwrap()
            {
                let nv = x * width + y;
                if used_grid[nv] {
                    continue;
                }
                edges.push((v, nv));
                queue.push_front(nv);
            }
        }
    }
    edges
}

fn divide_edges(lines: &mut Vec<(usize, usize)>, step: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::with_capacity(lines.len() * step);
    for (start, end) in lines {
        if start > end {
            swap(start, end);
        }
        let interval = (*end - *start) / step;
        for line_begin in (*start..*end).step_by(interval) {
            edges.push((line_begin, line_begin + interval));
        }
    }
    edges
}

fn shift_vertical(
    edges: &mut Vec<(usize, usize)>,
    width: usize,
    height: usize,
    step: usize,
    offset: Offset,
) {
    match offset {
        Offset::Zero => {
            for row in (0..height).step_by(step) {
                let pos = row * width + width - 1;
                edges.push((pos - 1, pos));
            }
        }
        Offset::One => {
            edges.iter_mut().for_each(|(_, y)| *y += 1);
            for row in (0..height).step_by(step) {
                let pos = row * width;
                edges.push((pos, pos + 1));
            }
        }
    }
}

fn shift_horizontal(
    edges: &mut Vec<(usize, usize)>,
    width: usize,
    height: usize,
    step: usize,
    offset: Offset,
) {
    match offset {
        Offset::Zero => {
            for col in (0..width).step_by(step) {
                let pos = (height - 1) * width + col;
                edges.push((pos - width, pos));
            }
        }
        Offset::One => {
            edges.iter_mut().for_each(|(x, _)| *x += 1);
            for col in (0..width).step_by(step) {
                let pos = col;
                edges.push((pos, pos + width));
            }
        }
    }
}

#[cfg(test)]
mod tests {}
