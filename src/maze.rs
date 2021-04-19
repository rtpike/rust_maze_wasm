use std::collections::HashMap;
use std::collections::HashSet;

use rand::Rng;
use rand::seq::SliceRandom;

use petgraph::Graph; // used for A*
use petgraph::graphmap::UnGraphMap;
use petgraph::dot::{Dot, Config};
//use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::data::FromElements;
use petgraph::algo::{dijkstra, min_spanning_tree, astar};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {

    fn len() -> usize {
        4
    }

    fn opposite(self) -> Directions {
        match self {
            Directions::Up => Directions::Down,
            Directions::Down => Directions::Up,
            Directions::Left => Directions::Right,
            Directions::Right => Directions::Left,
        }
    }

    fn rand() -> Directions {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..Directions::len()) {
            0 => Directions::Down,
            1 => Directions::Up,
            2 => Directions::Right,
            3 => Directions::Left,
            _ => Directions::Down,
        }
    }

    fn to_array() -> [Directions; 4] {
        [Directions::Down, Directions::Up, Directions::Right, Directions::Left]
    }

    fn index(index: usize) -> Directions  {
        match index % Directions::len() {
            0 => Directions::Down,
            1 => Directions::Up,
            2 => Directions::Right,
            3 => Directions::Left,
            _ => panic!("Directions Enum at {:?} doesn't exist.", index),
        }
    }

    fn get_index(&self) -> usize  {
        match &self {
            Directions::Down  => 0,
            Directions::Up    => 1,
            Directions::Right => 2,
            Directions::Left  => 3,
        }
    }


}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CellType {
    Room,
    Wall,
    Path,
}

pub type Cell = (u32, u32);

#[derive(Debug, Clone)]
pub struct Maze {
    width: u32,
    height: u32,
    walls: HashMap<Cell, CellType>,
}
/// Maze DOCS (TODO)
impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        let walls = HashMap::new();
        Maze{width, height, walls}
    }

    /// empty maze with side walls only
    pub fn default(width: u32, height: u32) -> Maze {
        let walls = HashMap::new();
        let mut maze = Maze{width, height, walls};
        for x in 0..width {
            for y in 0..height {
                if x == 0 || x == width-1 || y == 0 || y == height-1 {
                    maze.add_cell((x,y), CellType::Wall);
                } else {
                    maze.add_cell((x,y), CellType::Room);
                }
            }
        }
        maze
    }

     fn filled(width: u32, height: u32) -> Maze {
        let walls = HashMap::new();
        let mut maze = Maze{width, height, walls};
        for x in 0..width {
            for y in 0..height {
                maze.add_cell((x,y), CellType::Wall);
            }
        }
        maze
    }

    pub fn add_cell(&mut self, cell: Cell, cell_type: CellType) -> &mut Maze {
        // TODO: check for valid Cell condiments
        self.walls.insert(cell, cell_type);
        self
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns Cell in the given direction or None
    fn neighbour(&self, cell: Cell, direction: Directions) -> Option<Cell> {
        let (row, col) = cell;
        match direction {
            Directions::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Directions::Down => {
                if row < self.height() - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Directions::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Directions::Right => {
                if col < self.width() - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
        }
    }


    /// Generates a Maze with `width` and `height`
    /// using Prim's algorithm.
    pub fn generate(width: u32, height: u32, seed: u64) -> Maze {
        let mut maze = Maze::filled(width, height);
        let rng = rand::thread_rng(); // TODO: add seed

        let start_cell = (0, 0);

        let mut in_maze = HashSet::new();
        in_maze.insert(start_cell);
        let mut walls = vec![];
        maze.add_cell_walls_to_vec(&mut walls, start_cell);

        while let Some((cell, wall)) = walls.pop() {
            if let Some(neighbour) = maze.neighbour(cell, wall) {
                //println!("neighbour: {:?}|{:?} -> {:?}", cell, wall, neighbour);
                if !in_maze.contains(&neighbour) {
                    in_maze.insert(neighbour);

                    //wall of the other sides
                    for i in 0..Directions::len()-1 {
                        if let Some(side) = maze.neighbour(cell, Directions::index(wall.get_index() + i)) {
                            in_maze.insert(side);
                        }
                    }

                    // create wall on the other size
                    //if rng.gen_range(0..2) != 0 {
                    //    if let Some(next) = maze.neighbour(cell, Directions::rand()) {
                    //        in_maze.insert(next);
                    //    }
                    //}

                    maze.add_cell_walls_to_vec(&mut walls, neighbour);
                    maze.remove_wall(cell, wall);
                }
            }
        }

        maze
    }

    // FIXME
    fn add_cell_walls_to_vec(&self, walls: &mut Vec<(Cell, Directions)>, cell: Cell) {
        //walls.push((cell, Directions::Up));
        //walls.push((cell, Directions::Down));
        //walls.push((cell, Directions::Left));
        //walls.push((cell, Directions::Right));

        //let direct = Directions::rand();
        //walls.push((cell,direct));
        //walls.push((cell,direct.opposite()));

        let mut rng = rand::thread_rng();
        let mut dlist = Directions::to_array();
        dlist.shuffle(&mut rng);
        for i in dlist.iter() {
            walls.push((cell, *i));
        }
    }

    /// Removes `wall` from `cell`, and the corresponding wall
    /// of the `cell`'s neighbour, if it exists.
    fn remove_wall(&mut self, cell: Cell, wall: Directions) {
        match self.walls.get_mut(&cell) {
            Some(walls) => {
                self.add_cell(cell, CellType::Room); // FIXME

                if let Some(neighbour) = self.neighbour(cell, wall) {
                    if let Some(neighbour_walls) = self.walls.get_mut(&neighbour) {
                        self.add_cell(neighbour, CellType::Room);
                        //neighbour_walls.remove(&wall.opposite());
                    }
                }
            }
            None => (),
        }
    }


     /// TODO: find path between cells using A*
     /// https://en.wikipedia.org/wiki/A*_search_algorithm
    pub fn find_path(&mut self, start: Cell, end: Cell, set_path: bool) -> Option<Vec<Cell>> {

        let dist = ((end.0 - start.0).pow(2) as f32 + (end.1 - start.1).pow(2) as f32).sqrt();
        println!("dist: {}", dist);

        // start or ends inside a wall
        if self.walls.get(&start) == Some(&CellType::Wall) || self.walls.get(&end) == Some(&CellType::Wall) {
         println!("find_path: no path found"); //DEBUG
         return None;
        }


         let mut deps = UnGraphMap::<Cell, ()>::new();
         //let mut deps = GraphMap::<Cell, f32>::new();
         let mut checked_nodes: HashSet<Cell> = HashSet::new();
         checked_nodes.insert(start);
         let mut cells: Vec<_> = vec![];
         cells.push(deps.add_node(start));

         // add nodes to graph
         while let Some(cnode) = cells.pop() {
             for d in Directions::to_array().iter() {
                 if let Some(nnode) = self.neighbour(cnode, *d) {
                     if self.walls.get(&nnode) == Some(&CellType::Room) {


                         //if !checked_nodes.contains(&neighbour) {  // for graph
                         if !deps.contains_node(nnode) {  // for graphmap
                             cells.push(nnode);
                         } else {
                             let nnode = deps.add_node(nnode);
                         }
                         deps.add_edge(cnode, nnode, ());
                         //cells.push(nnode);
                         //checked_nodes.insert(neighbour);  // for graph
                         println!("{:?}", nnode);
                     }
                 }
             }
         }

         //println!("deps: {:#?}", deps); //DEBUG

         //let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&deps));
         println!("__________________________________________\n\n{:?}",
                  Dot::with_config(&deps, &[Config::EdgeNoLabel]));

         let res = astar(&deps,
                         start,
                         |finish| finish == end,
                         |_| 0,
                         |n| ((end.0 - n.0).pow(2) as f32 + (end.1 - n.1).pow(2) as f32).sqrt() as u32
         );
         println!("\n\n{:?}", res);
         match res {
             Some((_,p)) => {
                 for i in &p {
                     self.add_cell(*i, CellType::Path);
                 }
                 Some(p)
             },
             _ => None,
         }


         // A*
         /*
         struct CellPath {
             cell: Cleek,
             path_len: u32,
             path_hist: u32,
         }

         let mut node_path: Vec<CellPath> = vec![]; // stack
        */

    }



    // find_path helper reconstruct_path
    //fn rec_path(from: Cell, curr: Cell) -> Vec<Cell>{
    //}


}

impl std::ops::Index<Cell> for Maze {
    type Output = CellType;
    fn index(&self, index: Cell) -> &Self::Output  {
        self
            .walls
            .get(&index)
            .expect(&format!("Cell at {:?} doesn't exist.", &index))
    }
}