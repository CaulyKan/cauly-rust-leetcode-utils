pub struct SegmentTree<TInput: Clone + Default, TOutput: Clone + Default> {
    list: Vec<TOutput>,
    conv_func: Box<dyn Fn(&TInput) -> TOutput>,
    aggr_func: Box<dyn Fn(&TOutput, &TOutput) -> TOutput>,
}

impl<T: Clone + Default> SegmentTree<T, T> {
    pub fn from_simple(list: &[T], aggr_func: Box<dyn Fn(&T, &T) -> T>) -> Self {
        SegmentTree::from(list, Box::new(|x| x.clone()), aggr_func)
    }
}

impl<TInput: Clone + Default, TOutput: Clone + Default> SegmentTree<TInput, TOutput> {
    pub fn from(
        list: &[TInput],
        conv_func: Box<dyn Fn(&TInput) -> TOutput>,
        aggr_func: Box<dyn Fn(&TOutput, &TOutput) -> TOutput>,
    ) -> Self {
        let n = list.len();
        let mut temp = vec![Default::default(); n * 2];
        for i in n..n * 2 {
            temp[i] = conv_func(&list[i - n]);
        }
        for index in (1..n).rev() {
            temp[index] = (aggr_func)(&temp[index * 2], &temp[index * 2 + 1]);
        }
        SegmentTree {
            list: temp,
            conv_func,
            aggr_func,
        }
    }

    pub fn convert(&self, input: &TInput) -> TOutput {
        (self.conv_func)(input)
    }

    /// SegmentTree with custom aggr func.
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::segment_tree::*;
    /// let mut st = SegmentTree::from_simple(&[1,2,3], Box::new(|&x,&y|x+y));
    /// assert_eq!(6, st.query(0,3,0));
    /// st.update(0, &0);
    /// assert_eq!(5, st.query(0,3,0));
    /// ```
    pub fn update(&mut self, i: usize, val: &TInput) {
        let mut index = i + self.list.len() / 2;
        self.list[index] = self.convert(val);
        while index > 1 {
            index /= 2;
            self.list[index] = (self.aggr_func)(&self.list[index * 2], &self.list[index * 2 + 1]);
        }
    }

    /// Get the value of range [left, right)
    ///
    /// # Example
    /// ```
    /// use cauly_rust_leetcode_utils::segment_tree::*;
    /// let mut st = SegmentTree::from_simple(&[1,2,3], Box::new(|&x,&y|x+y));
    /// assert_eq!(6, st.query(0,3,0));
    /// assert_eq!(3, st.query(0,2,0));
    /// assert_eq!(5, st.query(1,3,0));
    /// let mut st2 = SegmentTree::from_simple(&[1,2,3], Box::new(|&x,&y|std::cmp::min(x,y)));
    /// assert_eq!(1, st2.query(0,3,999));
    /// assert_eq!(1, st2.query(0,2,999));
    /// assert_eq!(2, st2.query(1,3,999));
    /// ```
    pub fn query(&self, left: usize, right: usize, default: TOutput) -> TOutput {
        let n = self.list.len() / 2;
        let mut left = left + n;
        let mut right = right + n;
        let mut result: TOutput = default;
        while left < right {
            if (left & 1) == 1 {
                result = (self.aggr_func)(&result, &self.list[left]);
                left += 1;
            }
            if (right & 1) == 1 {
                right -= 1;
                result = (self.aggr_func)(&result, &self.list[right]);
            }
            left = left >> 1;
            right = right >> 1;
        }
        result
    }
}
