#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_two_sum() -> Result<(), ()> {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum::solution(nums, target), vec![0, 1]);
        Ok(())
    }

    #[test]
    fn test_merge_strings_alternate() -> Result<(), ()> {
        let str1: String = String::from("ABC");
        let str2: String = String::from("QRSYRD");
        assert_eq!(
            merge_strings_alternate::solution(str1, str2),
            String::from("AQBRCSYRD")
        );
        Ok(())
    }

    // Kids with candies

    #[test]
    fn test_kids_with_candies_1() -> Result<(), ()> {
        let candies: Vec<i32> = vec![2, 3, 5, 1, 3];
        let extra: i32 = 3;
        assert_eq!(
            kids_with_candies::solutiion(candies, extra),
            vec![true, true, true, false, true]
        );
        Ok(())
    }

    #[test]
    fn test_kids_with_candies_2() -> Result<(), ()> {
        let candies: Vec<i32> = vec![4, 2, 1, 1, 2];
        let extra: i32 = 1;
        assert_eq!(
            kids_with_candies::solutiion(candies, extra),
            vec![true, false, false, false, false]
        );
        Ok(())
    }

    #[test]
    fn test_kids_with_candies_3() -> Result<(), ()> {
        let candies: Vec<i32> = vec![12, 1, 12];
        let extra: i32 = 10;
        assert_eq!(
            kids_with_candies::solutiion(candies, extra),
            vec![true, false, true]
        );
        Ok(())
    }

    #[test]
    fn test_can_place_flowers_1() -> Result<(), ()> {
        let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 1];
        let n: i32 = 1;
        assert_eq!(can_place_flowers::solution(flowerbed, n), true);
        Ok(())
    }

    #[test]
    fn test_can_place_flowers_2() -> Result<(), ()> {
        let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 1];
        let n: i32 = 2;
        assert_eq!(can_place_flowers::solution(flowerbed, n), false);
        Ok(())
    }

    #[test]
    fn test_can_place_flowers_3() -> Result<(), ()> {
        let flowerbed: Vec<i32> = vec![1, 0, 0, 0, 0, 1];
        let n: i32 = 2;
        assert_eq!(can_place_flowers::solution(flowerbed, n), false);
        Ok(())
    }

    #[test]
    fn test_reverse_vowels_1() -> Result<(), ()> {
        let s = String::from("hello");
        assert_eq!(reverse_vowels::solution(s), String::from("holle"));
        Ok(())
    }

    #[test]
    fn test_reverse_vowels_2() -> Result<(), ()> {
        let s = String::from("leetcode");
        assert_eq!(reverse_vowels::solution(s), String::from("leotcede"));
        Ok(())
    }

    #[test]
    fn test_contains_duplicate_1() -> Result<(), ()> {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate::solution(nums), true);
        Ok(())
    }

    #[test]
    fn test_contains_duplicate_2() -> Result<(), ()> {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate::solution(nums), false);
        Ok(())
    }

    #[test]
    fn test_contains_duplicate_3() -> Result<(), ()> {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate::solution(nums), true);
        Ok(())
    }

    #[test]
    fn test_binary_search_1() -> Result<(), ()> {
        let haystack = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let needle = 14;
        assert_eq!(binary_search::solution(needle, haystack), Some(13));
        Ok(())
    }

    #[test]
    fn test_binary_search_2() -> Result<(), ()> {
        let haystack: Vec<i32> = (-1..=20).collect();
        let needle = 9;
        assert_eq!(binary_search::solution(needle, haystack), Some(4));
        Ok(())
    }

    #[test]
    fn test_binary_search_3() -> Result<(), ()> {
        let haystack = (1..=4000).collect();
        let needle = 3000;
        assert_eq!(binary_search::solution(needle, haystack), Some(2999));
        Ok(())
    }

    #[test]
    fn test_bubble_sort_1() -> Result<(), ()> {
        let mut input: Vec<i32> = vec![1, 4, 7, 4, 3];

        let result = bubble_sort::solution(&mut input);
        assert_eq!(result, vec![1, 3, 4, 4, 7]);
        Ok(())
    }
}
