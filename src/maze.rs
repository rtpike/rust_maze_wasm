use std::collections::HashMap;
use std::collections::HashSet;

use rand::Rng;
use rand::seq::SliceRandom;

// used for A*
use petgraph::graphmap::UnGraphMap;
use petgraph::algo::{dijkstra, min_spanning_tree, astar};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Directions {
    Down,
    Up,
    Right,
    Left,
}

impl Directions {

    /// length: 4
    fn len() -> usize {
        4
    }

    /// get opposite direction
    #[allow(dead_code)]
    fn opposite(self) -> Directions {
        match self {
            Directions::Up => Directions::Down,
            Directions::Down => Directions::Up,
            Directions::Left => Directions::Right,
            Directions::Right => Directions::Left,
        }
    }

    /// get random direction
    #[allow(dead_code)]
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

    // TODO: fix name
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

    /// get index value of enum
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

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    width: u32,
    height: u32,
    walls: HashMap<Cell, CellType>,
}
/// Maze DOCS
impl Maze {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Maze {
        let walls = HashMap::new();
        Maze{width, height, walls}
    }

    /// empty maze with side walls only
    #[allow(dead_code)]
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
        let (x, y) = cell;
        match direction {
            Directions::Up => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Directions::Down => {
                if x < self.width() - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Directions::Left => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Directions::Right => {
                if y < self.height() - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
        }
    }


    /// Generates a Maze with `width` and `height`
    /// using Prim's algorithm.
    pub fn generate(width: u32, height: u32, _seed: u64) -> Maze {
        let mut maze = Maze::filled(width, height);

        let start_cell = (0, 0);
        let mut in_maze = HashSet::new();
        in_maze.insert(start_cell);
        let mut walls: Vec<(Cell, Directions)>= vec![];
        maze.add_cell_walls_to_vec(&mut walls, start_cell);

        while let Some((cell, direction)) = walls.pop() {
            if let Some(neighbour) = maze.neighbour(cell, direction) {
                //println!("neighbour: {:?}|{:?} -> {:?}", cell, wall, neighbour);
                if !in_maze.contains(&neighbour) {
                    in_maze.insert(neighbour);

                    // wall of the other sides
                    for i in 0..Directions::len()-1 {
                        if let Some(side) = maze.neighbour(cell, Directions::index(direction.get_index() + i)) {
                            in_maze.insert(side);
                        }
                    }

                    // fill in corner with a wall
/*
                    if let Some(op_cell) = maze.neighbour(cell, direction.opposite()) {
                        if let Some(side) = maze.neighbour(op_cell, Directions::index(direction.get_index() + 1)) {
                            in_maze.insert(side);
                            println!("found")
                        }
                        if let Some(side) = maze.neighbour(op_cell, Directions::index(direction.get_index() + 3)) {
                            in_maze.insert(side);
                        }
                    }
 */

                    maze.add_cell(cell, CellType::Room);

                    /* FIXME
                    let mut rng = rand::thread_rng(); // TODO: add seed
                    // create wall on the other size
                    if !rng.gen_range(0..10) != 0 {
                        if let Some(next) = maze.neighbour(cell, Directions::rand()) {
                            in_maze.insert(next);
                        }
                    }
                    */

                    // generate next directions
                    maze.add_cell_walls_to_vec(&mut walls, neighbour);
                }
            }
        }
        maze
    }

    /// get a room in a random direction
    fn add_cell_walls_to_vec(&self, walls: &mut Vec<(Cell, Directions)>, cell: Cell) {
/*
        walls.push((cell, Directions::Up));
        walls.push((cell, Directions::Down));
        walls.push((cell, Directions::Left));
        walls.push((cell, Directions::Right));
*/
        let mut rng = rand::thread_rng();
        let mut dlist = Directions::to_array();
        dlist.shuffle(&mut rng);
        for i in &dlist {
            walls.push((cell, *i));
        }
    }


     /// find path between cells using A*
     /// https://en.wikipedia.org/wiki/A*_search_algorithm
    pub fn find_path(&mut self, start: Cell, end: Cell, set_path: bool) -> Option<Vec<Cell>> {

         let dist = ((end.0 - start.0).pow(2) as f32 + (end.1 - start.1).pow(2) as f32).sqrt();
         println!("Direct distance: {:.2}", dist);

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
                         }
                         deps.add_edge(cnode, nnode, ());

                     }
                 }
             }
         }

         // Print out graph in Graphviz text format
         //println!("deps: {:#?}", deps); //DEBUG
         //let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&deps));
         //println!("__________________________________________\n\n{:?}",
         //         Dot::with_config(&deps, &[Config::EdgeNoLabel]));

         let res = astar(&deps,
                         start,
                         |finish| finish == end,
                         |_| 0,
                         |n| ((end.0 - n.0).pow(2) as f32 + (end.1 - n.1).pow(2) as f32).sqrt() as u32
         );
         //println!("\n\n{:?}", res);
         match res {
             Some((_,p)) => {
                 if set_path {
                     for i in &p {
                         self.add_cell(*i, CellType::Path);
                     }
                 }
                 println!("Path distance:  {}", p.len());
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
}

impl std::ops::Index<Cell> for Maze {
    type Output = CellType;
    fn index(&self, index: Cell) -> &Self::Output  {
        self
            .walls
            .get(&index)
            .unwrap_or_else(|| panic!("Cell at {:?} doesn't exist.", &index))
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn maze_default() {
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
        assert_eq!(Maze::default(3,3), maze);
    }

    #[test]
    fn maze_filled() {
        let mut maze = Maze::new(3,3);
        maze.add_cell((0,0), CellType::Wall);
        maze.add_cell((0,1), CellType::Wall);
        maze.add_cell((0,2), CellType::Wall);
        maze.add_cell((1,0), CellType::Wall);
        maze.add_cell((1,1), CellType::Wall);
        maze.add_cell((1,2), CellType::Wall);
        maze.add_cell((2,0), CellType::Wall);
        maze.add_cell((2,1), CellType::Wall);
        maze.add_cell((2,2), CellType::Wall);
        assert_eq!(Maze::filled(3,3), maze);
    }

    #[test]
    fn directions() {
        assert_eq!(Directions::len(), 4);
        assert_eq!(Directions::opposite(Directions::Right), Directions::Left);
        assert_eq!(Directions::opposite(Directions::Up), Directions::Down);
        assert_eq!(Directions::opposite(Directions::Left), Directions::Right);
        assert_eq!(Directions::opposite(Directions::Down), Directions::Up);

        // TODO: add rand()

        assert_eq!(Directions::to_array(), [Directions::Down,Directions::Up,Directions::Right,Directions::Left]);

        assert_eq!(Directions::index(0), Directions::Down);
        assert_eq!(Directions::index(1), Directions::Up);
        assert_eq!(Directions::index(2), Directions::Right);
        assert_eq!(Directions::index(3), Directions::Left);
        assert_eq!(Directions::index(4), Directions::Down);
        assert_eq!(Directions::index(5), Directions::Up);
        assert_eq!(Directions::index(6), Directions::Right);
        assert_eq!(Directions::index(7), Directions::Left);

        assert_eq!(Directions::Down.get_index(), 0);
        assert_eq!(Directions::Up.get_index(), 1);
        assert_eq!(Directions::Right.get_index(), 2);
        assert_eq!(Directions::Left.get_index(), 3);
    }

}