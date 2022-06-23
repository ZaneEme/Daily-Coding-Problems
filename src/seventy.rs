/**
 * A number is considered perfect if its digits sum up to exactly 10.
 * Given a positive integer n, return the n-th perfect number.
 * For example, given 1, you should return 19. Given 2, you should return 28.
**/

fn find_sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;

    while num != 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

pub fn seventy(num_to_find: i32) {
    let mut counter: i32 = 0;
    let mut curr_num = 0;

    while counter < num_to_find {
        curr_num += 1;
        if find_sum_of_digits(curr_num) == 10 {
            counter += 1;
        }
    }

    println!("The {}th perfect number is {}.", counter, curr_num);
}
