use web_sys::CanvasRenderingContext2d;

use crate::algo::kruskal;

pub const UP: u8 = 1 << 0;
pub const RIGHT: u8 = 1 << 1;
pub const DOWN: u8 = 1 << 2;
pub const LEFT: u8 = 1 << 3;

// 行(row) x 列(col)のグリッドについて、kruskal法で生成した最小全域木の
// 使用した辺=通路として、セルごとに開いている方向をビットマスクで返す。
// このビットマスクは draw_walls での描画とプレイ時の移動判定の両方で使う。
pub fn generate_walls(row: usize, col: usize) -> Vec<u8> {
    let mut walls = vec![0u8; row * col];
    if row == 0 || col == 0 {
        return walls;
    }

    let used_edges =
        kruskal::extract_maze_edges_by_kruskal(col, row, 1, kruskal::KruskalResultEdge::Used);

    for (a, b) in used_edges {
        let idx_a = a.x * col + a.y;
        let idx_b = b.x * col + b.y;
        if a.x == b.x {
            if a.y + 1 == b.y {
                walls[idx_a] |= RIGHT;
                walls[idx_b] |= LEFT;
            } else if b.y + 1 == a.y {
                walls[idx_b] |= RIGHT;
                walls[idx_a] |= LEFT;
            }
        } else if a.y == b.y {
            if a.x + 1 == b.x {
                walls[idx_a] |= DOWN;
                walls[idx_b] |= UP;
            } else if b.x + 1 == a.x {
                walls[idx_b] |= DOWN;
                walls[idx_a] |= UP;
            }
        }
    }

    walls
}

// walls ビットマスクから内壁を描画する。外周は呼び出し側で rect を描く想定。
// 各セルの右側・下側だけを見ることで、隣接セルとの二重描画を避ける。
pub fn draw_walls(
    ctx: &CanvasRenderingContext2d,
    walls: &[u8],
    row: usize,
    col: usize,
    space: f64,
) {
    for r in 0..row {
        for c in 0..col {
            let cell = walls[r * col + c];
            if c + 1 < col && cell & RIGHT == 0 {
                let x = (c + 1) as f64 * space;
                ctx.move_to(x, r as f64 * space);
                ctx.line_to(x, (r + 1) as f64 * space);
            }
            if r + 1 < row && cell & DOWN == 0 {
                let y = (r + 1) as f64 * space;
                ctx.move_to(c as f64 * space, y);
                ctx.line_to((c + 1) as f64 * space, y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_cell_has_at_least_one_open_direction() {
        let row = 6;
        let col = 5;
        let walls = generate_walls(row, col);

        assert_eq!(row * col, walls.len());
        for cell in walls {
            assert_ne!(0, cell, "spanning tree should connect every cell");
        }
    }

    #[test]
    fn open_directions_are_mutually_consistent() {
        let row = 4;
        let col = 4;
        let walls = generate_walls(row, col);

        for r in 0..row {
            for c in 0..col {
                let cell = walls[r * col + c];
                if cell & RIGHT != 0 {
                    assert!(c + 1 < col);
                    assert_ne!(0, walls[r * col + c + 1] & LEFT);
                }
                if cell & DOWN != 0 {
                    assert!(r + 1 < row);
                    assert_ne!(0, walls[(r + 1) * col + c] & UP);
                }
            }
        }
    }

    #[test]
    fn zero_sized_grid_returns_empty() {
        assert_eq!(0, generate_walls(0, 5).len());
        assert_eq!(0, generate_walls(5, 0).len());
    }
}
