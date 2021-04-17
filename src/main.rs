mod draw;
mod maze;
use draw::draw;
use maze::*;

use std::error::Error;


fn main() {
    let mut maze = Maze::new(3,3);
    maze.add_cell((0,0), CellType::Wall);
    maze.add_cell((0,1), CellType::Wall);
    maze.add_cell((0,2), CellType::Wall);

    maze.add_cell((1,0), CellType::Wall);
    maze.add_cell((1,1), CellType::Room);
    maze.add_cell((1,2), CellType::Wall);

    maze.add_cell((2,0), CellType::Wall);
    maze.add_cell((2,1), CellType::Wall);
    maze.add_cell((2,2), CellType::Wall);

    println!("Maze: {:?}", maze);

    //let t = Instant::now();
    let document = draw(&maze);
    svg::save("image.svg", &document).unwrap();
    println!("Saved to SVG in.");
    //println!("Saved to SVG in {:?}.", t.elapsed());
}
