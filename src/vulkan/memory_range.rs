#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FreeMemoryRange {
    start: u64,
    end: u64,
}

impl FreeMemoryRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }

    pub fn contains_or_is_neighbor(&self, value: u64) -> bool {
        let start = if self.start == 0 {
            self.start
        } else {
            self.start - 1
        };
        value >= start && value <= self.end + 1
    }

    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn maybe_merge_with(&self, other: Self) -> Option<Self> {
        if self.contains_or_is_neighbor(other.start) {
            Some(Self::new(self.start, other.end))
        } else if other.contains_or_is_neighbor(self.start) {
            Some(Self::new(other.start, self.end))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FreeMemoryRanges {
    start: u64,
    end: u64,
    ranges: Vec<FreeMemoryRange>,
}

impl FreeMemoryRanges {
    pub fn fully_occupied(start: u64, end: u64) -> crate::Result<Self> {
        crate::verify!(start <= end, "{start} must be less than or equal to {end}");
        Ok(Self {
            start,
            end,
            ranges: vec![],
        })
    }

    pub fn fully_free(start: u64, end: u64) -> crate::Result<Self> {
        crate::verify!(start <= end, "{start} must be less than or equal to {end}");
        Ok(Self {
            start,
            end,
            ranges: vec![FreeMemoryRange::new(start, end)],
        })
    }

    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn free_len(&self) -> u64 {
        self.ranges
            .iter()
            .fold(0, |accum, range| accum + range.len())
    }

    pub fn unfree_len(&self) -> u64 {
        self.len() - self.free_len()
    }

    pub fn unfree_range(&mut self, start: u64, end: u64) -> crate::Result<()> {
        crate::verify!(start <= end, "{start} must be less than or equal to {end}");
        crate::verify!(start >= self.start, "{start} must be inside the range");
        crate::verify!(end <= self.end, "{end} must be inide the range");

        if self.start == start && self.end == end {
            self.ranges = vec![];
        } else {
            let mut new_ranges = Vec::with_capacity(self.ranges.len());
            let mut add_range_if_valid = |range: FreeMemoryRange| {
                if range.start() <= range.end() {
                    new_ranges.push(range);
                }
            };
            let clamp_unsigned_subtract = |a, b| {
                if a < b {
                    0
                } else {
                    a - b
                }
            };

            for range in self.ranges.iter() {
                let mut unmodified_range = true;

                // If only start is inside the range, split this range before it
                if range.contains(start) {
                    add_range_if_valid(FreeMemoryRange::new(range.start, clamp_unsigned_subtract(start, 1)));
                    unmodified_range = false;
                }

                // If only end is inside the range, split this range after it
                if range.contains(end) {
                    add_range_if_valid(FreeMemoryRange::new(end + 1, range.end));
                    unmodified_range = false;
                }

                // Otherwise, just add this range to the list
                if unmodified_range {
                    add_range_if_valid(*range);
                }
            }
            self.ranges = new_ranges;
        }
        Ok(())
    }

    fn merge_ranges(&mut self) {
        let mut new_ranges: Vec<FreeMemoryRange> = Vec::new();

        // This algorithm is O(n²) if the memory ranges are extremely fragmented.
        // This shouldn't happen that much though, because I'm using a linear allocator.
        for range in self.ranges.iter() {
            let mut new_ranges_updated = false;
            for new_range in new_ranges.iter_mut() {
                if let Some(merged) = new_range.maybe_merge_with(*range) {
                    *new_range = merged;
                    new_ranges_updated = true;
                    break;
                }
            }

            if !new_ranges_updated {
                new_ranges.push(*range);
            }
        }
        self.ranges = new_ranges;
    }

    pub fn free_range(&mut self, start: u64, end: u64) -> crate::Result<()> {
        crate::verify!(start <= end, "{start} must be less than or equal to {end}");
        crate::verify!(start >= self.start, "{start} must be inside the range");
        crate::verify!(end <= self.end, "{end} must be inide the range");

        if self.start == start && self.end == end {
            self.ranges = vec![FreeMemoryRange::new(start, end)];
        } else {
            self.ranges.push(FreeMemoryRange::new(start, end));
            self.merge_ranges();
        }

        Ok(())
    }

    pub fn find_range_that_can_fit_width(&self, width: u64) -> Option<FreeMemoryRange> {
        for range in self.ranges.iter() {
            if range.len() >= width {
                return Some(*range);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_of_one_wide_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_free(0, 0).unwrap();
        assert_eq!(1, memory_range.len());
    }

    #[test]
    fn len_of_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_free(31, 90).unwrap();
        assert_eq!(60, memory_range.len());
    }

    #[test]
    fn free_len_of_one_wide_fully_free_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_free(0, 0).unwrap();
        assert_eq!(1, memory_range.free_len());
    }

    #[test]
    fn free_len_of_one_wide_occupied_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_occupied(0, 0).unwrap();
        assert_eq!(0, memory_range.free_len());
    }

    #[test]
    fn free_len_of_fully_free_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_free(5, 66).unwrap();
        assert_eq!(62, memory_range.free_len());
    }

    #[test]
    fn free_len_of_fragmented_ranges_is_correct() {
        let mut range = FreeMemoryRanges::fully_free(10, 1000).unwrap();
        range.unfree_range(700, 700).unwrap();
        range.unfree_range(15, 100).unwrap();
        range.unfree_range(75, 200).unwrap();
        range.unfree_range(750, 800).unwrap();
        range.unfree_range(500, 600).unwrap();

        // ranges: vec![
        //      FreeMemoryRange::new(10, 14),
        //      FreeMemoryRange::new(201, 499),
        //      FreeMemoryRange::new(601, 699),
        //      FreeMemoryRange::new(701, 749),
        //      FreeMemoryRange::new(801, 1000),
        // ],
        assert_eq!(5 + 299 + 99 + 49 + 200, range.free_len());
    }

    #[test]
    fn unfree_len_of_one_wide_fully_free_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_free(9, 9).unwrap();
        assert_eq!(0, memory_range.unfree_len());
    }

    #[test]
    fn unfree_len_of_one_wide_occupied_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_occupied(0, 0).unwrap();
        assert_eq!(1, memory_range.unfree_len());
    }

    #[test]
    fn unfree_len_of_fully_free_ranges_is_correct() {
        let memory_range = FreeMemoryRanges::fully_free(5, 66).unwrap();
        assert_eq!(0, memory_range.unfree_len());
    }

    #[test]
    fn unfree_len_of_fragmented_ranges_is_correct() {
        let mut range = FreeMemoryRanges::fully_free(10, 600).unwrap();
        range.unfree_range(100, 100).unwrap();
        range.unfree_range(95, 100).unwrap();
        range.unfree_range(250, 300).unwrap();
        range.unfree_range(34, 104).unwrap();
        range.unfree_range(559, 600).unwrap();

        // ranges: vec![
        //      FreeMemoryRange::new(10, 33),
        //      FreeMemoryRange::new(105, 249),
        //      FreeMemoryRange::new(301, 558),
        // ],
        let expected = (600 - 10 + 1) - ((33 - 10 + 1) + (249 - 105 + 1) + (558 - 301 + 1));
        assert_eq!(expected, range.unfree_len());
    }

    #[test]
    fn merging_neighboring_ranges_results_in_correct_result() {
        let first = FreeMemoryRange::new(0, 25);
        let second = FreeMemoryRange::new(14, 56);
        let expected = FreeMemoryRange::new(0, 56);
        assert_eq!(Some(expected), first.maybe_merge_with(second));
        assert_eq!(Some(expected), second.maybe_merge_with(first));
    }

    #[test]
    fn merging_non_neighboring_ranges_results_in_none_value() {
        let first = FreeMemoryRange::new(0, 56);
        let second = FreeMemoryRange::new(146, 891);
        assert_eq!(None, first.maybe_merge_with(second));
        assert_eq!(None, second.maybe_merge_with(first));
    }

    #[test]
    fn unfreeing_entire_range_results_in_empty_range() {
        let mut range = FreeMemoryRanges::fully_free(0, 64).unwrap();
        range.unfree_range(0, 64).unwrap();
        let expected = FreeMemoryRanges::fully_occupied(0, 64).unwrap();
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_range_from_start_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(5, 102).unwrap();
        range.unfree_range(5, 64).unwrap();
        let expected = FreeMemoryRanges {
            start: 5,
            end: 102,
            ranges: vec![FreeMemoryRange::new(65, 102)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_range_from_start_multiple_times_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(5, 102).unwrap();
        range.unfree_range(5, 64).unwrap();
        range.unfree_range(5, 100).unwrap();
        let expected = FreeMemoryRanges {
            start: 5,
            end: 102,
            ranges: vec![FreeMemoryRange::new(101, 102)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_range_to_end_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(12, 96).unwrap();
        range.unfree_range(66, 96).unwrap();
        let expected = FreeMemoryRanges {
            start: 12,
            end: 96,
            ranges: vec![FreeMemoryRange::new(12, 65)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_inbetween_range_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(6, 400).unwrap();
        range.unfree_range(55, 100).unwrap();
        let expected = FreeMemoryRanges {
            start: 6,
            end: 400,
            ranges: vec![FreeMemoryRange::new(6, 54), FreeMemoryRange::new(101, 400)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_range_to_end_and_then_inbetween_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(40, 1300).unwrap();
        range.unfree_range(1200, 1300).unwrap();
        range.unfree_range(100, 400).unwrap();

        let expected = FreeMemoryRanges {
            start: 40,
            end: 1300,
            ranges: vec![
                FreeMemoryRange::new(40, 99),
                FreeMemoryRange::new(401, 1199),
            ],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_random_inbetween_ranges_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(10, 1000).unwrap();
        range.unfree_range(700, 700).unwrap();
        range.unfree_range(15, 100).unwrap();
        range.unfree_range(75, 200).unwrap();
        range.unfree_range(750, 800).unwrap();
        range.unfree_range(500, 600).unwrap();

        let expected = FreeMemoryRanges {
            start: 10,
            end: 1000,
            ranges: vec![
                FreeMemoryRange::new(10, 14),
                FreeMemoryRange::new(201, 499),
                FreeMemoryRange::new(601, 699),
                FreeMemoryRange::new(701, 749),
                FreeMemoryRange::new(801, 1000),
            ],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_overlapping_ranges_from_same_start_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(4, 200).unwrap();
        range.unfree_range(4, 95).unwrap();
        range.unfree_range(4, 145).unwrap();

        let expected = FreeMemoryRanges {
            start: 4,
            end: 200,
            ranges: vec![FreeMemoryRange::new(146, 200)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_overlapping_ranges_first_with_width_one_and_second_until_first_results_in_correct_ranges(
    ) {
        let mut range = FreeMemoryRanges::fully_free(6, 200).unwrap();
        range.unfree_range(94, 94).unwrap();
        range.unfree_range(12, 94).unwrap();

        let expected = FreeMemoryRanges {
            start: 6,
            end: 200,
            ranges: vec![FreeMemoryRange::new(6, 11), FreeMemoryRange::new(95, 200)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn unfreeing_overlapping_ranges_from_different_start_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_free(4, 200).unwrap();
        range.unfree_range(4, 95).unwrap();
        range.unfree_range(59, 145).unwrap();

        let expected = FreeMemoryRanges {
            start: 4,
            end: 200,
            ranges: vec![FreeMemoryRange::new(146, 200)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn freeing_entire_range_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_occupied(6, 109).unwrap();
        range.free_range(6, 109).unwrap();
        let expected = FreeMemoryRanges::fully_free(6, 109).unwrap();
        assert_eq!(expected, range);
    }

    #[test]
    fn freeing_range_from_start_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_occupied(7, 96).unwrap();
        range.free_range(7, 55).unwrap();
        let expected = FreeMemoryRanges {
            start: 7,
            end: 96,
            ranges: vec![FreeMemoryRange::new(7, 55)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn freeing_range_from_end_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_occupied(46, 204).unwrap();
        range.free_range(150, 204).unwrap();
        let expected = FreeMemoryRanges {
            start: 46,
            end: 204,
            ranges: vec![FreeMemoryRange::new(150, 204)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn freeing_range_from_end_multiple_times_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_occupied(601, 1000).unwrap();
        range.free_range(705, 1000).unwrap();
        range.free_range(622, 1000).unwrap();
        let expected = FreeMemoryRanges {
            start: 601,
            end: 1000,
            ranges: vec![FreeMemoryRange::new(622, 1000)],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn freeing_random_inbetween_ranges_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_occupied(41, 2000).unwrap();
        range.free_range(300, 300).unwrap();
        range.free_range(45, 100).unwrap();
        range.free_range(85, 110).unwrap();
        range.free_range(711, 800).unwrap();
        range.free_range(520, 600).unwrap();

        let expected = FreeMemoryRanges {
            start: 41,
            end: 2000,
            ranges: vec![
                FreeMemoryRange::new(300, 300),
                FreeMemoryRange::new(45, 110),
                FreeMemoryRange::new(711, 800),
                FreeMemoryRange::new(520, 600),
            ],
        };
        assert_eq!(expected, range);
    }

    #[test]
    fn freeing_neighboring_ranges_results_in_correct_ranges() {
        let mut range = FreeMemoryRanges::fully_occupied(0, 1000).unwrap();
        range.free_range(0, 100).unwrap();
        range.free_range(101, 200).unwrap();

        let expected = FreeMemoryRanges {
            start: 0,
            end: 1000,
            ranges: vec![FreeMemoryRange::new(0, 200)],
        };
        assert_eq!(expected, range);
    }
}
