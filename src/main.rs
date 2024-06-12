/*fn main() {
    dbg!(is_palindrom(121));
    dbg!(is_palindrom(-121));
    dbg!(is_palindrom(10));
}
pub fn is_palindrom(x : i32 ) -> bool{
    x.to_string()==x.to_string().chars().rev().collect::<String>()
} */
// -------------- Palindrom -----------------------

/*fn find_first_occurance(arr: &[i32], target:i32)-> Option<usize>{
    for(i, &num) in arr.iter().enumerate(){
        if num == target{
            return Some(i);
        }
    }
    None
}
fn main(){
    let arr = [1,2,2,3,4,4,4,5];
    let target = 1;
    match find_first_occurance(&arr, target){
        Some(index)=> println!("the first occurance of {} is at index {}",target,index), 
        None => println!("{} not found in the array", target),
    }
}*/
// ------------------------ sorted array index-----------------------------

/*fn shortest_word(s: &str)-> Option<&str>{
    s.split_whitespace().min_by_key(|word| word.len())
}
fn main() {
    let s= "This is my internship  assignment to find the shortest word ";
    match shortest_word(s){
        Some(shortest)=>println!("The shortest word is:{}", shortest),
        None => println!("No words found"),
    }
} */
// -----------------------------Shortest word------------------------------


/*fn is_prime(n: u64) -> bool{
    if n<= 1{
        return false;
    }
    for i in 2..=n/2{
        if n% i ==0{
            return false;
        }
    }
    true
}

fn main(){
    let num = 28;
    if is_prime(num){
        print!("{} is prime",num);
    }else{
        print!("{} is not prime", num);
    }
} */

//-------------------------------prime numeber --------------------------------------

/* fn find_median(arr: &[i32]) -> f64{
    let len = arr.len();
    if len % 2 ==1{
        return  arr[len/2] as f64;
    }else{
        let mid1 = arr[len/2-1];
        let mid2 = arr[len/2];
        return (mid1 as f64+ mid2 as f64 ) / 2.0;
    }
}

fn main(){
    let arr = [1,2,3,4,5];
    println!("Median:{}", find_median(&arr));

    let arr2= [1,2,3,4,5,6];
    println!("Median:{}", find_median(&arr2));
} */

//---------------------------------median-----------------------------------------------
/*fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    for (i, char) in strings[0].chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(c) = string.chars().nth(i) {
                if c != char {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(char);
    }
    prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings));

    let strings2 = ["dddddog", "ddddracecar", "ddddcar"];
    println!("Longest common prefix: {}", longest_common_prefix(&strings2));
} */

//--------------------------------longest common prefix--------------------------

// Struct definition for binary tree

/*use std::string;



struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Function to find the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0, // If the root is None, the depth is 0
        Some(node) => {
            let left_depth = max_depth(node.left); // Recursively calculate the depth of the left subtree
            let right_depth = max_depth(node.right); // Recursively calculate the depth of the right subtree
            1 + std::cmp::max(left_depth, right_depth) // Return the maximum depth of the subtrees plus 1 for the current node
        }
    }
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));
    println!("Maximum depth of binary tree: {:?}", max_depth(root));
}*/
//-----------------------------binary tree----------------------------------------------------------------------------
/*fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let original = "hello";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}*/
//--------------------------------string reverse----------------------------------------------------------------------------

/*fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    let sqrt_num = (num as f64).sqrt() as u64;
    for i in 2..=sqrt_num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num = 29;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}*/
//--------------------prime number------------------------------------------
/*fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

fn main() {
    let arr1 = [1, 3, 5, 7, 9];
    let arr2 = [2, 4, 6, 8, 10];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}
*/
// -------------------------------------------------merge sort-------------------------------------------------
/*fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = i32::max(num, current_sum + num);
        max_sum = i32::max(max_sum, current_sum);
    }

    max_sum
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}*/

// --------------------------------------------subarray sum------------------------------------


fn main(){
    print!("Thank you")
}
