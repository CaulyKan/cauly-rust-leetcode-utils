pub struct BinarySearch<T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    list: Vec<T>,
    weight_func: TWeightFn,
}

impl<T, TWeightFn> BinarySearch<T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    pub fn new(weight_func: TWeightFn) -> Self {
        BinarySearch {
            list: Vec::new(),
            weight_func,
        }
    }

    pub fn from(list: Vec<T>, weight_func: TWeightFn) -> Self {
        let mut temp = Vec::from(list);
        temp.sort_by(|x, y| weight_func(&x).cmp(&weight_func(&y)));
        BinarySearch {
            list: temp,
            weight_func,
        }
    }

    pub fn from_sorted(list: Vec<T>, weight_func: TWeightFn) -> Self {
        BinarySearch {
            list: Vec::from(list),
            weight_func,
        }
    }

    pub fn get(&self) -> &Vec<T> {
        &self.list
    }

    pub fn get_mut(&mut self) -> &mut Vec<T> {
        &mut self.list
    }

    pub fn weight(&self, v: &T) -> i32 {
        (self.weight_func)(v)
    }

    pub fn how_many_values_smaller_than(&self, v: T) -> usize {
        if let Some(i) = self.find_last_index_smaller_than(v) {
            i + 1
        } else {
            0
        }
    }

    pub fn find_last_index_smaller_than(&self, v: T) -> Option<usize> {
        if self.list.len() == 0 {
            return None;
        }
        let mut left = 0;
        let mut right = self.list.len();
        let v_weight = self.weight(&v);
        while left < right {
            let mid = (left + right) / 2;
            if self.weight(&self.list[mid]) == v_weight {
                right = mid;
            } else if self.weight(&self.list[mid]) < v_weight {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == self.list.len() {
            Some(self.list.len() - 1)
        } else if left != 0 || v_weight != self.weight(&self.list[0]) {
            Some(left)
        } else {
            None
        }
    }

    pub fn find_smallest_index_equal_to(&self, v: T) -> Option<usize> {
        let v_weight = self.weight(&v);
        let n = self.how_many_values_smaller_than(v);
        if self.weight(&self.list[n]) == v_weight {
            Some(n)
        } else {
            None
        }
    }

    /// Find a index that is equal to v. If multiple v exist, any index of them may return.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::BinarySearch;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(Some(2), result.find_any_index_equal_to(2)); // note that Some(1), Some(3) are also valid result.
    /// ```
    pub fn find_any_index_equal_to(&self, v: T) -> Option<usize> {
        if self.list.len() == 0 {
            return None;
        }
        let mut left = 0;
        let mut right = self.list.len() - 1;
        let v_weight = self.weight(&v);
        while left <= right {
            let mid = left + (right - left) / 2;
            if self.weight(&self.list[mid]) == v_weight {
                return Some(mid);
            } else if self.weight(&self.list[mid]) < v_weight {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        None
    }

    /// Check if v exists in collection.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::BinarySearch;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(true, result.exists(2));
    /// ```
    pub fn exists(&self, v: T) -> bool {
        self.find_any_index_equal_to(v).is_some()
    }
}
