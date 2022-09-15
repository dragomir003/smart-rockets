mod population;
mod rocket;

use std::{error::Error, io::Write};
use std::rc::Rc;

use simple_game_engine::{self as sge,
                         prelude::*,
                         input::InputState,
                         Application,
                         WindowCanvas};

use population::Population;
use rocket::Rocket;

pub static WINDOW_DIMENSIONS: (u32, u32) = (500, 500);

#[derive(Debug)]
struct App {
    goal: Rc<sge::Point>,
    population: Population<Rocket>,

    generation_count: usize,
    is_paused: bool,
}

impl App {
    fn new() -> Self {
        let (width, _) = WINDOW_DIMENSIONS;
        Self {
            goal: Rc::new(sge::Point::new(width as i32 / 2, 20)),
            population: vec![].into(),
            generation_count: 1,
            is_paused: false,
        }
    }
}

impl Application for App {
    fn on_create(
        &mut self,
        _canvas: &mut sge::canvas::WindowCanvas,
        _input: &sge::input::InputState,
    ) -> sge::ApplicationResult {
        let (_, height) = WINDOW_DIMENSIONS;
        // Generate random rockets
        let rockets = (1..50)
            .map(|_| {
                Rocket::randomize(
                    sge::Point::new(10, height as i32 - 10),
                    Rc::clone(&self.goal),
                    30,
                )
            })
            .collect::<Vec<_>>();

        self.population = rockets.into();

        Ok(true)
    }

    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        input: &InputState,
        _elapsed_time: f64,
    ) -> sge::ApplicationResult {
        if input.keyboard.pressed(Scancode::P) {
            self.is_paused = !self.is_paused;
        } else if input.keyboard.pressed(Scancode::Q) {
            return Ok(false);
        }

        if self.is_paused {
            return Ok(true);
        }

        canvas.set_draw_color(sge::Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(sge::Color::GREEN);
        canvas.fill_rect(sge::Rect::new(self.goal.x, self.goal.y, 10, 10))?;

        print!("Generation {}\r", self.generation_count);
        std::io::stdout().flush()?;

        canvas.set_draw_color(sge::Color::BLUE);
        for rocket in self.population.get() {
            canvas.fill_rect(sge::Rect::new(rocket.coords.x, rocket.coords.y, 10, 10))?;
        }

        if self.population.update() {
            self.population.restart();
            use std::time::Duration;
            std::thread::sleep(Duration::from_secs(2));
            self.generation_count += 1;
        }

        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let (width, height) = WINDOW_DIMENSIONS;

    let mut engine = sge::Engine::new(&mut app, "Smart Rockets", width, height)?;

    engine.start(true)
}
