mod draw;
mod maze;
use draw::draw;
use maze::*;

use std::error::Error;

/// Modified version of https://github.com/Lakret/gir/tree/master/mazes

fn main() {
    /*
    let mut maze = Maze::new(5,5);
    maze.add_cell((0,0), CellType::Wall);
    maze.add_cell((0,1), CellType::Wall);
    maze.add_cell((0,2), CellType::Wall);
    maze.add_cell((0,3), CellType::Wall);
    maze.add_cell((0,4), CellType::Wall);

    maze.add_cell((1,0), CellType::Wall);
    maze.add_cell((1,1), CellType::Room);
    maze.add_cell((1,2), CellType::Room);
    maze.add_cell((1,3), CellType::Wall);
    maze.add_cell((1,4), CellType::Wall);

    maze.add_cell((2,0), CellType::Wall);
    maze.add_cell((2,1), CellType::Room);
    maze.add_cell((2,2), CellType::Wall);
    maze.add_cell((2,3), CellType::Wall);
    maze.add_cell((2,4), CellType::Wall);

    maze.add_cell((3,0), CellType::Wall);
    maze.add_cell((3,1), CellType::Room);
    maze.add_cell((3,2), CellType::Room);
    maze.add_cell((3,3), CellType::Room);
    maze.add_cell((3,4), CellType::Wall);

    maze.add_cell((4,0), CellType::Wall);
    maze.add_cell((4,1), CellType::Wall);
    maze.add_cell((4,2), CellType::Wall);
    maze.add_cell((4,3), CellType::Wall);
    maze.add_cell((4,4), CellType::Wall);

    let mut maze = Maze::default(10,10);
    maze.find_path((0,0),(1,1), true);
    */

    let mut maze = Maze::generate(30,30,0);
    maze.add_cell((maze.width()-1,maze.height()-1), CellType::Room);
    maze.find_path((0,0),(maze.width()-1,maze.height()-1), true);

    //println!("Maze: {:?}", maze);

    //let t = Instant::now();
    let document = draw(&maze);
    svg::save("image.svg", &document).unwrap();
    println!("Saved to SVG.");
    //println!("Saved to SVG in {:?}.", t.elapsed());
}


#[cfg(test)] //TODO
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn default() {
        let mut maze = Maze::new(2,2);
        maze.add_cell((0,0), CellType::Wall);
        maze.add_cell((0,1), CellType::Wall);
        assert_eq!(Maze::default(2,2);, maze);
    }
}