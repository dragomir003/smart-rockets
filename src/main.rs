mod population;
mod rocket;

use std::error::Error;
use std::rc::Rc;

use simple_game_engine::{ self as sge, Application, WindowCanvas, input::InputState };

use population::Population;
use rocket::Rocket;

pub static WINDOW_DIMENSIONS: (u32, u32) = (500, 500);

#[derive(Debug)]
struct App {
    goal: Rc<sge::Point>,
    population: Population<Rocket>,
}

impl App {
    fn new() -> Self {
        let (width, _) = WINDOW_DIMENSIONS;
        Self {
            goal: Rc::new(sge::Point::new(width as i32 / 2, 20)),
            population: vec![].into(),
        }
    }
}

impl Application for App {
    fn on_create(&mut self, _canvas: &mut sge::canvas::WindowCanvas, _input: &sge::input::InputState) -> sge::ApplicationResult {
        // Generate random rockets
        let rockets = vec![];
        self.population = rockets.into();

        Ok(true)
    }

    fn on_update(
            &mut self,
            canvas: &mut WindowCanvas,
            _input: &InputState,
            _elapsed_time: f64,
        ) -> sge::ApplicationResult {

        canvas.set_draw_color(sge::Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(sge::Color::BLUE);
        canvas.fill_rect(sge::Rect::new(self.goal.x, self.goal.y, 10, 10))?;

        for _rocket in self.population.get() {
            // Render each rocket
        }

        if self.population.update() {
            self.population.restart();
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
