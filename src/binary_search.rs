pub struct BinarySearch<T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    list: Vec<T>,
    weight_func: TWeightFn,
}

pub struct ReadonlyBinarySearch<'a, T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    list: &'a [T],
    weight_func: TWeightFn,
}

pub trait IBinarySearch<T, TWeightFn> {
    fn weight(&self, v: &T) -> i32;
    fn get_list(&self) -> &[T];

    /// Find how many values are smaller than v.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(4, result.how_many_values_smaller_than(&3));
    /// assert_eq!(1, result.how_many_values_smaller_than(&2));
    /// assert_eq!(0, result.how_many_values_smaller_than(&1));
    /// assert_eq!(0, result.how_many_values_smaller_than(&0));
    /// ```
    fn how_many_values_smaller_than(&self, v: &T) -> usize {
        if let Some(i) = self.find_last_index_smaller_than(v) {
            i + 1
        } else {
            0
        }
    }

    /// Find how many values are smaller than v.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(4, result.how_many_values_smaller_than(&3));
    /// assert_eq!(1, result.how_many_values_smaller_than(&2));
    /// assert_eq!(0, result.how_many_values_smaller_than(&1));
    /// assert_eq!(0, result.how_many_values_smaller_than(&0));
    /// ```
    fn how_many_values_larger_than(&self, v: &T) -> usize {
        if let Some(i) = self.find_first_index_larger_than(v) {
            self.get_list().len() - i
        } else {
            0
        }
    }

    /// Find index i that list[i] < v && list[i+1] >= v. If v is the smallest, return None
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(Some(3), result.find_last_index_smaller_than(&3));
    /// assert_eq!(Some(0), result.find_last_index_smaller_than(&2));
    /// assert_eq!(None, result.find_last_index_smaller_than(&1));
    /// ```
    fn find_last_index_smaller_than(&self, v: &T) -> Option<usize> {
        if self.get_list().len() == 0 {
            return None;
        }
        let mut left = 0;
        let mut right = self.get_list().len();
        let v_weight = self.weight(v);
        while left < right {
            let mid = (left + right) / 2;
            if self.weight(&self.get_list()[mid]) == v_weight {
                right = mid;
            } else if self.weight(&self.get_list()[mid]) < v_weight {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if left == self.get_list().len() {
            Some(self.get_list().len() - 1)
        } else if left == 0 {
            None
        } else {
            Some(left - 1)
        }
    }

    /// Find index i that list[i] > v && list[i-1] <= v. If v is the largest, return None
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(Some(1), result.find_first_index_larger_than(&1));
    /// assert_eq!(Some(4), result.find_first_index_larger_than(&2));
    /// assert_eq!(None, result.find_first_index_larger_than(&3));
    /// ```
    fn find_first_index_larger_than(&self, v: &T) -> Option<usize> {
        if self.get_list().len() == 0 {
            return None;
        }
        let mut left = 0;
        let mut right = self.get_list().len();
        let v_weight = self.weight(v);
        while left < right {
            let mid = (left + right) / 2;
            if self.weight(&self.get_list()[mid]) == v_weight {
                left = mid + 1;
            } else if self.weight(&self.get_list()[mid]) < v_weight {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if right == 0 {
            Some(0)
        } else if right >= self.get_list().len() {
            None
        } else {
            Some(right)
        }
    }

    /// Find the smallest index of v.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(Some(1), result.find_smallest_index_equal_to(&2));
    /// ```
    fn find_smallest_index_equal_to(&self, v: &T) -> Option<usize> {
        let v_weight = self.weight(&v);
        let n = self.how_many_values_smaller_than(v);
        if n == self.get_list().len() {
            None
        } else {
            if self.weight(&self.get_list()[n]) == v_weight {
                Some(n)
            } else {
                None
            }
        }
    }

    /// Find the largest index of v.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(Some(3), result.find_largest_index_equal_to(&2));
    /// ```
    fn find_largest_index_equal_to(&self, v: &T) -> Option<usize> {
        let v_weight = self.weight(&v);
        let n = self.how_many_values_larger_than(v);
        if n == self.get_list().len() {
            None
        } else {
            if self.weight(&self.get_list()[self.get_list().len() - n - 1]) == v_weight {
                Some(self.get_list().len() - n - 1)
            } else {
                None
            }
        }
    }

    /// Find a index that is equal to v. If multiple v exist, any index of them may return.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(Some(0), result.find_any_index_equal_to(&1));
    /// assert_eq!(Some(2), result.find_any_index_equal_to(&2)); // note that Some(1), Some(3) are also valid result.
    /// assert_eq!(Some(4), result.find_any_index_equal_to(&3));
    /// assert_eq!(None, result.find_any_index_equal_to(&4));
    /// ```
    fn find_any_index_equal_to(&self, v: &T) -> Option<usize> {
        if self.get_list().len() == 0 {
            return None;
        }
        let mut left = 0;
        let mut right = self.get_list().len() - 1;
        let v_weight = self.weight(v);
        while left <= right {
            let mid = left + (right - left) / 2;
            if self.weight(&self.get_list()[mid]) == v_weight {
                return Some(mid);
            } else if self.weight(&self.get_list()[mid]) < v_weight {
                left = mid + 1;
            } else {
                if mid == 0 {
                    return None;
                }
                right = mid - 1;
            }
        }
        None
    }

    /// Check if v exists in collection.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// assert_eq!(true, result.exists(&2));
    /// assert_eq!(false, result.exists(&4));
    /// assert_eq!(false, result.exists(&0));
    /// ```
    fn exists(&self, v: &T) -> bool {
        self.find_any_index_equal_to(v).is_some()
    }
}

impl<T, TWeightFn> IBinarySearch<T, TWeightFn> for BinarySearch<T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    fn weight(&self, v: &T) -> i32 {
        (self.weight_func)(v)
    }
    fn get_list(&self) -> &[T] {
        self.list.as_slice()
    }
}

impl<T, TWeightFn> BinarySearch<T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    /// Create a empty binary search list, with a custom weight function.
    pub fn new(weight_func: TWeightFn) -> Self {
        BinarySearch {
            list: Vec::new(),
            weight_func,
        }
    }

    /// Build from a vec. The vec will then be sorted.
    pub fn from(list: Vec<T>, weight_func: TWeightFn) -> Self {
        let mut temp = Vec::from(list);
        temp.sort_by(|x, y| weight_func(&x).cmp(&weight_func(&y)));
        BinarySearch {
            list: temp,
            weight_func,
        }
    }

    /// Build from a sorted vec.
    pub fn from_sorted(list: Vec<T>, weight_func: TWeightFn) -> Self {
        BinarySearch {
            list: Vec::from(list),
            weight_func,
        }
    }

    /// Get the underlying vec.
    pub fn get(&self) -> &Vec<T> {
        &self.list
    }

    /// Get the underlying vec as mutable. Be sure to keep the vec IN ORDER!
    pub fn get_mut(&mut self) -> &mut Vec<T> {
        &mut self.list
    }

    /// Insert a value into list.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let mut result = BinarySearch::from(vec![1, 2, 2, 2, 3], |x| *x);
    /// result.insert(4);
    /// assert_eq!(true, result.exists(&4));
    /// ```
    pub fn insert(&mut self, v: T) {
        match self.find_first_index_larger_than(&v) {
            Some(i) => self.list.insert(i, v),
            None => self.list.push(v),
        }
    }
}

impl<'a, T, TWeightFn> IBinarySearch<T, TWeightFn> for ReadonlyBinarySearch<'a, T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    fn weight(&self, v: &T) -> i32 {
        (self.weight_func)(v)
    }
    fn get_list(&self) -> &[T] {
        self.list
    }
}

impl<'a, T, TWeightFn> ReadonlyBinarySearch<'a, T, TWeightFn>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
    TWeightFn: Fn(&T) -> i32,
{
    /// Build from a slice. The slice must be sorted.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let vec = vec![1,2,3,4];
    /// let result = ReadonlyBinarySearch::from(&vec[1..], |x|*x);
    /// assert_eq!(false, result.exists(&1));
    /// ```
    pub fn from(list: &'a [T], weight_func: TWeightFn) -> Self {
        ReadonlyBinarySearch { list, weight_func }
    }

    /// Get length of current slice
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::binary_search::*;
    /// let vec = vec![1,2,3,4];
    /// let result = ReadonlyBinarySearch::from(&vec[1..], |x|*x);
    /// assert_eq!(3, result.len());
    /// ```
    pub fn len(&self) -> usize {
        self.list.len()
    }
}
