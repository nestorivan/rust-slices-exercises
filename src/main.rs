// fn main() {
//     let tweet = String::from("This is my tweet and it's very very long");
//     //this works because of dref coercion
//     let trimmed_tweet: &str = trim_tweet(&tweet);

//     let tweet2 = "This is my tweet and it's very very long";
//     let trimmed_tweet2 = trim_tweet(tweet2);

//     println!("{trimmed_tweet}");
//     println!("{trimmed_tweet2}");

//     let a = [1, 2, 3, 4, 5, 6];
//     let a_slice = &a[..3];

//     print!("{:?}", a_slice);
// }

// fn trim_tweet(tweet: &str) -> &str {
//     &tweet[..20]
// }

// Complete the function definition to make the code compile.
// The output of the code should be logically correct.

// fn main() {
//     let text = String::from("Today is a very warm and sunny day.");
//     let words = ["very", "arm", "say", "sun", "dew"];
//     let mut pos;

//     println!("Text: {text}");
//     for word in words {
//         pos = find_substr_pos(&text, word);
//         if pos == text.len() {
//             println!("{word} is not present in text");
//         } else {
//             println!("{word} present at index {pos}");
//         }
//     }
// }

// // this function tries to search for substr in text from left to right
// // if it finds substr, it returns the index where it starts
// // otherwise it returns length of text (which is an invalid index)
// fn find_substr_pos(text: &str, substr: &str) -> usize {
//     // both parameters should have same data type
//     if text.len() < substr.len() {
//         return text.len();
//     }
//     let len = substr.len();

//     for start in 0..text.len() - len + 1 {
//         if substr == &text[start..start + len] {
//             // what will be the correct range?
//             return start;
//         }
//     }
//     text.len()
// }

// Fix the program so that it compiles successfully and produces the desired output.

fn main() {
    let nums = [1, 1, 2, 3, 5, 8, 13];
    let res = find_subarray(&nums[..], 16);
    if res.0 == nums.len() {
        println!("No subarray found");
    } else {
        println!("Subarray found: {:?}", &nums[res.0..res.0 + res.1]);
    }
}

// this function searches an array to find a subarray with the given sum
// it returns the index where the subarray starts along with the length of the subarray
// if the array does not include any subarray with the sum, it returns a tuple with length or array
fn find_subarray(nums: &[i32], sum: i32) -> (usize, usize) {
    // we will create a pointer to each element of the nums array
    for current_num_index in (1..nums.len() + 1).rev() {
        // we calculate the starting index of the subarray
        // using the index of the current element of the numbers array + 1
        // minus the length of the subarray
        // 0..(7-[7,6,5,4,3,2,1] + 1 )
        for subarray_index in 0..nums.len() - current_num_index + 1 {
            println!(
                "subarray_index: {:?}, current_num_index: {:?}",
                subarray_index, current_num_index
            );
            // we grab a subarray starting from the current subarray index ending at the current subarray index + the current pointer of the nums array index
            // 1..(1+7) = 1..8
            if array_sum(&nums[subarray_index..subarray_index + current_num_index]) == sum {
                return (subarray_index, current_num_index);
            }
        }
    }
    (nums.len(), nums.len())
}
fn array_sum(nums: &[i32]) -> i32 {
    // println!("nums :{:?}", nums);
    let mut res = 0;
    for num in nums {
        res += num;
    }
    res
}
