#[macro_use]
extern crate stdweb;

macro_rules! println {
    ($($tt:tt)*) => {{
        let msg = format!($($tt)*);
        js! { console.log(@{ msg }) }
    }}
}


mod draw;

mod maze;
use maze::*;
//use std::env;
use draw::Canvas;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget};

/// Modified version of https://github.com/Lakret/gir/tree/master/mazes

fn main() {
    stdweb::initialize();

    let width: u32 = 25;
    let height: u32 = 25;
    let canvas = Canvas::new("#canvas", width, height);

    //let mut maze = Maze::default(width,height);
    let mut maze = Maze::generate(width,height,0);
    maze.add_cell((maze.width()-1,maze.height()-1), CellType::Room);
    maze.find_path((0,0),(maze.width()-1,maze.height()-1), true);
    println!("using arguments: {}x{}", maze.width(), maze.height());

    stdweb::web::document().add_event_listener({
        //let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => { println!("Left") },
                "ArrowRight" => { println!("Right") },
                "ArrowDown" => { println!("Down") },
                "ArrowUp" => { println!("Up") },
                _ => {}
            };
        }
    });


    //fn game_loop(maze: Rc<RefCell<Maze>>, canvas: Rc<Canvas>, time: u32) {
    fn game_loop(maze: Maze, canvas: Rc<Canvas>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(maze.clone(), canvas.clone(), time);
                //snake.borrow_mut().update();
                //snake.borrow().draw(&canvas);
                canvas.draw_maze(&maze);
            },
            time,
        );
    }

    //println!("Maze: {:?}", maze);
    //canvas.draw_maze(&maze);

    game_loop(maze, Rc::new(canvas), 100);
    //canvas.draw(0, 0, "red"); // DEBUG
    //canvas.draw(width-1, 0, "purple"); // DEBUG
    //canvas.draw(0, height-1, "green"); // DEBUG
    //canvas.draw(width-1, height-1, "orange"); // DEBUG
    //canvas.draw(width/2, height/2, "blue");
    stdweb::event_loop();
}
