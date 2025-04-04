use std::collections::HashSet;

pub const DIRECTION_RIGHT: Point = Point { x: 1, y: 0 };
pub const DIRECTION_LEFT: Point = Point { x: -1, y: 0 };
pub const DIRECTION_UP: Point = Point { x: 0, y: 1 };
pub const DIRECTION_DOWN: Point = Point { x: 0, y: -1 };
pub const DIRECTIONS: [Point; 4] = [
    DIRECTION_RIGHT,
    DIRECTION_LEFT,
    DIRECTION_UP,
    DIRECTION_DOWN,
];

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}
pub fn get_distance(a: &Point, b: &Point) -> u8 {
    (a.x - b.x).abs() as u8 + (a.y - b.y).abs() as u8
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Cell {
    Empty = 0,
    Color1 = 1,
    Color2 = 2,
    Color3 = 3,
    Color4 = 4,
}

#[derive(Clone)]
pub struct Grid {
    pub width: u8,
    pub height: u8,
    pub cells: Vec<Cell>,
}
impl Grid {
    pub fn create_empty(width: u8, height: u8) -> Grid {
        let n = (width as usize) * (height as usize);
        let cells = (0..n).map(|_| Cell::Empty).collect();

        Grid {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, x: i8, y: i8) -> usize {
        return (x as usize) * (self.height as usize) + (y as usize);
    }
    pub fn get_cell(&self, p: &Point) -> Cell {
        let i = self.get_index(p.x, p.y);
        return self.cells[i];
    }
    pub fn set_cell(&mut self, p: &Point, value: Cell) -> () {
        let i = self.get_index(p.x, p.y);
        self.cells[i] = value;
    }
    pub fn is_inside(&self, p: &Point) -> bool {
        0 <= p.x && p.x < (self.width as i8) && 0 <= p.y && p.y < (self.height as i8)
    }
    pub fn is_inside_margin(&self, p: &Point, m: i8) -> bool {
        -m <= p.x && p.x < (self.width as i8) + m && -m <= p.y && p.y < (self.height as i8) + m
    }
    pub fn iter(&self) -> impl Iterator<Item = Point> {
        let mut i = 0;
        let width = self.width;
        let height = self.height as usize;
        std::iter::from_fn(move || {
            let p = Point {
                x: (i / height) as i8,
                y: (i % height) as i8,
            };

            i += 1;

            if p.x >= (width as i8) {
                None
            } else {
                Some(p)
            }
        })
    }
}

#[derive(Clone)]
pub struct WalkableGrid {
    pub grid: Grid,
    walkable: Cell,
}
impl WalkableGrid {
    pub fn create(grid: Grid, walkable: Cell) -> WalkableGrid {
        WalkableGrid { grid, walkable }
    }
    pub fn is_cell_walkable(&self, p: &Point) -> bool {
        !self.grid.is_inside(p) || self.grid.get_cell(p) <= self.walkable
    }
    pub fn set_walkable(&mut self, walkable: Cell) -> () {
        self.walkable = walkable;
    }
    pub fn is_inside(&self, p: &Point) -> bool {
        self.grid.is_inside(p)
    }
    pub fn is_inside_margin(&self, p: &Point, margin: i8) -> bool {
        self.grid.is_inside_margin(p, margin)
    }
    pub fn get_cell(&self, p: &Point) -> Cell {
        self.grid.get_cell(p)
    }
}

#[test]
fn it_should_sort_cell() {
    assert_eq!(Cell::Empty < Cell::Color1, true);
    assert_eq!(Cell::Color1 < Cell::Color2, true);
    assert_eq!(Cell::Color2 < Cell::Color3, true);
    assert_eq!(Cell::Color3 < Cell::Color4, true);
}
#[test]
fn it_should_grid_create() {
    let grid = Grid::create_empty(30, 10);

    assert_eq!(grid.width, 30);
    assert_eq!(grid.height, 10);
    assert_eq!(grid.get_cell(&Point { x: 2, y: 3 }), Cell::Empty);
}
#[test]
fn it_should_grid_setter() {
    let mut grid = Grid::create_empty(20, 10);

    grid.set_cell(&Point { x: 12, y: 3 }, Cell::Color1);

    assert_eq!(grid.get_cell(&Point { x: 12, y: 3 }), Cell::Color1);
}
#[test]
fn it_should_iterate() {
    let grid = Grid::create_empty(2, 2);

    assert_eq!(
        grid.iter().collect::<HashSet<_>>(),
        HashSet::from([
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
        ])
    );
}
