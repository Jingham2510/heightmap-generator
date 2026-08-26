
 

///A point which describes a cell location
#[derive(Debug)]
pub struct Point{
    x : usize,
    y : usize,
}

impl Point{
    ///Create a point
    fn create(x : usize, y :usize) -> Self{
        Point{
            x,
            y
        }
    }

    fn as_xy(&self) -> (usize, usize){
        (self.x, self.y)
    }
}


//Indicates a direction from a cell (where north is up)
#[derive(Debug, Clone, PartialEq)]
pub enum Direction{
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHWEST,
    NORTHEAST,
    SOUTHEAST,
    SOUTHWEST
}

///An edge which describes the cell and the direction(s) of the edge
#[derive(Debug)]
pub struct Edge{
    cell : Point,
    dir : Vec<Direction>
}

impl Edge{
    ///Create an edge object
    pub fn create(x : usize, y: usize, dir : Vec<Direction>) -> Self{
        Edge{
            cell : Point::create(x, y),
            dir
        }
    }

    ///Return the edge directions
    pub fn dir(&self) -> Vec<Direction>{
        self.dir.clone()
    }


    pub fn pos_f64(&self) -> (f64, f64){
        let (x, y) = self.cell.as_xy();
        (x as f64, y as f64)
    }

}

