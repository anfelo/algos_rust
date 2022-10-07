// # Directions
//
// Write a program that console logs the numbers from 1 to n. But for multiples
// of three print "fizz" instead of the number and for the multiples of five
// print "buzz". For numbers which are multiples of both three and five
// print "fizzbuzz"
//
// # Example
//
// - fizzbuzz(5)
//
// ```
// // 1
// // 2
// // fizz
// // 4
// // buzz
// ```

pub fn fizzbuzz(num: u8) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    }
    return num.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_buzz() {
        let result = fizzbuzz(5);

        assert_eq!(result, "buzz".to_string())
    }

    #[test]
    fn it_returns_fizz() {
        let result = fizzbuzz(3);

        assert_eq!(result, "fizz".to_string())
    }

    #[test]
    fn it_returns_num() {
        let num = 4;
        let result = fizzbuzz(num);

        assert_eq!(result, num.to_string())
    }

    #[test]
    fn it_returns_fizzbuzz() {
        let num = 15;
        let result = fizzbuzz(num);

        assert_eq!(result, "fizzbuzz".to_string())
    }
}
