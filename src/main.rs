use rand::Rng;
use std::{io::stdin, thread, time::Duration, usize};

const WORLD_SIZE: usize = 30;
const FRAME_TIME: usize = 10;

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
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct World {
    population: u64,
    position: Position,
    space: Vec<Vec<Cell>>,
}

impl World {
    fn new(position: Position) -> Self {
        let mut space = Vec::new();

        for y in 0..position.y {
            space.push(Vec::new());

            for x in 0..position.x {
                space[y].push(Cell::new(x, y));
            }
        }

        return Self {
            population: 0,
            position,
            space,
        };
    }

    fn add_cell(&mut self, cell: Cell) {
        self.space[cell.position.y][cell.position.x] = cell;
        self.population += 1;
    }

    fn randon_populate_world(&mut self, mut population: u32) {
        while population > 0 {
            let rand_x_poss = rand::rng().random_range(0..self.position.x);
            let rand_y_poss = rand::rng().random_range(0..self.position.y);
            let current_cell = &mut self.space[rand_y_poss][rand_x_poss];
            if !current_cell.alive {
                current_cell.alive = true;
                population -= 1;
            }
        }
    }
}

trait WorldDriver {
    fn render(&self, world: &World);
    fn handle_input(&mut self, word: &mut World);
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
    fn render(&self, world: &World) {
        TerminalDriver::clean_terminal();
        TerminalDriver::show_horizontal_separator(world.position.x);
        for y in 0..world.position.y {
            print!("|");
            for x in 0..world.position.x {
                match world.space[y][x].alive {
                    true => print!("O"),
                    false => print!(" "),
                }
            }
            println!("|");
        }

        TerminalDriver::show_horizontal_separator(world.position.x);
    }

    fn sleep(&self) {
        thread::sleep(self.sleep_duration);
    }

    fn handle_input(&mut self, world: &mut World) {
        let mut response_buffer = String::new();

        println!("Do you want to add a new life? [y/n] ");
        stdin()
            .read_line(&mut response_buffer)
            .expect("failed to get response from terminal");

        if let Some(response) = response_buffer.trim_ascii().chars().nth(0) {
            if response.to_ascii_lowercase() != 'y' {
                return;
            };
        }

        response_buffer.clear();

        println!("Insert the X and Y position from the new life starts, Ex: 0 2");
        stdin()
            .read_line(&mut response_buffer)
            .expect("failed to get response from terminal");

        let coordenates = response_buffer
            .split_ascii_whitespace()
            .into_iter()
            .map(|char| {
                char.parse::<i32>()
                    .expect("failed to parse input to number")
            })
            .collect::<Vec<i32>>();

        if coordenates.len() < 2 {
            eprintln!("At least 2 coordenates shoud be provided");
            return;
        }

        world.add_cell(Cell::new_alive(
            coordenates[0] as usize,
            coordenates[1] as usize,
        ));
    }
}

fn main() {
    let mut world = World::new(Position {
        x: WORLD_SIZE,
        y: WORLD_SIZE,
    });

    world.randon_populate_world(20);

    let mut terminal_driver = TerminalDriver::new();

    loop {
        terminal_driver.render(&mut world);
        terminal_driver.handle_input(&mut world);
        terminal_driver.sleep();
    }
}
