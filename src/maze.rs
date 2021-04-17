use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CellType {
    Room,
    Wall,
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