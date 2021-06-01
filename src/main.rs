use sfml::{
    graphics::{Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Transformable, Shape},
    system::Vector2f,
    window::{mouse, Event, Key, Style},
};
use std::{ops::Not, thread, time};

#[derive(Copy, Clone)]
enum State {
    Dead,
    Alive,
}


const WIN_W: usize = 800;
const WIN_H: usize = 600;
const CELL_SIZE: usize = 10;
type Cells = [[State; WIN_W / CELL_SIZE]; WIN_H / CELL_SIZE];

fn main() {
    let mut window = RenderWindow::new(
        (WIN_W as u32, WIN_H as u32),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut cells: Cells = [[State::Dead; WIN_W / CELL_SIZE]; WIN_H / CELL_SIZE];
    let mut paused = true;
    let mut framerate: f32 = 14.;

    loop {
        window.clear(Color::WHITE);
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => return,
                Event::KeyPressed {
                    code: Key::SPACE, ..
                } => {
                    paused = !paused;
                    framerate = 14.;
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    if !paused {
                        if framerate + delta > 12. && framerate + delta < 80. { framerate += delta; } 
                    }
                }
                _ => {}
            }
        }

        if window.has_focus() {
            let pos = window.mouse_position();
            if (pos.y as usize) < WIN_H && (pos.x as usize) < WIN_W {
                let (y, x) = ((pos.y / CELL_SIZE as i32) as usize, (pos.x / CELL_SIZE as i32) as usize);
                if mouse::Button::LEFT.is_pressed() {
                    cells[y][x] = State::Alive;
                } else if mouse::Button::RIGHT.is_pressed() {
                    cells[y][x] = State::Dead;
                }
            }
        }

        if !paused {
            update_cells(&mut cells);
        } else {
            framerate = 40.;
        }

        for row in 0..cells.len() {
            for col in 0..cells[0].len() {
                match cells[row][col] {
                    State::Alive => {
                        let mut rect = RectangleShape::default();
                        rect.set_size(Vector2f::new((CELL_SIZE - 2) as f32, (CELL_SIZE - 2) as f32));
                        rect.set_position(((col * CELL_SIZE) as f32, (row * CELL_SIZE) as f32));
                        rect.set_fill_color(Color::rgb(238, 244, 255));
                        rect.set_outline_thickness(2.);
                        rect.set_outline_color(Color::rgb(223, 236, 255));
                        window.draw(&rect);
                    }
                    State::Dead => {}
                }
            }
        }
        window.display();
        window.set_framerate_limit(framerate as u32);
    }
}

fn adjacents(cells: &Cells, row: &usize, col: &usize) -> usize {
    let mut adjc = 0;
    let alignments = [
        (-1, 0),
        (-1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (-1, 1),
    ];

    for (i, j) in &alignments {
        let r = *row as i32 + i;
        let c = *col as i32 + j;
        if r >= 0 && r < cells.len() as i32 && c >= 0 && c < cells[0].len() as i32 {
            match cells[r as usize][c as usize] {
                State::Alive => {
                    adjc += 1;
                }
                State::Dead => {}
            }
        }
    }
    return adjc as usize;
}

fn update_cells(cells: &mut Cells) {
    let mut keep_alive: Vec<(usize, usize)> = Vec::new();
    let mut dead: Vec<(usize, usize)> = Vec::new();
    for row in 0..cells.len() {
        for col in 0..cells[0].len() {
            let adjc = adjacents(&cells, &row, &col);
            match cells[row][col] {
                State::Alive => {
                    if adjc < 2 || adjc > 3 {
                        dead.push((row, col));
                    } else {
                        keep_alive.push((row, col));
                    }
                }
                State::Dead => {
                    if adjc == 3 {
                        keep_alive.push((row, col));
                    }
                }
            }
        }
    }

    for (r, c) in keep_alive {
        cells[r][c] = State::Alive;
    }
    for (r, c) in dead {
        cells[r][c] = State::Dead;
    }
}
