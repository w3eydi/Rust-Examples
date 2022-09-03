pub mod sorting {
    
    pub fn selection_sort<T: Ord>(array: &mut [T]) {
        let len = array.len();
        
        for i in 0..len {
            let mut min_id = i;

            for j in (i + 1)..len {
                if array[j] < array[min_id] {
                    min_id = j;
                }
            }

            array.swap(min_id, i);
        }
    }

    pub fn bubble_sort<T: Ord>(array: &mut [T]) {
        let len = array.len();

        for i in 0..len {

            for j in 0..len -i -1 {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod sorting_tests {
    use super::sorting::*;

    #[test]
    fn selection_sort_test() {
        let mut a = [4, 5, 1, 2, 3, -1, -5, 9];
        selection_sort(&mut a);
        assert_eq!(a, [-5, -1, 1, 2, 3, 4, 5, 9]);
    }

    #[test]
    fn bubble_sort_test() {
        let mut a = [4, 5, 1, 2, 3, -1, -5, 9];
        bubble_sort(&mut a);
        assert_eq!(a, [-5, -1, 1, 2, 3, 4, 5, 9]);
    }
}