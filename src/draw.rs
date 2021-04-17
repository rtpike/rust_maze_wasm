
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::Document;

use crate::maze::{Maze, Cell, CellType};

const CELL_SIDE: u32 = 30;
const STROKE_WIDTH: u32 = 29;

pub fn draw(maze: &Maze) -> Document {
    let mut paths = vec![];

    for row in 0..maze.height() {
        for col in 0..maze.width() {
            let cell = (row, col);
            add_cell_paths(&mut paths, &maze, cell, &maze[cell]);
        }
    }

    let (width, height) = (maze.width() * CELL_SIDE, maze.height() * CELL_SIDE);
    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("style", "background-color: white;");
    paths.into_iter().fold(document, |document, path| document.add(path))
}

fn make_line(from: (u32, u32), relative_to: (u32, u32), color: &str) -> Path {
    let data = Data::new().move_to(from).line_by(relative_to);

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-linejoin", "square")
        .set("stroke-linecap", "square")
        .set("d", data)
}

fn make_rect(from: (u32, u32), relative_to: (u32, u32), color: &str) -> Rectangle {
    let (x,y ) = from;

    Rectangle::new()
        .set("fill", color)
        //.set("stroke", color)
        .set("width", STROKE_WIDTH)
        .set("height", STROKE_WIDTH)
        .set("x", x)
        .set("y", y)
}

fn add_cell_paths(paths: &mut Vec<Rectangle>, maze: &Maze, (row, col): Cell, cell_type: &CellType) {
    let left_corner = (col * CELL_SIDE, row * CELL_SIDE);
    let (left_corner_x, left_corner_y) = left_corner;

    match cell_type {
        CellType::Wall => {
            let path = make_rect(left_corner, (0, 0), "black");
            paths.push(path)
        }
        CellType::Room => {
            let path = make_rect(left_corner, (0, 0), "gray");
            paths.push(path)
        }
    };

}
