use rand::Rng;
use std::{io::stdin, thread, time::Duration};

const WORLD_SIZE: usize = 30;
const FRAME_TIME: usize = 10 / 1;

#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool,
    x: usize,
    y: usize,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Self { alive: false, x, y }
    }

    fn new_alive(x: usize, y: usize) -> Self {
        Self { alive: true, x, y }
    }
}

struct Dimensions {
    x: usize,
    y: usize,
}

struct World {
    dimension: Dimensions,
    space: Vec<Vec<Cell>>,
}

impl World {
    fn new(dimensions: Dimensions) -> Self {
        let mut space = Vec::new();

        for y in 0..dimensions.y {
            space.push(Vec::new());

            for x in 0..dimensions.x {
                space[y].push(Cell::new(x, y));
            }
        }

        return Self {
            dimension: dimensions,
            space: space,
        };
    }

    fn terminal_render(&self) {
        let separator = "-";
        let separator = separator.repeat(self.dimension.x);
        print!("+");
        print!("{separator}");
        println!("+");

        for y in 0..self.dimension.y {
            print!("|");
            for x in 0..self.dimension.x {
                match self.space[y][x].alive {
                    true => print!("O"),
                    false => print!(" "),
                }
            }

            println!("|");
        }

        print!("+");
        print!("{separator}");
        println!("+");

        thread::sleep(Duration::from_millis(FRAME_TIME as u64));
    }

    fn add_cell(&mut self, cell: Cell) {
        self.space[cell.y][cell.x] = cell
    }

    fn clean_terminal(&self) {
        print!("{}[2J", 27 as char);
    }

    fn manage_god_input(&mut self) {
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

        self.add_cell(Cell::new_alive(
            coordenates[0] as usize,
            coordenates[1] as usize,
        ));
    }

    fn randon_populate_world(&mut self, mut population: u32) {
        while population > 0 {
            let rand_x_poss = rand::rng().random_range(0..self.dimension.x);
            let rand_y_poss = rand::rng().random_range(0..self.dimension.y);
            let current_cell = &mut self.space[rand_y_poss][rand_x_poss];
            if !current_cell.alive {
                current_cell.alive = true;
                population -= 1;
            }
        }
    }
}

fn main() {
    let mut world = World::new(Dimensions {
        x: WORLD_SIZE,
        y: WORLD_SIZE,
    });

    world.randon_populate_world(20);

    loop {
        world.clean_terminal();
        world.add_cell(Cell::new_alive(1, 1));
        world.terminal_render();
        world.clean_terminal();
    }
}
