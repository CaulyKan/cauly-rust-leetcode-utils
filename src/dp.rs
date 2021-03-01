// #region dp

pub struct DP<T: Copy> {
    pub data: Vec<T>,
    pub sizes: Vec<usize>,
}

impl<T: Copy> DP<T> {
    pub fn get<TIndexer: DPIndexer>(&self, indexer: &TIndexer) -> T {
        self.data[indexer.index()]
    }
    pub fn get_mut<TIndexer: DPIndexer>(&mut self, indexer: &TIndexer) -> &mut T {
        self.data.get_mut(indexer.index()).unwrap()
    }
    pub fn new<TIndexer: DPIndexer>(init_val: T) -> Self {
        let sizes = TIndexer::sizes();
        let total = sizes.iter().fold(1, |x, i| x * i);
        let data = vec![init_val; total];
        DP { data, sizes }
    }
}

pub trait DPIndexer {
    fn index(&self) -> usize;
    fn sizes() -> Vec<usize>;
}

#[macro_export]
macro_rules! define_dp {
    ( $type:ty, $element1:ident : $len1:expr ) => {
        impl DPIndexer for $type {
            fn index(&self) -> usize {
                self.$element1
            }
            fn sizes() -> Vec<usize> {
                vec![$len1]
            }
        }
    };
    ( $type:ty, $element1:ident : $len1:expr, $element2:ident : $len2:expr) => {
        impl DPIndexer for $type {
            fn index(&self) -> usize {
                self.$element1 * $len2 + self.$element2
            }
            fn sizes() -> Vec<usize> {
                vec![$len1, $len2]
            }
        }
    };
    ( $type:ty, $element1:ident : $len1:expr, $element2:ident : $len2:expr, $element3:ident : $len3:expr) => {
        impl DPIndexer for $type {
            fn index(&self) -> usize {
                self.$element1 * $len2 * $len3 + self.$element2 * $len3 + self.$element3
            }
            fn sizes() -> Vec<usize> {
                vec![$len1, $len2, $len3]
            }
        }
    };
    ( $type:ty, $element1:ident : $len1:expr, $element2:ident : $len2:expr, $element3:ident : $len3:expr, $element4:ident : $len4:expr) => {
        impl DPIndexer for $type {
            fn index(&self) -> usize {
                self.$element1 * $len2 * $len3 * $len4
                    + self.$element2 * $len3 * $len4
                    + self.$element3 * $len4
                    + self.$element4
            }
            fn sizes() -> Vec<usize> {
                vec![$len1, $len2, $len3, $len4]
            }
        }
    };
}

// #endregion
