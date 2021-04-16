// #region Graph
use std::cmp::*;
use std::collections::*;

pub struct Graph {
    map: Vec<Vec<Option<i32>>>,
    nodes: HashSet<usize>,
    is_directed: bool,
    max_id: usize,
}

impl Graph {
    pub fn new<F>(
        paths: &Vec<Vec<i32>>,
        is_weighted: bool,
        is_directed: bool,
        on_duplicate: F,
    ) -> Self
    where
        F: Fn(i32, i32) -> i32,
    {
        let mut nodes = HashSet::new();
        let mut max_id = i32::min_value();
        for i in paths {
            nodes.insert(i[0] as usize);
            nodes.insert(i[1] as usize);
            max_id = max(i[0], max(i[1], max_id));
        }
        let max_id = max_id as usize;
        let n = max_id + 1;
        let mut map: Vec<Vec<Option<i32>>> = vec![vec![None; n]; n];
        for edge in paths {
            if is_weighted {
                if let Some(vold) = map[edge[0] as usize][edge[1] as usize] {
                    let vnew = on_duplicate(vold, edge[2]);
                    map[edge[0] as usize][edge[1] as usize] = Some(vnew);
                    if !is_directed {
                        map[edge[1] as usize][edge[0] as usize] = Some(vnew);
                    }
                } else {
                    map[edge[0] as usize][edge[1] as usize] = Some(edge[2]);
                    if !is_directed {
                        map[edge[1] as usize][edge[0] as usize] = Some(edge[2]);
                    }
                }
            } else {
                map[edge[0] as usize][edge[1] as usize] = Some(1);
                if !is_directed {
                    map[edge[1] as usize][edge[0] as usize] = Some(1);
                }
            }
        }
        Graph {
            map,
            nodes: nodes,
            is_directed,
            max_id,
        }
    }

    pub fn dist(&self, source: i32, target: i32) -> Option<i32> {
        self.map[source as usize][target as usize]
    }
    pub fn nodes(&self) -> impl Iterator<Item = i32> + '_ {
        self.nodes.iter().map(|&x| x as i32)
    }

    pub fn set_dist(&mut self, source: i32, target: i32, v: i32) {
        self.map[source as usize][target as usize] = Some(v);
        if !self.is_directed {
            self.map[target as usize][source as usize] = Some(v);
        }
    }

    pub fn get_connects(&self, source: i32) -> impl Iterator<Item = (usize, i32)> + '_ {
        let source = source as usize;
        self.nodes
            .iter()
            .filter(move |&&target| target != source && self.map[source][target].is_some())
            .map(move |&target| (target, self.map[source][target].unwrap()))
    }

    pub fn bfs<F>(&self, start: Vec<i32>, mut f: F)
    where
        F: FnMut(i32, i32) -> bool,
    {
        let mut current = start;
        while current.len() != 0 {
            let mut next = Vec::new();
            for &source in &current {
                let source = source as usize;
                for &target in &self.nodes {
                    if source != target && self.map[source as usize][target as usize].is_some() {
                        if f(source as i32, target as i32) {
                            next.push(target as i32);
                        }
                    }
                }
            }
            current = next;
        }
    }

    pub fn bfs_rev<F>(&self, start: Vec<i32>, mut f: F)
    where
        F: FnMut(i32, i32) -> bool,
    {
        let mut current = start;
        while current.len() != 0 {
            let mut next = Vec::new();
            for &target in &current {
                let target = target as usize;
                for &source in &self.nodes {
                    if source != target && self.map[source as usize][target as usize].is_some() {
                        if f(source as i32, target as i32) {
                            next.push(target as i32);
                        }
                    }
                }
            }
            current = next;
        }
    }

    /// dijkstra algorithm that solves the shortest path from start to end node
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::graph::*;
    /// let graph = Graph::new(&vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]], true, true, |x,y|x);
    /// assert_eq!(Some(2), graph.dijkstra(2, 4))
    /// ```
    pub fn dijkstra(&self, start: i32, end: i32) -> Option<i32> {
        let mut scores = vec![None; self.max_id + 1];
        let mut is_visited = vec![false; self.max_id + 1];
        scores[start as usize] = Some(0);
        loop {
            let current = (0..self.max_id + 1)
                .filter(|&x| !is_visited[x])
                .min_by_key(|&x| scores[x].unwrap_or(i32::max_value()));

            match current {
                Some(current) if scores[current].is_some() && current as i32 == end => {
                    return Some(scores[current].unwrap())
                }
                Some(current) if scores[current].is_some() => {
                    is_visited[current] = true;
                    for (other, d) in self.get_connects(current as i32) {
                        if scores[other as usize].is_some() {
                            scores[other as usize] = Some(min(
                                scores[other as usize].unwrap_or(i32::max_value()),
                                scores[current].unwrap() + d,
                            ));
                        } else {
                            scores[other as usize] = Some(scores[current].unwrap() + d);
                        }
                    }
                }
                _ => break None,
            }
        }
    }
    /// dijkstra algorithm that solves the shortest path from start to all nodes
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::graph::*;
    /// let graph = Graph::new(&vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]], true, true, |x,y|x);
    /// assert_eq!(Some(2), graph.dijkstra_all(2)[4]);
    /// assert_eq!(Some(1), graph.dijkstra_all(2)[3]);
    /// assert_eq!(Some(1), graph.dijkstra_all(2)[1]);
    /// assert_eq!(Some(0), graph.dijkstra_all(2)[2]);
    /// ```
    pub fn dijkstra_all(&self, start: i32) -> Vec<Option<i32>> {
        let mut scores = vec![None; self.max_id + 1];
        let mut is_visited = vec![false; self.max_id + 1];
        scores[start as usize] = Some(0);

        loop {
            let current = (0..self.max_id + 1)
                .filter(|&x| !is_visited[x])
                .min_by_key(|&x| scores[x].unwrap_or(i32::max_value()));

            match current {
                Some(current) if scores[current].is_some() => {
                    is_visited[current] = true;
                    for (other, d) in self.get_connects(current as i32) {
                        if scores[other as usize].is_some() {
                            scores[other as usize] = Some(min(
                                scores[other as usize].unwrap_or(i32::max_value()),
                                scores[current].unwrap() + d,
                            ));
                        } else {
                            scores[other as usize] = Some(scores[current].unwrap() + d);
                        }
                    }
                }
                _ => break,
            }
        }
        scores
    }
}
// #endregion
