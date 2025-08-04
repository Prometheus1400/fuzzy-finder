pub trait SortedExtend<T> {
    fn sorted_extend_from_slice(&mut self, slice: &[T])
    where
        T: Ord + Clone;
}

impl<T: Ord + Clone> SortedExtend<T> for Vec<T> {
    fn sorted_extend_from_slice(&mut self, slice: &[T])
    where
        T: Ord + Clone,
    {
        for item in slice {
            match self.binary_search(item) {
              Ok(index) | Err(index) => self.insert(index, item.clone()),
          }
        }
    }
}
