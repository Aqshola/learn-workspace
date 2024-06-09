use std::collections::HashMap;

fn main() {

    // implement_two_sum();
    implement_is_palindrome();
}

#[allow(dead_code)]
fn implement_two_sum() {
    let data = vec![3, 3];
    let target = 7;

    let res = two_sum(data, target);

    println!("{:?}", res);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dict_num: HashMap<i32, i32> = HashMap::new();

    for (i, &curr) in nums.iter().enumerate() {
        let sum = target - curr;

        let exist_dict = dict_num.get(&sum);

        if exist_dict != None {
            return vec![*exist_dict.unwrap(), i as i32];
        }

        dict_num.insert(curr, i as i32);
    }

    vec![0, 0]
}

fn implement_is_palindrome(){
    let tes=is_palindrome(-121);

    println!("{}",tes);    
}

pub fn is_palindrome(x: i32) -> bool {
    let str_x = x.to_string();
    if str_x.len()==2{
        let mut chars_strx = str_x.chars();
        let first=chars_strx.next().unwrap();
        let last= chars_strx.last().unwrap();

        println!("{},{}",first,last);

        return first==last
    }


    for i in 0..((str_x.len()/2)){
        
        let first=str_x.chars().nth(i).unwrap();
        let last=str_x.chars().nth(str_x.len()-1-i).unwrap();

        if first!=last{
            return false;
        }

    }
    true
}
