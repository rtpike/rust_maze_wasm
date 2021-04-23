
use svg::node::element::{Rectangle,Text} ;
use svg::Document;

use crate::maze::{Maze, Cell, CellType};

const CELL_SIDE: u32 = 30;
const STROKE_WIDTH: u32 = 29;

pub fn draw(maze: &Maze) -> Document {
    let mut paths = vec![];

    for w in 0..maze.width() {
        for h in 0..maze.height() {
            let cell= (w, h);
            add_cell_paths(&mut paths, cell, &maze[cell]);
        }
    }

    let (width, height) = (maze.width() * CELL_SIDE, maze.height() * CELL_SIDE);
    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("style", "background-color: white;");
    paths.into_iter().fold(document, |document, path| document.add(path))
}

fn make_rect(from: (u32, u32), color: &str) -> Rectangle {
    let (x,y ) = from;

    Rectangle::new()
        .set("fill", color)
        //.set("stroke", color)
        .set("width", STROKE_WIDTH)
        .set("height", STROKE_WIDTH)
        .set("x", x)
        .set("y", y)
}

#[allow(dead_code)]
fn make_text(from: (u32, u32), color: &str) -> Text {
    let (x,y ) = from;
    Text::new()
        .set("text", "(0,0)")
        .set("x", x)
        .set("y", y)
        .set("fill", color)
}


fn add_cell_paths(paths: &mut Vec<Rectangle>, (x, y): Cell, cell_type: &CellType) {
    let left_corner = (x * CELL_SIDE, y * CELL_SIDE);

    match cell_type {
        CellType::Wall => {
            let path = make_rect(left_corner,  "black");
            paths.push(path)
        }
        CellType::Room => {
            let path = make_rect(left_corner, "gray");
            paths.push(path)
        }
        CellType::Path => {
            let path = make_rect(left_corner, "blue");
            paths.push(path)
        }
    };
}
