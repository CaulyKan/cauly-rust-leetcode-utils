// #region UnionFind
use std::collections::HashMap;

pub struct UnionFind4Usize {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
    length: usize,
}

pub struct UnionFind<T>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: std::fmt::Debug,
{
    map: HashMap<T, usize>,
    uf: UnionFind4Usize,
}

impl UnionFind4Usize {
    pub fn new(count: usize) -> Self {
        UnionFind4Usize {
            count,
            length: count,
            id: (0..count).collect(),
            size: vec![1; count as usize],
        }
    }

    pub fn add(&mut self) -> usize {
        self.count += 1;
        self.id.push(self.length);
        self.size.push(1);
        self.length += 1;
        self.length - 1
    }

    pub fn is_connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn find(&mut self, p: usize) -> usize {
        let mut p = p;
        while p != self.id[p] {
            self.id[p] = self.id[self.id[p]];
            p = self.id[p]
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            self.id[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.id[j] = i;
            self.size[i] += self.size[j];
        }
        self.count -= 1;
    }

    pub fn union_count(&self) -> usize {
        self.count
    }

    pub fn union_size(&mut self, p: usize) -> usize {
        let root = self.find(p);
        return self.size[root];
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> UnionFind<T>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        UnionFind {
            map: HashMap::new(),
            uf: UnionFind4Usize {
                count: 0,
                length: 0,
                id: Vec::new(),
                size: Vec::new(),
            },
        }
    }

    pub fn from_iter<I>(iter: I) -> UnionFind<T>
    where
        I: IntoIterator<Item = T>,
    {
        let mut map = HashMap::new();
        let mut index = 0;
        for item in iter.into_iter() {
            map.insert(item, index);
            index += 1;
        }
        let len = map.len();
        UnionFind {
            map,
            uf: UnionFind4Usize::new(len),
        }
    }

    pub fn len(&self) -> usize {
        self.uf.len()
    }

    pub fn union_count(&self) -> usize {
        self.uf.union_count()
    }

    pub fn union_size(&mut self, p: T) -> Option<usize> {
        if let Some(index) = self.map.get(&p) {
            let root_index = self.uf.find(*index);
            Some(self.uf.union_size(root_index))
        } else {
            None
        }
    }

    pub fn find(&mut self, p: T) -> Option<&T> {
        if let Some(index) = self.map.get(&p) {
            let root_index = self.uf.find(*index);
            self._find_by_index(root_index)
        } else {
            None
        }
    }

    pub fn union(&mut self, p: T, q: T) -> Result<usize, String> {
        if let Some(pindex) = self.map.get(&p) {
            if let Some(qindex) = self.map.get(&q) {
                self.uf.union(*pindex, *qindex);
                return Ok(self.uf.union_size(*pindex));
            } else {
                return Err(format!("{:?} not found.", q));
            }
        } else {
            return Err(format!("{:?} not found.", p));
        }
    }

    pub fn is_connected(&mut self, p: T, q: T) -> Result<bool, String> {
        if let Some(pindex) = self.map.get(&p) {
            if let Some(qindex) = self.map.get(&q) {
                return Ok(self.uf.find(*pindex) == self.uf.find(*qindex));
            } else {
                return Err(format!("{:?} not found.", q));
            }
        } else {
            return Err(format!("{:?} not found.", p));
        }
    }

    pub fn add(&mut self, p: T) {
        let index = self.uf.add();
        self.map.insert(p, index);
    }

    fn _find_by_index(&self, index: usize) -> Option<&T> {
        for (k, v) in self.map.iter() {
            if *v == index {
                return Some(k);
            }
        }
        None
    }
}

// #endregion
