mod draw;
mod maze;
use draw::draw;
use maze::*;
use std::env;

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

    let args: Vec<String> = env::args().collect();
    let mut filename: &str = "image.svg";
    let mut width: u32 = 32;
    let mut height: u32 = 32;
    let mut parse_err = false;

    println!("{:?}", args);
    match args.len() {
        1 => println!("using defaults arguments: {}x{} {}", width, height, filename),
        3 => {
            if let Ok(w) = args[1].parse::<u32>() {width=w;} else {
                println!("Invalid width");
                parse_err = true;
            }
            if let Ok(h) = args[2].parse::<u32>() {height=h;} else {
                println!("Invalid height");
                parse_err = true;
            }
        },
        4 => {
            filename = args[3].as_str();
        },
        _ => {
            parse_err = true;
        },
    }

    if parse_err {
        println!("Usage: {} <width> <height> [filename]", args[0]);
        std::process::exit(1);
    }

    println!("using arguments: {}x{} {}", width, height, filename);

    let mut maze = Maze::generate(width,height,0);
    maze.add_cell((maze.width()-1,maze.height()-1), CellType::Room);
    maze.find_path((0,0),(maze.width()-1,maze.height()-1), true);

    //println!("Maze: {:?}", maze);

    //let t = Instant::now();
    let document = draw(&maze);
    svg::save(filename, &document).unwrap();
    println!("Saved to SVG.");
    //println!("Saved to SVG in {:?}.", t.elapsed());
}


