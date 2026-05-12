use std::{
    collections::HashMap,
    io::Read,
    ops::{Index, Shl},
};


    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut temp_res:Vec<i32> = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            for (i1, n1) in nums.iter().enumerate() {
                //println!("{} {}",i,i1);
                if (*n as i32 + *n1 as i32) == target && i!=i1 {
                    temp_res.push(i as i32);
                    temp_res.push(i1 as i32);          
                    return temp_res;
                }
            }
        }
        vec![]
    }
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[rustfmt::skip]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn push(&mut self, val: i32) {
        let mut current: &mut ListNode = self;
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(Box::new(ListNode { val, next: None }));
    }
    pub fn print(&self) {
        print!("[{:>3?}]", self.val);
        let mut current: &ListNode = self;
        while current.next.is_some() {
            current = current.next.as_ref().unwrap();
            print!(" -> [{:>3?}]", current.val);
        }
        println!("");
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>,l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curl: &ListNode = l1.as_ref().unwrap();
        let mut curr: &ListNode = l2.as_ref().unwrap();
        let default_node = Box::new(ListNode::new(0));
        // println!("[{:>3?}] [{:>3?}]", curl.val, curr.val);
        let a = curl.val + curr.val;
        let mut d = 0;
        let c = if a >= 10 { d = 1; a % 10 } else { a };//1
        let mut l3 = Some(Box::new(ListNode::new(c)));
        //println!("[{:>3?}]", a);
        while curl.next.is_some() || curr.next.is_some() {
            // let prevc = c;
            curl = curl.next.as_ref().unwrap_or(&default_node);
            curr = curr.next.as_ref().unwrap_or(&default_node);
            let mut b = curl.val + curr.val + d;
            b = if b >= 10 { d = 1; b % 10 } else { b };//1

            l3.as_mut().unwrap().push(b);
        }
        if d == 1 {
            l3.as_mut().unwrap().push(d);
        }
        l3
    }
}
#[rustfmt::skip]
pub fn length_of_longest_substring(s: String) -> i32 {
    //------------------------------------------------------
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut counter = 0;
    let mut win: String = String::from("");
    for c in s.chars() {
        println!("'{}'", c);
        if win.contains(c) {
            let pp = if let Some(pos) = win.find(c) { pos } else { 0 };
            counter = 0;
            counts.insert(win.clone(), win.len());//1

            win.drain(..pp + 1);//1

            counter += 1;
            win.push(c);
            //win.shrink_to_fit();
        } else {
            counter += 1;
            win.push(c);
        }
    }
    counts.insert(win.clone(), win.len());//1
    let mut max = 0;
    for (_len, string) in counts {
        println!("'{}': {}", _len, string);
        if string > max {
            max = string;
        }
    }
    max as i32
    //------------------------------------------------------
    //------------------------------------------------------
}
// use core::f64::*;
//     // let start = if let Some(o1) = n1.get(0) { *o1 } else { 0 };
// let end = if let Some(o2) = n1.get(n1.len() - 1) {
//     *o2
// } else {
//     0
// };
// let p = (end as f64 / 2.0 + start as f64 / 2.0);
// let p1 = if let Some(o3) = n1.get(p as usize) {
//     *o3
// } else {
//     0
// };
//     //println!("res {}", p1);
// let sz = n1.clone().into_iter().filter(|&x| x == 0).count();
// if (sz == n1.len()) {
//     p = 0.0;
// }
// let pp = n1.len() % 2;
// if (pp > 0) {
//     println!("остаток {}", pp);
//     let o1 = n1.len() / 2;
//     let o3 = (o1 as f64) / 2.0 as f64 + 0.5;
//     if let Some(oo) = n1.get(o3 as usize) {
//         p = *oo as f64;
//     };
// } else {
//     println!("целое {}", pp);
//     let o1 = n1.len() / 2;
//     let o2 = o1 + 1;
//     p = (o1 as f64 + o2 as f64) / 2.0 as f64;
//     println!("result {}", p);
// }
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut n1 = nums1.clone(); //1
    n1.extend(nums2.clone()); //1
    let mut p: f64 = 0.0;
    n1.sort();

    let k = n1.len() % 2;
    if k == 0 {
        let k3 = n1.len() / 2;
        let k1 = (k3 as f64 - 0.5) as usize;
        let k2 = (k3 as f64 + 0.5) as usize;
        let st = if let Some(o1) = n1.get(k1) { *o1 } else { 0 };
        let en = if let Some(o2) = n1.get(k2) { *o2 } else { 0 };
        p = (st as f64 + en as f64) / 2.0;
        println!("{} {} {} {} {} {}", n1.len(), k, k1, k2, st, en);
    } else {
        let k3 = n1.len() / 2;
        let k1 = (k3 as f64) as usize;
        let k2 = (k3 as f64) as usize;
        let st = if let Some(o1) = n1.get(k1) { *o1 } else { 0 };
        let en = if let Some(o2) = n1.get(k2) { *o2 } else { 0 };
        p = (st as f64 + en as f64) / 2.0;
        println!("{} {} {} {} {} {}", n1.len(), k, k1, k2, st, en);
    }
    println!("{}", p);

    p
}

// let k = s1.len() % 2;
// if k == 0 {
//     let k3 = s1.len() / 2;
//     let k1 = (k3 as f64 - 0.5) as usize;
//     let k2 = (k3 as f64 + 0.5) as usize;
//     // let st = if let Some(o1) = s1.as_bytes().get(k1) {
//     //     *o1
//     // } else {
//     //     0
//     // };
//     // let en = if let Some(o2) = s1.as_bytes().get(k2) {
//     //     *o2
//     // } else {
//     //     0
//     // };
//     // p = (st as f64 + en as f64) / 2.0;
//     println!("1 {} {}", k1, k2);
// } else {
//     let k3 = s1.len() / 2;
//     let k1 = (k3 as f64) as usize;
//     let k2 = (k3 as f64) as usize;
//     // let st = if let Some(o1) = s1.as_bytes().get(k1) {
//     //     *o1
//     // } else {
//     //     0
//     // };
//     // let en = if let Some(o2) = s1.as_bytes().get(k2) {
//     //     *o2
//     // } else {
//     //     0
//     // };
//     // p = (st as f64 + en as f64) / 2.0;
//     println!("2 {} {}", k1, k2);
// }

pub fn longest_palindrome(s: String) -> String {
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    let mut st = 0;
    let mut en = s.len();
    let mut st1 = 0;
    let mut en1 = s.len();
    for i2 in 0..s.len() {
        for i1 in st1..en1 {
            if st == en {
                st += 1;
                en = s.len();
                st1 += 1;
                en1 = s.len();
                continue;
            }

            let s2 = s[st..en].chars();
            let s3 = s2.clone().rev();
            if s2.eq(s3) {
                let strr = s[st..en].chars().as_str().to_string();
                counts.insert(strr, s[st..en].len());
            }
            en -= 1;
        }
    }
    let mut max = 0;
    let mut strR = String::from("");
    for (len, string) in counts {
        if string > max {
            max = string;
            strR = len;
        }
    }
    strR
}

    pub fn convert(s: String, num_rows: i32) -> String {
            let n:i32 = s.len() as i32;
            let mut rr=String::from("");
            if num_rows > 1 && s.len() > 1{
            let chars: Vec<char> = s.chars().collect();
            
            let cycle:i32 = (num_rows - 1) * 2; // Для 3 строк это 4
            
            for r in 0..num_rows {
                for i in (r..n).step_by(cycle as usize) {
                    //print!("{}", chars[i as usize]); // Основной символ колонки
                    rr.push(chars[i as usize]);
                    // Если строка средняя, нужно добавить промежуточный символ
                    let second_j = i + cycle - 2 * r;
                    if r != 0 && r != num_rows - 1 && second_j < n {
                        //print!("{}", chars[second_j as usize]);
                        rr.push(chars[second_j as usize]);
                    }
                }
            }
            return rr;
            }
            else {return s;};
            
    }
pub fn my_atoi(s: String) -> i32 {
    let s1: Vec<_> = s.chars().collect();
    let mut s2=String::from("");
    // println!("{:?}",s1);
    // let mut counter = 0;
    if s1[0] >= '0' && s1[0] <= '9'||s1[0]==' ' {
        //s1[0] >='!' && s1[0] < '0'||s1[0]>'9'&&s1[0]<='~'
        for i in &s1 {
            if *i==' '{continue;}
            if *i >= '0' && *i <= '9'|| *i=='-' {
                // println!("{}", *i);
                s2.push(*i);
                // counter += 1;
            }
            else {
                break;
            }
        }
    }
    let op = s2.parse::<i32>().unwrap_or(0);
    // println!("{}",op);
    op
}
pub fn reverse(x: i32) -> i32 {
    let p = x.to_string();
    let mut p1 = String::from("");
    let mut b = false;
    for i in 0..p.len() {
        if let Some(pp) = p.chars().nth(i) {
            if pp != '-' && i == 0 {
                if let Some(pp1) = p.chars().nth(0) {
                    // p1.push(pp1);
                    b = true;
                    continue;
                };
            }
            if pp == '-' && i == 0 {
                p1.push(pp);
            } else if pp != '-' && i != 0 {
                if let Some(pp1) = p.chars().nth(p.len() - i) {
                    p1.push(pp1);
                };
            }
        };
    }
    if b == true {
        if let Some(pp1) = p.chars().nth(0) {
            // println!("{}", pp1);
            p1.push(pp1);
            // b = true;
        };
    }
    let p2 = p1.parse::<i32>();
    println!("{}", p2.clone().unwrap_or(0));
    p2.clone().unwrap_or(0)
}

pub fn is_palindrome(x: i32) -> bool {
    let a = x.to_string();
    let b = x.to_string();
    let c = b.chars().rev().clone();
    let d = a.chars();
    if c.eq(d) { true } else { false }
}

//190
pub fn reverse_bits(n: i32) -> i32 {
    n.reverse_bits()
}

//2119
pub fn is_same_after_reversals(num: i32) -> bool {
    let p = num.to_string();
    let mut p1 = String::from("");
    let mut b = false;
    let mut cn = 0;
    let mut nn = 0;
    let mut bn = false;
    for i in 0..p.len() {
        if let Some(pp) = p.chars().nth(i) {
            if pp != '-' && i == 0 {
                if let Some(pp1) = p.chars().nth(0) {
                    // p1.push(pp1);
                    b = true;
                    continue;
                };
            }
            if pp == '-' && i == 0 {
                p1.push(pp);
            } else if pp != '-' && i != 0 {
                if let Some(pp1) = p.chars().nth(p.len() - i) {
                    if p.len() == 1 && pp1 == '0' {
                        p1.push(pp1);
                    } else if p.len() > 1 && pp1 == '0' && bn == false {
                        cn += 1;
                    } else if p.len() > 1 && pp1 != '0' {
                        nn += 1;
                        p1.push(pp1);
                        bn = true;
                    } else if p.len() > 1 && bn == true {
                        //nn+=1;
                        p1.push(pp1);
                        //bn=true;
                    }
                };
            }
        };
    }
    if b == true {
        if let Some(pp1) = p.chars().nth(0) {
            // println!("{}", pp1);
            p1.push(pp1);
            // b = true;
        };
    }
    let p3 = p1;
    let mut p2 = String::from("");
    let mut b = false;
    for i in 0..p3.len() {
        if let Some(pp) = p3.chars().nth(i) {
            if pp != '-' && i == 0 {
                if let Some(pp1) = p3.chars().nth(0) {
                    // p1.push(pp1);
                    b = true;
                    continue;
                };
            }
            if pp == '-' && i == 0 {
                p2.push(pp);
            } else if pp != '-' && i != 0 {
                if let Some(pp1) = p3.chars().nth(p3.len() - i) {
                    p2.push(pp1);
                };
            }
        };
    }
    if b == true {
        if let Some(pp1) = p3.chars().nth(0) {
            // println!("{}", pp1);
            p2.push(pp1);
            // b = true;
        };
    }
    let p4 = p2.parse::<i32>();
    println!("{}", p4.clone().unwrap_or(0));
    let i2: i32 = p4.clone().unwrap_or(0);
    i2.eq(&num)
}

pub fn main() {
    //1
    // let v: Vec<i32> = vec![3, 2, 4];
    // let p = two_sum(v, 6);
    // for (i, ii) in p.iter().enumerate() {
    //     println!("{}", ii)
    // }
    // l1:Option<Box<ListNode>> = [2,4,3]; l2:Option<Box<ListNode>> = [5,6,4];
    //2
    // let mut l1 = Some(Box::new(ListNode::new(9)));
    // l1.as_mut().unwrap().push(9);
    // l1.as_mut().unwrap().push(9);
    // l1.as_mut().unwrap().push(9);
    // l1.as_mut().unwrap().push(9);
    // l1.as_mut().unwrap().push(9);
    // l1.as_mut().unwrap().push(9);
    // let mut l2 = Some(Box::new(ListNode::new(9)));
    // l2.as_mut().unwrap().push(9);
    // l2.as_mut().unwrap().push(9);
    // l2.as_mut().unwrap().push(9);

    // let mut t = ListNode::add_two_numbers(l1, l2);
    // t.as_ref().unwrap().print();
    //3
    // let mut p = length_of_longest_substring(String::from("dvdf"));
    // println!("{}", p);
    // 4
    // find_median_sorted_arrays(vec![1, 2], vec![2, 4, 2]);
    //5
    //let pp = longest_palindrome(String::from(
    //    "zudfweormatjycujjirzjpyrmaxurectxrtqedmmgergwdvjmjtstdhcihacqnothgttgqfywcpgnuvwglvfiuxteopoyizgehkwuvvkqxbnufkcbodlhdmbqyghkojrgokpwdhtdrwmvdegwycecrgjvuexlguayzcammupgeskrvpthrmwqaqsdcgycdupykppiyhwzwcplivjnnvwhqkkxildtyjltklcokcrgqnnwzzeuqioyahqpuskkpbxhvzvqyhlegmoviogzwuiqahiouhnecjwysmtarjjdjqdrkljawzasriouuiqkcwwqsxifbndjmyprdozhwaoibpqrthpcjphgsfbeqrqqoqiqqdicvybzxhklehzzapbvcyleljawowluqgxxwlrymzojshlwkmzwpixgfjljkmwdtjeabgyrpbqyyykmoaqdambpkyyvukalbrzoyoufjqeftniddsfqnilxlplselqatdgjziphvrbokofvuerpsvqmzakbyzxtxvyanvjpfyvyiivqusfrsufjanmfibgrkwtiuoykiavpbqeyfsuteuxxjiyxvlvgmehycdvxdorpepmsinvmyzeqeiikajopqedyopirmhymozernxzaueljjrhcsofwyddkpnvcvzixdjknikyhzmstvbducjcoyoeoaqruuewclzqqqxzpgykrkygxnmlsrjudoaejxkipkgmcoqtxhelvsizgdwdyjwuumazxfstoaxeqqxoqezakdqjwpkrbldpcbbxexquqrznavcrprnydufsidakvrpuzgfisdxreldbqfizngtrilnbqboxwmwienlkmmiuifrvytukcqcpeqdwwucymgvyrektsnfijdcdoawbcwkkjkqwzffnuqituihjaklvthulmcjrhqcyzvekzqlxgddjoir",
    //));
    //println!("{}", pp);
    // 6
    //convert(String::from("PAYPALISHIRING"), 3);
    // 7
    reverse(1534236469);
}
