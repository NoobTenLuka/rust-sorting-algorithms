use crate::SortingDirection;

pub trait QuickSort {
    fn quick_sort(&mut self, sorting_direction: SortingDirection);
    fn _quick_sort(&mut self, left: usize, right: usize, sorting_direction: &SortingDirection);
    fn divide(&mut self, left: usize, right: usize, sorting_direction: &SortingDirection);
}

impl<T, const N: usize> QuickSort for [T; N]
where
    T: PartialOrd,
{
    fn quick_sort(&mut self, sorting_direction: SortingDirection) {
        self._quick_sort(0, self.len() - 1, &sorting_direction)
    }

    fn _quick_sort(&mut self, left: usize, right: usize, sorting_direction: &SortingDirection) {
        if match sorting_direction {
            SortingDirection::Ascending => left > right,
            SortingDirection::Descending => left < right,
        } {
            let divider = self.divide();
            self._quick_sort(left, divider - 1, sorting_direction);
            self._quick_sort(divider + 1, right, sorting_direction);
        }
    }

    fn divide(&mut self, _: usize, _: usize, _: &SortingDirection) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::quick_sort::QuickSort;

    #[test]
    fn quick_sort_asc() {
        let mut test_data = [6, 2, 9, 15, 1];
        test_data.quick_sort(crate::SortingDirection::Ascending);

        assert_eq!(test_data, [1, 2, 6, 9, 15])
    }

    #[test]
    fn quick_sort_desc() {
        let mut test_data = [6, 2, 9, 15, 1];
        test_data.quick_sort(crate::SortingDirection::Descending);

        assert_eq!(test_data, [15, 9, 6, 2, 1])
    }
}
