use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View, FloatRect},
    system::Vector2f,
    window::{mouse, Event, Key, Style},
};

use std::collections::HashSet;

const WIN_W: u32 = 800;
const WIN_H: u32 = 600;
const CELL_SIZE: usize = 10;

fn main() {
    let mut window = RenderWindow::new(
        (WIN_W, WIN_H),
        "Game Of Life",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut alive_cells: HashSet<(u32, u32)> = HashSet::new();
    let mut paused = true;
    let mut framerate: f32 = 14.;

//    let mut view = View::from_rect(&FloatRect::from_vecs(Vector2f::new(0., 0.), Vector2f::new(WIN_W as f32, WIN_H as f32)));

    loop {
        window.clear(Color::rgb(241, 250, 238));
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
                Event::KeyPressed { code: Key::X, .. } => {
                    alive_cells = HashSet::new();
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    // if Key::LCONTROL.is_pressed() {}

                    if !paused {
                        if framerate + delta > 12. && framerate + delta < 80. {
                            framerate += delta;
                        }
                    }
                }
                _ => {}
            }
        }

        if window.has_focus() {
            let pos = window.mouse_position();
            if (pos.y as u32) < WIN_H && (pos.x as u32) < WIN_W {
                let pos = (
                    (pos.x / CELL_SIZE as i32) as u32,
                    (pos.y / CELL_SIZE as i32) as u32,
                );
                if mouse::Button::LEFT.is_pressed() {
                    alive_cells.insert(pos);
                } else if mouse::Button::RIGHT.is_pressed() {
                    alive_cells.remove(&pos);
                }
            }
        }

        if !paused {
            update_cells(&mut alive_cells);
        } else {
            framerate = 40.;
        }

        for (x, y) in &alive_cells {
            let x = *x as usize;
            let y = *y as usize;
            let mut rect = RectangleShape::default();
            rect.set_size(Vector2f::new(CELL_SIZE as f32 - 2., CELL_SIZE as f32 - 2.));
            rect.set_position(((x * CELL_SIZE) as f32, (y * CELL_SIZE) as f32));
            rect.set_fill_color(Color::rgb(168, 218, 220));
            rect.set_outline_thickness(2.);
            rect.set_outline_color(Color::rgb(69, 123, 157));
            window.draw(&rect);
        }
        window.display();
        window.set_framerate_limit(framerate as u32);
    }
}

fn adjacents(x: &u32, y: &u32) -> HashSet<(u32, u32)> {
    let mut adjc: HashSet<(u32, u32)> = HashSet::new();
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
        let r = *y as i32 + i;
        let c = *x as i32 + j;
        adjc.insert((c as u32, r as u32));
    }
    adjc
}

fn count_living(cells: &HashSet<(u32, u32)>, x: &u32, y: &u32) -> u32 {
    adjacents(x, y).iter().map(|pos| match cells.get(pos) { Some(_l) => 1u32, None => 0u32 }).sum::<u32>()
}

fn update_cells(cells: &mut HashSet<(u32, u32)>) {
    let mut keep_alive: HashSet<(u32, u32)> = HashSet::new();
    let mut dead: HashSet<(u32, u32)> = HashSet::new();
    for (x, y) in cells.iter() {
        let x = *x as u32;
        let y = *y as u32;
        if !(2..4).contains(&count_living(&cells, &x, &y)) {
            dead.insert((x, y));
        }
        let adjc = adjacents(&x, &y);
        for (a_x, a_y) in adjc.iter() {
            match cells.get(&(*a_x, *a_y)) {
                Some(_c) => {}
                None => {
                    if count_living(&cells, a_x, a_y) == 3 {
                       keep_alive.insert((*a_x, *a_y));
                    }
                }
            }
        }
    }

    *cells = &(&*cells - &dead) | &keep_alive;
}
