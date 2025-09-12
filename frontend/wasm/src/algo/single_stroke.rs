use std::{collections::VecDeque, mem::swap};

use rand::SeedableRng;
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::algo::kruskal;

fn log_str(s: &str) {
    console::log_1(&JsValue::from_str(s));
}

enum Offset {
    One,
    Zero,
}

pub fn single_stroke_maze(mut width: usize, mut height: usize) -> Vec<(usize, usize)> {
    width -= 1;
    height -= 1;
    if width % 2 == 0 && height % 2 == 0 {
        return Vec::new();
    }

    let step = 2;
    let mut used_grid_line = kruskal::extract_used_maze_edges_by_kruskal(
        width - (width + 1) % 2,
        height - (height + 1) % 2,
        step,
    );

    let mut used_grid_edges = divide_edges(&mut used_grid_line, step);

    let mut random_bytes = [0u8; 1];
    getrandom::getrandom(&mut random_bytes).unwrap();
    let offset = match u8::from_ne_bytes(random_bytes) % 2 {
        0 => Offset::Zero,
        1 => Offset::One,
        _ => panic!("impossible value"),
    };

    if width % 2 == 0 {
        shift_horizontal(&mut used_grid_edges, width - 1, height, step, offset);
    } else if height % 2 == 0 {
        shift_vertical(&mut used_grid_edges, width, height, step, offset);
    }

    let w = width + 2;
    let h = height + 2;

    let mut edges: Vec<(usize, usize)> = used_grid_edges
        .iter()
        .map(|(start, end)| (start / width * w + start % width + w + 1, end / width * w + end % width + w + 1))
        .collect();

    let size = w * h;
    let mut used_grid = vec![false; size];
    for (start, end) in edges.iter() {
        assert!(*start < size && *end < size);
        used_grid[*start] = true;
        used_grid[*end] = true;
    }
    let mut queue = VecDeque::new();

    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];
    queue.push_front(0);
    while let Some(v) = queue.pop_front() {
        if used_grid[v] {
            continue;
        }
        used_grid[v] = true;
        let x = v / w;
        let y = v % w;
        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];
            if 0 <= nx && nx < h as i32 && 0 <= ny &&  ny < w as i32 {
                let nv = w * nx as usize + ny as usize;
                if used_grid[nv] {
                    continue;
                }
                edges.push((v, nv));
                queue.push_back(nv);
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

fn shift_horizontal(
    edges: &mut Vec<(usize, usize)>,
    width: usize,
    height: usize,
    step: usize,
    offset: Offset,
) {
    match offset {
        Offset::Zero => {
            edges.iter_mut().for_each(|(x, y)| {
                *x = *x / width * (width + 1) + *x % width;
                *y = *y / width * (width + 1) + *y % width;
            });
            for row in (0..height).step_by(step) {
                let pos = row * (width + 1) + width;
                edges.push((pos - 1, pos));
            }
        }
        Offset::One => {
            edges.iter_mut().for_each(|(x, y)| {
                *x = *x / width * (width + 1) + *x % width + 1;
                *y = *y / width * (width + 1) + *y % width + 1; 
            });
            for row in (0..height).step_by(step) {
                let pos = row * (width + 1);
                edges.push((pos, pos + 1));
            }
        }
    }
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
            for col in (0..width).step_by(step) {
                let pos = (height - 1) * width + col;
                edges.push((pos - width, pos));
            }
        }
        Offset::One => {
            edges.iter_mut().for_each(|(x, y)| {
                *x += width;
                *y += width;
            });
            for col in (0..width).step_by(step) {
                let pos = col;
                edges.push((pos, pos + width));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algo::single_stroke::shift_vertical;
    use rstest::*;

    use super::Offset;

    #[rstest]
    #[case(&mut vec![(2, 3), (3, 4), (3, 8)], 5, 11, 1, Offset::Zero, &mut vec![(2, 3), (3, 4), (3, 8), (45, 50), (46, 51), (47, 52), (48, 53), (49, 54)])]
    #[case(&mut vec![(2, 3), (3, 4), (3, 8)], 5, 11, 1, Offset::One, &mut vec![(7, 8), (8, 9), (8, 13), (0, 5), (1, 6), (2, 7), (3, 8), (4, 9)])]
    fn shift_vertically_edges_correct_offset(
        #[case] edges: &mut Vec<(usize, usize)>,
        #[case] width: usize,
        #[case] height: usize,
        #[case] step: usize,
        #[case] offset: Offset,
        #[case] expect: &mut Vec<(usize, usize)>,
    ) {
        shift_vertical(edges, width, height, step, offset);

        expect.sort();
        edges.sort();

        assert_eq!(expect, edges);
    }
}
