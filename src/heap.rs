
pub struct Heap<T: Ord> {
    heap: Vec<T>
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Heap { 
            heap: Vec::new()
        }
    }

    pub fn new_from_vector(mut init_vals: Vec<T>) -> Self {
        Self::heapify(&mut init_vals);
        Heap {
            heap: init_vals
        }
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn push(&mut self, item: T) {
        let new_pos = self.heap.len();
        self.heap.push(item);
        Self::sift_up(&mut self.heap, new_pos);
    }

    pub fn pop(&mut self) -> Option<T> {
        let length = self.heap.len();
        if length > 0 {
            self.heap.swap(0, length - 1);
            let ret = self.heap.pop();
            Self::sift_down(&mut self.heap, 0);
            ret
        } else {
            None
        }
    }

    pub fn drain_sorted(&mut self) -> HeapOrderedDrainIterator<T> {
        HeapOrderedDrainIterator {
            heap: self
        }
    }

    pub fn drain(&mut self) -> std::vec::Drain<'_, T> {
        self.heap.drain(..)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.heap.iter()
    }

    fn sift_up(arr: &mut Vec<T>, index: usize) {
        let mut cur = index;
        let mut swapped = true;
        while swapped {
            swapped = if let Some(parent) = Self::get_parent(cur) {
                if arr[cur] > arr[parent]{
                    arr.swap(cur, parent);
                    cur = parent;
                    true
                } else {
                    false
                }
            } else { false };
        }
    }

    fn sift_down(arr:&mut Vec<T>, index: usize) {
        let mut cur = index;
        let mut swapped = true;
        while swapped {
            swapped = match Self::get_children(cur, arr.len()) {
                (Some(l), Some(r)) => {
                    let big = if arr[l] > arr[r] { l } else { r };
                    if arr[big] > arr[cur] {
                        arr.swap(big, cur);
                        cur = big;
                        true
                    } else {
                        false
                    }
                },
                (Some(l), None) => {
                    if arr[l] > arr[cur] {
                        arr.swap(l, cur);
                        cur = l;
                    }
                    false
                },
                (None, Some(_)) => unreachable!("Somehow this node has a right child but not a left child, get_children is wrong"),
                (None, None) => false
            }
        }
    }

    fn get_parent(index: usize) -> Option<usize> {
        if index > 0 {
            Some((index - 1)/2)
        } else {
            None
        }
    }

    fn get_children(index: usize, total_size: usize) -> (Option<usize>, Option<usize>) {
        let left_index = (2*index) + 1;
        let right_index = (2*index) + 2;
        let left = if left_index < total_size { Some(left_index) } else { None };
        let right = if right_index < total_size { Some(right_index) } else { None };
        (left, right)
    }

    fn heapify(arr: &mut Vec<T>) {
        let mut stack = (0..(arr.len()/2)).collect::<Vec<usize>>();
        while let Some(cur) = stack.pop() {
            Self::sift_down(arr, cur);
        }
    }
}

pub struct HeapOrderedDrainIterator<'a, T: Ord> {
    heap: &'a mut Heap<T>
}

impl<'a, T: Ord> Iterator for HeapOrderedDrainIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use rand::seq::SliceRandom;

    #[test]
    fn empty_heap() {
        let mut heap = Heap::<i32>::new();
        assert_eq!(heap.size(), 0);
        assert!(heap.pop().is_none());
    }

    #[test]
    fn pushing_multiple_should_get_largest_from_pop() {
        let mut heap = Heap::new();
        let mut shuffled_values = (0..45).collect::<Vec<i32>>();
        shuffled_values.shuffle(&mut thread_rng());
        for i in shuffled_values {
            heap.push(i);
        }

        let mut last = if let Some(val) = heap.pop() { val } else { unreachable!("Heap should have values to pop here, something went wrong...") };
        while let Some(val) = heap.pop() {
            assert!(last >= val);
            last = val;
        }
    }

    #[test]
    fn pushing_multiple_should_get_smallest_from_pop() {
        let mut heap = Heap::new();
        let mut shuffled_values = (0..45).collect::<Vec<i32>>();
        shuffled_values.shuffle(&mut thread_rng());
        for i in shuffled_values {
            heap.push(core::cmp::Reverse(i));
        }

        let mut last = if let Some(val) = heap.pop() { val } else { unreachable!("Heap should have values to pop here, something went wrong...") };
        while let Some(val) = heap.pop() {
            assert!(last.0 <= val.0);
            last = val;
        }
    }

    #[test]
    fn initial_vector_heapify() {
        let mut shuffled_values = (0..45).collect::<Vec<i32>>();
        shuffled_values.shuffle(&mut thread_rng());
        let mut heap = Heap::new_from_vector(shuffled_values);

        let mut last = if let Some(val) = heap.pop() { val } else { unreachable!("Heap should have values to pop here, something went wrong...") };
        while let Some(val) = heap.pop() {
            assert!(last >= val);
            last = val;
        }
    }

    #[test]
    fn initial_vector_heapify_min_heap() {
        let mut shuffled_values = (0..45).map(|x| core::cmp::Reverse(x)).collect::<Vec<core::cmp::Reverse<i32>>>();
        shuffled_values.shuffle(&mut thread_rng());
        let mut heap = Heap::new_from_vector(shuffled_values);

        let mut last = if let Some(val) = heap.pop() { val } else { unreachable!("Heap should have values to pop here, something went wrong...") };
        while let Some(val) = heap.pop() {
            assert!(last.0 <= val.0);
            last = val;
        }
    }
}
