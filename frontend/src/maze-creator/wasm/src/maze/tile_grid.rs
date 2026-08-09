use web_sys::CanvasRenderingContext2d;

use super::wall_grid::{DOWN, LEFT, RIGHT, UP};

// 壁ビットマスクのグリッド(row x col)を、壁も床も同じ大きさのタイルとして
// 表現した (2*row+1) x (2*col+1) のタイルグリッドに変換する。
// 偶数インデックスの行・列は壁(または柱)、奇数インデックスは元のセル(床)を表す。
pub fn tile_dimensions(row: usize, col: usize) -> (usize, usize) {
    (2 * row + 1, 2 * col + 1)
}

pub fn build_tile_grid(walls: &[u8], row: usize, col: usize) -> Vec<bool> {
    let (tile_rows, tile_cols) = tile_dimensions(row, col);
    let mut tiles = vec![true; tile_rows * tile_cols];

    for r in 0..row {
        for c in 0..col {
            let cell = walls[r * col + c];
            let tr = 2 * r + 1;
            let tc = 2 * c + 1;

            tiles[tr * tile_cols + tc] = false;

            if cell & RIGHT != 0 {
                tiles[tr * tile_cols + (tc + 1)] = false;
            }
            if cell & DOWN != 0 {
                tiles[(tr + 1) * tile_cols + tc] = false;
            }
            if cell & LEFT != 0 {
                tiles[tr * tile_cols + (tc - 1)] = false;
            }
            if cell & UP != 0 {
                tiles[(tr - 1) * tile_cols + tc] = false;
            }
        }
    }

    tiles
}

pub fn draw_tiles(
    ctx: &CanvasRenderingContext2d,
    walls: &[u8],
    row: usize,
    col: usize,
    tile_size: f64,
) {
    let (tile_rows, tile_cols) = tile_dimensions(row, col);
    let tiles = build_tile_grid(walls, row, col);

    let width = tile_cols as f64 * tile_size;
    let height = tile_rows as f64 * tile_size;
    ctx.clear_rect(0.0, 0.0, width, height);

    for r in 0..tile_rows {
        for c in 0..tile_cols {
            let x = c as f64 * tile_size;
            let y = r as f64 * tile_size;
            if tiles[r * tile_cols + c] {
                draw_wall_tile(ctx, x, y, tile_size);
            } else {
                draw_floor_tile(ctx, x, y, tile_size);
            }
        }
    }
}

fn draw_floor_tile(ctx: &CanvasRenderingContext2d, x: f64, y: f64, size: f64) {
    ctx.set_fill_style_str("#f2efe8");
    ctx.fill_rect(x, y, size, size);
    ctx.set_stroke_style_str("#dedad0");
    ctx.set_line_width(1.0);
    ctx.stroke_rect(x + 0.5, y + 0.5, size - 1.0, size - 1.0);
}

fn draw_wall_tile(ctx: &CanvasRenderingContext2d, x: f64, y: f64, size: f64) {
    ctx.set_fill_style_str("#5d4a3a");
    ctx.fill_rect(x, y, size, size);

    let inset = (size * 0.15).max(1.0);
    if size - inset * 2.0 > 0.0 {
        ctx.set_fill_style_str("#7a624c");
        ctx.fill_rect(x + inset, y + inset, size - inset * 2.0, size - inset * 2.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::wall_grid::generate_walls;

    #[test]
    fn tile_grid_has_expected_dimensions() {
        let row = 4;
        let col = 5;
        let walls = generate_walls(row, col);
        let tiles = build_tile_grid(&walls, row, col);

        let (tile_rows, tile_cols) = tile_dimensions(row, col);
        assert_eq!(2 * row + 1, tile_rows);
        assert_eq!(2 * col + 1, tile_cols);
        assert_eq!(tile_rows * tile_cols, tiles.len());
    }

    #[test]
    fn cell_centers_are_always_floor() {
        let row = 4;
        let col = 5;
        let walls = generate_walls(row, col);
        let tiles = build_tile_grid(&walls, row, col);
        let (_, tile_cols) = tile_dimensions(row, col);

        for r in 0..row {
            for c in 0..col {
                let idx = (2 * r + 1) * tile_cols + (2 * c + 1);
                assert!(!tiles[idx], "cell ({r},{c}) center should be floor");
            }
        }
    }

    #[test]
    fn corners_are_always_wall() {
        let row = 4;
        let col = 5;
        let walls = generate_walls(row, col);
        let tiles = build_tile_grid(&walls, row, col);
        let (tile_rows, tile_cols) = tile_dimensions(row, col);

        for r in (0..tile_rows).step_by(2) {
            for c in (0..tile_cols).step_by(2) {
                assert!(tiles[r * tile_cols + c], "corner ({r},{c}) should be wall");
            }
        }
    }

    #[test]
    fn open_passage_produces_floor_tile_between_cells() {
        let row = 2;
        let col = 2;
        // 2x2の全域木は必ず3本の通路を持つので、少なくとも1つは通路タイルがあるはず。
        let walls = generate_walls(row, col);
        let tiles = build_tile_grid(&walls, row, col);
        let (_, tile_cols) = tile_dimensions(row, col);

        let has_any_open_wall_slot = (0..row).any(|r| {
            (0..col).any(|c| {
                let cell = walls[r * col + c];
                let tr = 2 * r + 1;
                let tc = 2 * c + 1;
                (cell & RIGHT != 0 && !tiles[tr * tile_cols + (tc + 1)])
                    || (cell & DOWN != 0 && !tiles[(tr + 1) * tile_cols + tc])
            })
        });

        assert!(has_any_open_wall_slot);
    }
}
