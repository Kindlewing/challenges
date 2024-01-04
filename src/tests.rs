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
}
