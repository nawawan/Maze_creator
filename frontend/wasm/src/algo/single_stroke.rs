use crate::algo::{kruskal, shape::Point};
use std::{collections::VecDeque, mem::swap};

#[derive(Clone, Copy)]
enum Offset {
    One,
    Zero,
}

const BIG_NUM: usize = 100000;

pub fn single_stroke_maze(
    mut width: usize,
    mut height: usize,
) -> Vec<(Point<usize>, Point<usize>)> {
    width -= 1;
    height -= 1;
    if width % 2 == 0 && height % 2 == 0 {
        return Vec::new();
    }

    let step = 2;
    let mut used_grid_line = kruskal::extract_maze_edges_by_kruskal(
        width - (width + 1) % 2,
        height - (height + 1) % 2,
        step,
        kruskal::KruskalResultEdge::Used,
    );
    let modulo = get_random_bool();
    let offset = match modulo {
        false => Offset::Zero,
        true => Offset::One,
    };

    if width % 2 == 0 {
        shift_horizontal(&mut used_grid_line, width, offset);
    } else if height % 2 == 0 {
        shift_vertical(&mut used_grid_line, height, offset);
    }

    let mut used_grid_edges = divide_edges(&mut used_grid_line, step);

    if width % 2 == 0 {
        add_end_horizontal(&mut used_grid_edges, width, height, step, offset);
    } else if height % 2 == 0 {
        add_end_vertical(&mut used_grid_edges, width, height, step, offset);
    }

    let w = width + 2;
    let h = height + 2;

    used_grid_edges.iter_mut().for_each(|(start, end)| {
        start.shift_horizontal();
        start.shift_vertical();
        end.shift_horizontal();
        end.shift_vertical();
    });

    let mut edges = used_grid_edges;

    let size = w * h;
    let mut used_grid = vec![false; size];
    for (start, end) in edges.iter() {
        used_grid[start.flatten(w)] = true;
        used_grid[end.flatten(w)] = true;
    }

    log::info!("single stroke. create outer perimeter");
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
            if 0 <= nx && nx < h as i32 && 0 <= ny && ny < w as i32 {
                let nv = w * nx as usize + ny as usize;
                if used_grid[nv] {
                    continue;
                }
                edges.push((Point::from_1d_index(v, w), Point::from_1d_index(nv, w)));
                queue.push_back(nv);
            }
        }
    }
    edges
}

// 与えられたPointのタプル間の線分を、step個に区切って新たなPointのタプルとして返す
fn divide_edges(
    lines: &Vec<(Point<usize>, Point<usize>)>,
    step: usize,
) -> Vec<(Point<usize>, Point<usize>)> {
    let mut edges = Vec::with_capacity(lines.len() * step);
    let width = BIG_NUM;
    for (start, end) in lines {
        let mut from = start.flatten(width);
        let mut to = end.flatten(width);
        if from > to {
            swap(&mut from, &mut to);
        }
        let interval = (to - from) / step;
        for line_begin in (from..to).step_by(interval) {
            edges.push((
                Point::<usize>::from_1d_index(line_begin, width),
                Point::<usize>::from_1d_index(line_begin + interval, width),
            ));
        }
    }
    edges
}

// 二次元座標中に存在する線分を、水平方向に+1移動させる
fn shift_horizontal(edges: &mut Vec<(Point<usize>, Point<usize>)>, width: usize, offset: Offset) {
    match offset {
        Offset::Zero => {
            edges.iter_mut().for_each(|(x, y)| {
                if x.y == y.y && x.y == width - 2 && get_random_bool() {
                    x.shift_horizontal();
                    y.shift_horizontal();
                }
            });
        }
        Offset::One => {
            edges.iter_mut().for_each(|(x, y)| {
                if !(x.y == y.y && x.y == 0 && get_random_bool()) {
                    x.shift_horizontal();
                    y.shift_horizontal();
                }
            });
        }
    }
}

fn add_end_horizontal(
    edges: &mut Vec<(Point<usize>, Point<usize>)>,
    width: usize,
    height: usize,
    step: usize,
    offset: Offset,
) {
    match offset {
        Offset::Zero => {
            for row in (0..height).step_by(step) {
                edges.push((Point::new(row, width - 2), Point::new(row, width - 1)));
            }
        }
        Offset::One => {
            for row in (0..height).step_by(step) {
                edges.push((Point::new(row, 0), Point::new(row, 1)));
            }
        }
    }
}

// 二次元座標中に存在する線分を、垂直s方向に+1移動させる
fn shift_vertical(edges: &mut Vec<(Point<usize>, Point<usize>)>, height: usize, offset: Offset) {
    match offset {
        Offset::Zero => {
            edges.iter_mut().for_each(|(x, y)| {
                if x.x == y.x && x.x == height - 2 && get_random_bool() {
                    x.shift_vertical();
                    y.shift_vertical();
                }
            });
        }
        Offset::One => {
            edges.iter_mut().for_each(|(x, y)| {
                if !(x.x == y.x && x.x == 0 && get_random_bool()) {
                    x.shift_vertical();
                    y.shift_vertical();
                }
            });
        }
    }
}

fn add_end_vertical(
    edges: &mut Vec<(Point<usize>, Point<usize>)>,
    width: usize,
    height: usize,
    step: usize,
    offset: Offset,
) {
    match offset {
        Offset::Zero => {
            for col in (0..width).step_by(step) {
                edges.push((Point::new(height - 2, col), Point::new(height - 1, col)));
            }
        }
        Offset::One => {
            for col in (0..width).step_by(step) {
                edges.push((Point::new(0, col), Point::new(1, col)));
            }
        }
    }
}

fn get_random_bool() -> bool {
    let mut random_bytes = [0u8; 1];
    match getrandom::getrandom(&mut random_bytes) {
        Ok(_) => u8::from_ne_bytes(random_bytes) % 2 == 1,
        Err(_) => {
            log::warn!("get_randome failed, so return false");
            false
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::algo::single_stroke::shift_vertical;
//     use rstest::*;

//     use super::Offset;

//     #[rstest]
//     #[case(&mut vec![(2, 3), (3, 4), (3, 8)], 5, 11, 1, Offset::Zero, &mut vec![(2, 3), (3, 4), (3, 8), (45, 50), (46, 51), (47, 52), (48, 53), (49, 54)])]
//     #[case(&mut vec![(2, 3), (3, 4), (3, 8)], 5, 11, 1, Offset::One, &mut vec![(7, 8), (8, 9), (8, 13), (0, 5), (1, 6), (2, 7), (3, 8), (4, 9)])]
//     fn shift_vertically_edges_correct_offset(
//         #[case] edges: &mut Vec<(usize, usize)>,
//         #[case] width: usize,
//         #[case] height: usize,
//         #[case] step: usize,
//         #[case] offset: Offset,
//         #[case] expect: &mut Vec<(usize, usize)>,
//     ) {
//         shift_vertical(edges, width, height, step, offset);

//         expect.sort();
//         edges.sort();

//         assert_eq!(expect, edges);
//     }
// }
