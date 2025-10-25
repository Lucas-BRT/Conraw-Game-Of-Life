use rand::Rng;
use std::{thread, time::Duration, usize};

const INITIAL_POPULATION: u64 = 2000;
const FRAME_TIME: usize = 100;

#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool,
    position: Position,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Self {
            alive: false,
            position: Position { x, y },
        }
    }

    fn new_alive(x: usize, y: usize) -> Self {
        Self {
            alive: true,
            position: Position { x, y },
        }
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn count_neighbor(&self, world: &World) -> u8 {
        let mut counter = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                let x = self.position.x as i32 + dx;
                let y = self.position.y as i32 + dy;

                if x >= 0 && y >= 0 {
                    if let Some(row) = world.space.get(y as usize) {
                        if let Some(cell) = row.get(x as usize) {
                            if cell.alive {
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }

        return counter;
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct World {
    start: bool,
    population: u64,
    size: Position,
    space: Vec<Vec<Cell>>,
}

impl World {
    fn new(size: Position) -> Self {
        let mut space = Vec::new();

        for y in 0..size.y {
            space.push(Vec::new());
            for x in 0..size.x {
                space[y].push(Cell::new(x, y));
            }
        }

        return Self {
            start: true,
            population: 0,
            size,
            space,
        };
    }

    fn add_cell(&mut self, cell: Cell) {
        self.space[cell.position.y][cell.position.x] = cell;
        self.population += 1;
    }

    fn randon_populate_world(&mut self, mut population: u32) {
        while population > 0 {
            let rand_x_poss = rand::rng().random_range(0..self.size.x);
            let rand_y_poss = rand::rng().random_range(0..self.size.y);
            let current_cell = &mut self.space[rand_y_poss][rand_x_poss];
            if !current_cell.alive {
                current_cell.alive = true;
                population -= 1;
            }
        }
    }

    fn tick(&mut self) {
        let mut space = self.space.clone();

        if !self.start && self.population == 0 {
            return;
        }

        for row in 0..self.size.y {
            for column in 0..self.size.x {
                let a = &mut space[row][column];
                let neighbor = a.count_neighbor(self);
                if a.alive {
                    if neighbor < 2 {
                        a.kill();
                    }

                    if neighbor > 3 {
                        a.kill();
                    }
                } else {
                    if neighbor == 3 {
                        a.alive = true;
                    }
                }
            }
        }

        self.space = space;
    }
}

trait WorldDriver {
    fn render(&self, world: &mut World);
    fn sleep(&self);
}

struct TerminalDriver {
    sleep_duration: Duration,
}

impl TerminalDriver {
    fn new() -> Self {
        Self {
            sleep_duration: Duration::from_millis(FRAME_TIME as u64),
        }
    }

    fn clean_terminal() {
        print!("{}[2J", 27 as char);
    }

    fn show_horizontal_separator(world_x_dimension: usize) {
        let separator = "-".repeat(world_x_dimension);
        print!("+");
        print!("{separator}");
        println!("+");
    }
}

impl WorldDriver for TerminalDriver {
    fn render(&self, world: &mut World) {
        TerminalDriver::clean_terminal();
        TerminalDriver::show_horizontal_separator(world.size.x);
        for y in 0..world.size.y {
            print!("|");
            for x in 0..world.size.x {
                match world.space[y][x].alive {
                    true => print!("O"),
                    false => print!(" "),
                }
            }
            println!("|");
        }

        TerminalDriver::show_horizontal_separator(world.size.x);
        world.tick();
    }

    fn sleep(&self) {
        thread::sleep(self.sleep_duration);
    }
}

fn main() {
    let dimensions = term_size::dimensions().expect("failed to get terminal dimensions");

    let mut world = World::new(Position {
        x: dimensions.0 - 2,
        y: dimensions.1 - 3,
    });

    world.randon_populate_world(INITIAL_POPULATION as u32);

    let terminal_driver = TerminalDriver::new();

    loop {
        terminal_driver.render(&mut world);
        terminal_driver.sleep();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_neighbors() {
        let dimensions = term_size::dimensions().expect("failed to get terminal dimensions");

        let mut world = World::new(Position {
            x: dimensions.0 - 2,
            y: dimensions.1 - 3,
        });

        let first_cell = Cell::new_alive(0, 0);
        world.add_cell(first_cell);
        world.add_cell(Cell::new_alive(0, 1));
        world.add_cell(Cell::new_alive(1, 0));
        world.add_cell(Cell::new_alive(1, 1));

        assert_eq!(first_cell.count_neighbor(&world), 3);
    }

    #[test]
    fn count_up_neighbors() {
        let dimensions = term_size::dimensions().expect("failed to get terminal dimensions");

        let mut world = World::new(Position {
            x: dimensions.0 - 2,
            y: dimensions.1 - 3,
        });

        let first_cell = Cell::new_alive(1, 1);
        world.add_cell(first_cell);
        world.add_cell(Cell::new_alive(0, 0));
        world.add_cell(Cell::new_alive(1, 0));
        world.add_cell(Cell::new_alive(2, 0));

        assert_eq!(first_cell.count_neighbor(&world), 3);
    }
}
