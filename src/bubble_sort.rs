use crate::SortingDirection;

pub trait BubbleSort {
    fn bubble_sort(&mut self, sorting_direction: SortingDirection);
}

impl<T, const N: usize> BubbleSort for [T; N]
where
    T: PartialOrd,
{
    fn bubble_sort(&mut self, sorting_direction: SortingDirection) {
        for n in (2..=self.len()).rev() {
            for i in 0..n - 1 {
                if match sorting_direction {
                    SortingDirection::Ascending => self[i] > self[i + 1],
                    SortingDirection::Descending => self[i] < self[i + 1],
                } {
                    self.swap(i, i + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort::BubbleSort;

    #[test]
    fn bubble_sort_asc() {
        let mut test_data = [6, 2, 9, 15, 1];
        test_data.bubble_sort(crate::SortingDirection::Ascending);

        assert_eq!(test_data, [1, 2, 6, 9, 15])
    }

    #[test]
    fn bubble_sort_desc() {
        let mut test_data = [6, 2, 9, 15, 1];
        test_data.bubble_sort(crate::SortingDirection::Descending);

        assert_eq!(test_data, [15, 9, 6, 2, 1])
    }
}
