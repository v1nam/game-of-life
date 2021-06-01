use sfml::{
    graphics::{Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Transformable},
    system::Vector2f,
    window::{mouse, Event, Key, Style},
};
use std::{ops::Not, thread, time};

#[derive(Copy, Clone)]
enum State {
    Dead,
    Alive,
}

impl Not for State {
    // just for the funsies
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::Dead => State::Alive,
            State::Alive => State::Dead,
        }
    }
}

const WIN_W: usize = 800;
const WIN_H: usize = 600;
type Cells = [[State; WIN_W / 8]; WIN_H / 8];

fn main() {
    let mut window = RenderWindow::new(
        (WIN_W as u32, WIN_H as u32),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut cells: Cells = [[State::Dead; WIN_W / 8]; WIN_H / 8];
    let mut paused = true;

    loop {
        window.clear(Color::BLACK);
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
                }
                _ => {}
            }
        }

        if window.has_focus() {
            let pos = window.mouse_position();
            if (pos.y as usize) < WIN_H && (pos.x as usize) < WIN_W {
                let (y, x) = ((pos.y / 8) as usize, (pos.x / 8) as usize);
                if mouse::Button::LEFT.is_pressed() {
                    cells[y][x] = State::Alive;
                } else if mouse::Button::RIGHT.is_pressed() {
                    cells[y][x] = State::Dead;
                }
            }
        }

        if !paused {
            update_cells(&mut cells);
        }

        for row in 0..cells.len() {
            for col in 0..cells[0].len() {
                match cells[row][col] {
                    State::Alive => {
                        let mut rect = RectangleShape::default();
                        rect.set_size(Vector2f::new(8., 8.));
                        rect.set_position(((col * 8) as f32, (row * 8) as f32));
                        window.draw(&rect);
                    }
                    State::Dead => {}
                }
            }
        }
        window.display();
        window.set_framerate_limit(10);
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
