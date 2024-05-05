// #![windows_subsystem = "windows"]
use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{
    prelude::*,
    rand::{self},
};

const WINDOW: (i32, i32) = (600, 600);
const ROWS: i32 = 4;
const COLS: i32 = 4;

const RECT_HEIGHT: i32 = WINDOW.0 / ROWS;
const RECT_WIDTH: i32 = WINDOW.1 / COLS;

const OUTLINE_COLOR: Color = Color {
    r: 187f32 / 255f32,
    g: 173f32 / 255f32,
    b: 160f32 / 255f32,
    a: 255f32 / 255f32,
};
const OUTLINE_THICKNESS: i32 = 10;
const BACKGROUND_COLOR: Color = Color {
    r: 205f32 / 255f32,
    g: 192f32 / 255f32,
    b: 180f32 / 255f32,
    a: 255f32 / 255f32,
};
const FONT_COLOR: Color = Color {
    r: 119f32 / 255f32,
    g: 110f32 / 255f32,
    b: 101f32 / 255f32,
    a: 255f32 / 255f32,
};

const MOVE_VEL: f32 = 2750.0;

fn conf() -> Conf {
    Conf {
        window_title: String::from("2048"),
        window_width: WINDOW.0,
        window_height: WINDOW.1,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

const TILE_COLORS: [Color; 10] = [
    Color {
        r: 237f32 / 255f32,
        g: 229f32 / 255f32,
        b: 218f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 238f32 / 255f32,
        g: 225f32 / 255f32,
        b: 201f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 243f32 / 255f32,
        g: 178f32 / 255f32,
        b: 122f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 246f32 / 255f32,
        g: 150f32 / 255f32,
        b: 101f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 247f32 / 255f32,
        g: 124f32 / 255f32,
        b: 95f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 247f32 / 255f32,
        g: 95f32 / 255f32,
        b: 59f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 237f32 / 255f32,
        g: 208f32 / 255f32,
        b: 115f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 237f32 / 255f32,
        g: 204f32 / 255f32,
        b: 99f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 237f32 / 255f32,
        g: 202f32 / 255f32,
        b: 80f32 / 255f32,
        a: 255f32 / 255f32,
    },
    Color {
        r: 61f32 / 255f32,
        g: 58f32 / 255f32,
        b: 51f32 / 255f32,
        a: 255f32 / 255f32,
    },
];

#[derive(Debug, Clone, Copy, PartialEq)]
struct Tile {
    value: i32,
    row: i32,
    col: i32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    dead: bool,
    merged: bool,
    freeze: bool,
}

impl Tile {
    fn new(value: i32, row: i32, col: i32) -> Self {
        Tile {
            value,
            row,
            col,
            x: (col * RECT_WIDTH) as f32,
            y: (row * RECT_HEIGHT) as f32,
            w: RECT_WIDTH as f32,
            h: RECT_HEIGHT as f32,
            dead: false,
            merged: false,
            freeze: false,
        }
    }

    fn get_color(&self) -> Color {
        let color_index = f32::log2(self.value as f32) - 1.0;
        if color_index >= 9f32 {
            return TILE_COLORS[9];
        }
        TILE_COLORS[color_index as usize]
    }

    fn draw(&self) {
        let color = self.get_color();
        draw_rectangle(self.x as f32, self.y as f32, self.w, self.h, color);
        let color = if f32::log2(self.value as f32) - 1.0 >= 2.0 {
            WHITE
        } else {
            FONT_COLOR
        };
        draw_text(
            format!("{}", self.value).as_str(),
            self.x + (self.w / 2.0 - (1.0 + f32::log10(self.value as f32)) * 9.0),
            self.y + (self.h as f32 / 2.0 + 10.0),
            42.0,
            color,
        )
    }

    fn set_pos(&mut self, row: i32, col: i32) {
        self.row = row;
        self.col = col;
    }

    fn update(&mut self, dt: f32) {
        if (self.w as i32) < RECT_WIDTH {
            let (prev_w, prev_h) = (self.w, self.h);
            self.w += MOVE_VEL * dt / 7.0;
            self.h += MOVE_VEL * dt / 7.0;
            self.x += (prev_w - self.w) / 2.0;
            self.y += (prev_h - self.h) / 2.0;
            return;
        } else {
            self.w = RECT_WIDTH as f32;
            self.h = RECT_WIDTH as f32;
        }

        if self.x as i32 > self.col * RECT_WIDTH {
            if self.x - (RECT_WIDTH as f32 * 0.25) < (self.col * RECT_WIDTH) as f32 {
                self.x = (self.col * RECT_WIDTH) as f32;
            } else {
                self.x -= MOVE_VEL * dt;
            }
        } else if (self.x as i32) < self.col * RECT_WIDTH {
            if self.x + (RECT_WIDTH as f32 * 0.25) > (self.col * RECT_WIDTH) as f32 {
                self.x = (self.col * RECT_WIDTH) as f32;
            } else {
                self.x += MOVE_VEL * dt;
            }
        } else if (self.y as i32) > self.row * RECT_HEIGHT {
            if self.y - (RECT_WIDTH as f32 * 0.25) < (self.row * RECT_HEIGHT) as f32 {
                self.y = (self.row * RECT_HEIGHT) as f32;
            } else {
                self.y -= MOVE_VEL * dt;
            }
        } else if (self.y as i32) < self.row * RECT_HEIGHT {
            if self.y + (RECT_WIDTH as f32 * 0.25) > (self.row * RECT_HEIGHT) as f32 {
                self.y = (self.row * RECT_HEIGHT) as f32;
            } else {
                self.y += MOVE_VEL * dt;
            }
        } else {
            self.x = (self.col * RECT_WIDTH) as f32;
            self.y = (self.row * RECT_HEIGHT) as f32;
        }
    }
}

fn draw_grid2d() {
    for row in 1..ROWS {
        let y = row * RECT_HEIGHT;
        draw_line(
            0.0,
            y as f32,
            WINDOW.0 as f32,
            y as f32,
            OUTLINE_THICKNESS as f32,
            OUTLINE_COLOR,
        );
    }

    for row in 1..COLS {
        let x = row * RECT_WIDTH;
        draw_line(
            x as f32,
            0.0,
            x as f32,
            WINDOW.1 as f32,
            OUTLINE_THICKNESS as f32,
            OUTLINE_COLOR,
        );
    }

    draw_rectangle_lines(
        0.0,
        0.0,
        WINDOW.0 as f32,
        WINDOW.1 as f32,
        (OUTLINE_THICKNESS * 2i32) as f32,
        OUTLINE_COLOR,
    );
}

fn get_random_pos(tiles: &Vec<(String, Tile)>) -> (usize, usize) {
    let mut row: usize;
    let mut col: usize;

    let x = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();
    rand::srand(x as u64);
    loop {
        row = rand::gen_range(0, ROWS as usize);
        col = rand::gen_range(0, COLS as usize);

        let mut devam = false;
        for (k, _) in tiles {
            if format!("{}{}", row, col).to_string() == *k {
                devam = true;
            }
        }
        if !devam {
            break;
        }
    }
    (row, col)
}

fn generate_tiles() -> Vec<(String, Tile)> {
    let mut tiles: Vec<(String, Tile)> = Vec::new();
    for _ in 0..2 {
        let (row, col): (usize, usize) = get_random_pos(&tiles);
        tiles.push((
            format!("{}{}", row, col).to_string(),
            Tile::new(2, row as i32, col as i32),
        ));
    }
    tiles
}

fn spawn_tile(tiles: &mut Vec<(String, Tile)>) {
    if tiles.len() >= (ROWS * COLS) as usize {
        return;
    }

    let (row, col): (usize, usize) = get_random_pos(&tiles);

    let rand = rand::gen_range(0, 9);
    let val;
    if rand == 0 {
        val = 4;
    } else{
        val = 2;
    }

    let mut tile = Tile::new(val, row as i32, col as i32);
    let (prev_w, prev_h) = (tile.w, tile.h);
    tile.w *= 0.5;
    tile.h *= 0.5;
    tile.x += (prev_w - tile.w) / 2.0;
    tile.y += (prev_h - tile.h) / 2.0;

    tiles.push((format!("{}{}", row, col).to_string(), tile));
}

fn find_index_of_string_in_vec(vec: &Vec<(String, Tile)>, search_str: &str) -> Option<usize> {
    for (index, &(ref s, _)) in vec.iter().enumerate() {
        if s.contains(search_str) {
            return Some(index);
        }
    }
    None
}

fn move_tiles(tiles: &mut Vec<(String, Tile)>, direction: String) -> bool {
    let mut dir = (0, 0);
    let mut moved = false;
    if direction == "left" {
        dir = (0, -1);
    } else if direction == "right" {
        dir = (0, 1);
    } else if direction == "up" {
        dir = (-1, 0);
    } else if direction == "down" {
        dir = (1, 0);
    }
    let mut index = 0;
    let mut count = 0;
    loop {
        if direction == "left" {
            tiles.sort_by(|a, b| a.1.col.cmp(&b.1.col));
        } else if direction == "right" {
            tiles.sort_by(|a, b| b.1.col.cmp(&a.1.col));
        } else if direction == "up" {
            tiles.sort_by(|a, b| a.1.row.cmp(&b.1.row));
        } else if direction == "down" {
            tiles.sort_by(|a, b| b.1.row.cmp(&a.1.row));
        }

        let tiles_copy = tiles.clone();

        let (x, y) = &mut tiles[index];

        let mut next_tile = format!("{}{}", y.row + dir.0, y.col + dir.1).to_string();
        let mut next_tile_index = 0;
        let mut next_tile_value = -1;
        let mut next_tile_merged = true;
        if let Some(index) = find_index_of_string_in_vec(&tiles_copy, &next_tile) {
            next_tile_index = index;
            next_tile_value = tiles_copy[next_tile_index].1.value;
            next_tile_merged = tiles_copy[next_tile_index].1.merged;
        }

        let mut can_move = true;
        while can_move {
            if direction == "left" {
                if y.col == 0 {
                    y.freeze = true;
                    break;
                }
            } else if direction == "right" {
                if y.col == COLS - 1 {
                    y.freeze = true;
                    break;
                }
            } else if direction == "up" {
                if y.row == 0 {
                    y.freeze = true;
                    break;
                }
            } else if direction == "down" {
                if y.row == ROWS - 1 {
                    y.freeze = true;
                    break;
                }
            }

            if y.merged || y.freeze {
                break;
            }

            for other in &tiles_copy {
                if other.0 == *x {
                    continue;
                }

                if next_tile == other.0 {
                    can_move = false;
                }
            }

            if can_move {
                *x = format!("{}{}", y.row + dir.0, y.col + dir.1).to_string();
                y.set_pos(y.row + dir.0, y.col + dir.1);
                moved = true;
                next_tile = format!("{}{}", y.row + dir.0, y.col + dir.1).to_string();
            } else if next_tile_value == y.value {
                if !next_tile_merged {
                    moved = true;
                    y.dead = true;
                    y.merged = true;
                    y.freeze = true;
                    *x = format!("{}{}", y.row + dir.0, y.col + dir.1).to_string();
                    y.set_pos(y.row + dir.0, y.col + dir.1);
                    tiles[next_tile_index].1.value *= 2;
                    tiles[next_tile_index].1.merged = true;
                    tiles[next_tile_index].1.freeze = true;
                }
                break;
            }
        }
        if index < tiles.len() - 1 {
            index += 1;
            continue;
        } else {
            index = 0;
            count += 1;
        }
        let mut cont = false;
        for tile in tiles.clone() {
            if !tile.1.freeze {
                cont = true;
                break;
            }
        }
        if !cont {
            break;
        }

        if count > tiles.len() {
            break;
        }
    }
    tiles.retain(|x| !x.1.dead);
    for tile in tiles {
        tile.1.merged = false;
        tile.1.freeze = false;
    }
    moved
}

#[macroquad::main(conf)]
async fn main() {
    let mut _tiles: Vec<(String, Tile)> = Vec::new();
    _tiles = generate_tiles();

    'game: loop {
        if is_key_pressed(KeyCode::Escape) {
            break 'game;
        } else if is_key_pressed(KeyCode::R) {
            _tiles = generate_tiles();
        } else if is_key_pressed(KeyCode::K) {
            spawn_tile(&mut _tiles);
        } else if is_key_pressed(KeyCode::Left) {
            if move_tiles(&mut _tiles, String::from("left")) {
                spawn_tile(&mut _tiles);
            }
        } else if is_key_pressed(KeyCode::Right) {
            if move_tiles(&mut _tiles, String::from("right")) {
                spawn_tile(&mut _tiles);
            }
        } else if is_key_pressed(KeyCode::Up) {
            if move_tiles(&mut _tiles, String::from("up")) {
                spawn_tile(&mut _tiles);
            }
        } else if is_key_pressed(KeyCode::Down) {
            if move_tiles(&mut _tiles, String::from("down")) {
                spawn_tile(&mut _tiles);
            }
        }

        for tile in &mut _tiles {
            tile.1.update(get_frame_time());
        }

        clear_background(BACKGROUND_COLOR);

        // draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 48.0, 30.0, BLACK);

        for (_, tile) in &_tiles {
            tile.draw();
        }

        draw_grid2d();

        next_frame().await;
        std::thread::sleep(std::time::Duration::from_millis(3));
    }
}
