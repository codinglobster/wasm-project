use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn time(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn timeEnd(s: &str);
}

// 移除未使用的宏定义

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        Universe::new_with_size(64, 64)
    }

    pub fn new_with_size(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn new_empty(width: u32, height: u32) -> Universe {
        let cells = vec![Cell::Dead; (width * height) as usize];

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    // 性能测试方法
    pub fn tick_with_timing(&mut self) {
        time("rust-tick");
        self.tick();
        timeEnd("rust-tick");
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = match self.cells[idx] {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }

    pub fn set_cell(&mut self, row: u32, col: u32, cell: Cell) {
        let idx = self.get_index(row, col);
        self.cells[idx] = cell;
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::Dead;
        }
    }

    pub fn randomize(&mut self) {
        use js_sys::Math;
        for cell in &mut self.cells {
            *cell = if Math::random() < 0.5 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }
    }

    // 设置滑翔机图案
    pub fn set_glider(&mut self, start_row: u32, start_col: u32) {
        let glider_pattern = vec![
            (0, 1), (1, 2), (2, 0), (2, 1), (2, 2)
        ];
        
        for (row_offset, col_offset) in glider_pattern {
            let row = (start_row + row_offset) % self.height;
            let col = (start_col + col_offset) % self.width;
            self.set_cell(row, col, Cell::Alive);
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // 调整宇宙大小，保留现有细胞状态
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let mut new_cells = vec![Cell::Dead; (new_width * new_height) as usize];
        
        // 复制现有细胞到新的网格中
        let copy_width = self.width.min(new_width);
        let copy_height = self.height.min(new_height);
        
        for row in 0..copy_height {
            for col in 0..copy_width {
                let old_idx = self.get_index(row, col);
                let new_idx = (row * new_width + col) as usize;
                new_cells[new_idx] = self.cells[old_idx];
            }
        }
        
        self.width = new_width;
        self.height = new_height;
        self.cells = new_cells;
    }

    // 获取活细胞数量
    pub fn alive_count(&self) -> u32 {
        self.cells.iter().filter(|&&cell| cell == Cell::Alive).count() as u32
    }
}
