
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

use crate::maze::{Maze, Cell, CellType};

const CELL_SIDE: u32 = 30;
const STROKE_WIDTH: u32 = 29;

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.ctx.set_fill_style_color(color);

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );
    }

    #[allow(dead_code)]
    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.width * self.scaled_width),
            f64::from(self.height * self.scaled_height),
        );
    }



    pub fn draw_maze(&self, maze: &Maze) {
        //let mut paths = vec![];

        for w in 0..maze.width() {
            for h in 0..maze.height() {
                let cell= (w, h);
                self.add_cell_paths(cell, &maze[cell]);
            }
        }

        // let (width, height) = (maze.width() * CELL_SIDE, maze.height() * CELL_SIDE);
        // let document = Document::new()
        //     .set("viewBox", (0, 0, width, height))
        //     .set("style", "background-color: white;");
        // paths.into_iter().fold(document, |document, path| document.add(path))
    }


    fn make_rect(&self, from: (u32, u32), color: &str) {
        let (x,y ) = from;
        assert!(x < self.width);
        assert!(y < self.height);

        self.ctx.set_fill_style_color(color);

        let x = x * self.scaled_width;
        let y = y * self.scaled_height;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_width),
            f64::from(self.scaled_height),
        );
    }

    fn add_cell_paths(&self, (x, y): Cell, cell_type: &CellType) {
        let left_corner = (x,  y);

        match cell_type {
            CellType::Wall => {
                let path = self.make_rect(left_corner,  "black");
                //paths.push(path)
            }
            CellType::Room => {
                let path = self.make_rect(left_corner, "gray");
                //paths.push(path)
            }
            CellType::Path => {
                let path = self.make_rect(left_corner, "blue");
                //paths.push(path)
            }
        };
    }

}
