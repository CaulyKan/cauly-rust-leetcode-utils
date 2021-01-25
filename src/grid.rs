// #region Grid
pub struct Grid<T>
where
    T: Clone,
{
    width: usize,
    height: usize,
    values: Vec<Vec<T>>,
}

pub struct GridItem<T>
where
    T: Clone,
{
    val: T,
    x: usize,
    y: usize,
}

impl<T> Grid<T>
where
    T: Default,
    T: Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            values: vec![vec![T::default(); width]; height],
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn from(grid: Vec<Vec<T>>) -> Self {
        Grid {
            width: grid[0].len(),
            height: grid.len(),
            values: grid,
        }
    }
    pub fn get_near_4(&self, pos: (usize, usize)) -> Vec<GridItem<T>> {
        let mut result = Vec::new();

        if pos.0 > 0 {
            result.push(GridItem {
                x: pos.0 - 1,
                y: pos.1,
                val: self.values[pos.1][pos.0 - 1].clone(),
            });
        }
        if pos.0 < self.width - 1 {
            result.push(GridItem {
                x: pos.0 + 1,
                y: pos.1,
                val: self.values[pos.1][pos.0 + 1].clone(),
            });
        }
        if pos.1 > 0 {
            result.push(GridItem {
                x: pos.0,
                y: pos.1 - 1,
                val: self.values[pos.1 - 1][pos.0].clone(),
            });
        }
        if pos.1 < self.height - 1 {
            result.push(GridItem {
                x: pos.0,
                y: pos.1 + 1,
                val: self.values[pos.1 + 1][pos.0].clone(),
            });
        }

        result
    }
    pub fn is_near_4(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        let x = (pos1.0 as i32 - pos2.0 as i32).abs();
        let y = (pos1.1 as i32 - pos2.1 as i32).abs();
        x == 1 && y == 0 || x == 0 && y == 1
    }
}

// #endregion
