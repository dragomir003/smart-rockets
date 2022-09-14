use std::rc::Rc;

use rand::random;
use simple_game_engine as sge;

use crate::population::Phenotype;
use crate::WINDOW_DIMENSIONS as window;

type Vector = sge::Point;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RocketState {
    Running,
    HitWall,
    HitTarget,
}

/// Represents a rocket that is trying to find the goal.
#[derive(Debug, Clone)]
pub struct Rocket {
    pub coords: sge::Point,
    start: sge::Point,

    goal: Rc<sge::Point>,

    dna: Vec<Vector>,
    current: usize,

    state: RocketState,
}

impl Rocket {
    pub fn randomize(start: sge::Point, goal: Rc<sge::Point>, dna_len: usize) -> Self {
        let dna = (0..dna_len)
            .map(|_| sge::Point::new(random::<i32>() % 15, random::<i32>() % 15))
            .collect::<Vec<_>>();

        Self {
            coords: start,
            start,
            goal: Rc::clone(&goal),
            dna,
            current: 0,
            state: RocketState::Running,
        }
    }
}

impl Phenotype for Rocket {
    fn update(&mut self) -> bool {
        if self.current == self.dna.len() {
            return true;
        }

        if self.state == RocketState::Running {
            let offset = self.dna[self.current];
            self.coords = self.coords.offset(offset.x, offset.y);

            let (width, height) = window;

            let (x, y) = (self.coords.x, self.coords.y);
            let (gx, gy) = (self.goal.x, self.goal.y);

            if (x >= gx - 5 && x <= gx + 5) && (y >= gy - 5 && y <= gy + 5) {
                self.state = RocketState::HitTarget;
            } else if x < 0 || x > width as i32 ||
                      y < 0 || x > height as i32 {
                self.state = RocketState::HitWall;
            }
        }

        self.current += 1;

        false
    }

    fn calculate_fitness(&self) -> f32 {
        let calc_dist = |a: &sge::Point, b: &sge::Point| {
            let dx = (a.x - b.x) as f32;
            let dy = (a.y - b.y) as f32;

            (dx * dx + dy * dy).sqrt()
        };

        let dist = calc_dist(&self.coords, Rc::as_ref(&self.goal));

        match self.state {
            RocketState::Running => 1.0 / dist,
            RocketState::HitWall => (1.0 / dist) / 1000.0,
            RocketState::HitTarget => 1000.0
        }
    }

    fn mutate(&mut self, ammount: f32) {
        self.dna.iter_mut().for_each(|v| {
            let chance: f32 = random();
            if chance > ammount { return; }

            *v = sge::Point::new(random::<i32>() % 30, random::<i32>() % 30);
        });

    }

    fn crossover(&self, other: &Self) -> Self {

        let mut dna = Vec::with_capacity(self.dna.capacity());
        for i in 0..self.dna.len() {
            if random() {
                dna.push(self.dna[i]);
            } else {
                dna.push(other.dna[i]);
            }
        }

        let current = 0;
        let goal = Rc::clone(&self.goal);
        let start = self.start;
        let coords = self.start;

        Self {
            coords,
            start,
            goal,
            dna,
            current,
            state: RocketState::Running,
        }
    }
    fn from(other: &Self) -> Self {
        Self {
            coords: other.start,
            start: other.start,
            goal: Rc::clone(&other.goal),
            current: 0,
            state: RocketState::Running,
            dna: other.dna.clone(),
        }
    }
}

