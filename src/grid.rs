// #region Grid
pub struct Grid<T>
where
    T: Clone,
{
    width: usize,
    height: usize,
    values: Vec<Vec<T>>,
    is_enabled: Vec<Vec<bool>>,
}

#[derive(Clone)]
pub struct GridItem<T>
where
    T: Clone,
{
    pub val: T,
    pub x: usize,
    pub y: usize,
    pub is_enabled: bool,
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
            is_enabled: vec![vec![true; width]; height],
        }
    }

    pub fn get_values(&self) -> impl Iterator<Item = &T> {
        let mut it: Box<dyn Iterator<Item = &T>> = Box::new(std::iter::empty());
        for i in &self.values {
            it = Box::new(it.chain(i.into_iter()));
        }
        it
    }

    pub fn get_items<'a>(&'a self) -> impl Iterator<Item = GridItem<T>> + 'a {
        self.values.iter().enumerate().flat_map(move |(i, vec)| {
            vec.iter().enumerate().map(move |(j, val)| GridItem {
                val: val.clone(),
                x: j,
                y: i,
                is_enabled: self.is_enabled[i][j],
            })
        })
    }

    pub fn set_is_disabled(&mut self, pos: (usize, usize), value: bool) {
        self.is_enabled[pos.1][pos.0] = value;
    }
}

impl<T> std::cmp::PartialEq for GridItem<T>
where
    T: Clone,
{
    fn eq(&self, r: &GridItem<T>) -> bool {
        self.get_pos() == r.get_pos()
    }
}

impl<T> std::cmp::Eq for GridItem<T> where T: Clone {}

impl<T> std::fmt::Debug for GridItem<T>
where
    T: Clone,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {:?})", self.x, self.y, self.val)
    }
}

impl<T> std::hash::Hash for GridItem<T>
where
    T: Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> GridItem<T>
where
    T: Clone,
{
    pub fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn from(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Grid {
            width,
            height,
            values: grid,
            is_enabled: vec![vec![true; width]; height],
        }
    }
    pub fn get_near_4(&self, pos: (usize, usize)) -> Vec<GridItem<T>> {
        let mut result = Vec::new();

        if pos.0 > 0 && self.is_enabled[pos.1][pos.0 - 1] {
            result.push(GridItem {
                x: pos.0 - 1,
                y: pos.1,
                val: self.values[pos.1][pos.0 - 1].clone(),
                is_enabled: true,
            });
        }
        if pos.0 < self.width - 1 && self.is_enabled[pos.1][pos.0 + 1] {
            result.push(GridItem {
                x: pos.0 + 1,
                y: pos.1,
                val: self.values[pos.1][pos.0 + 1].clone(),
                is_enabled: true,
            });
        }
        if pos.1 > 0 && self.is_enabled[pos.1 - 1][pos.0] {
            result.push(GridItem {
                x: pos.0,
                y: pos.1 - 1,
                val: self.values[pos.1 - 1][pos.0].clone(),
                is_enabled: true,
            });
        }
        if pos.1 < self.height - 1 && self.is_enabled[pos.1 + 1][pos.0] {
            result.push(GridItem {
                x: pos.0,
                y: pos.1 + 1,
                val: self.values[pos.1 + 1][pos.0].clone(),
                is_enabled: true,
            });
        }

        result
    }

    pub fn get(&self, pos: (usize, usize)) -> GridItem<T> {
        GridItem {
            x: pos.0,
            y: pos.1,
            val: self.values[pos.1][pos.0].clone(),
            is_enabled: self.is_enabled[pos.1][pos.0],
        }
    }

    pub fn is_near_4(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        let x = (pos1.0 as i32 - pos2.0 as i32).abs();
        let y = (pos1.1 as i32 - pos2.1 as i32).abs();
        x == 1 && y == 0 || x == 0 && y == 1
    }

    pub fn rows(&self) -> usize {
        self.height
    }

    pub fn cols(&self) -> usize {
        self.width
    }

    pub fn range_get_near_4<'a, TIter>(
        &'a self,
        range: TIter,
    ) -> std::collections::HashSet<GridItem<T>>
    where
        TIter: IntoIterator<Item = &'a GridItem<T>>,
    {
        let mut result = std::collections::HashSet::new();
        let mut copy = Vec::new();
        for c in range.into_iter() {
            for neibour in self.get_near_4(c.get_pos()) {
                result.insert(neibour);
            }
            copy.push(c);
        }
        for c in copy {
            result.remove(&c);
        }
        result
    }
}

// #endregion
