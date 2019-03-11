fn main() {
    println!("{}", areSimilar(vec![2,3,1], vec![1,3,2]));
}

fn areSimilar(a: Vec<i32>, b: Vec<i32>) -> bool {                               // [2,3,1], [1,3,2]
    let mut iter_a = a.into_iter();
    let mut iter_b = b.into_iter();
    let mut swapped = false;
    let mut duplicated = false;
    let mut already_compared_a = std::i32::MIN;
    let mut already_compared_b = std::i32::MIN;

    loop {
        match (iter_a.next(), iter_b.next()) {                                  // Some(2), Some(1) -> Some(3), Some(3) -> (None, None)
            (Some(cmp_a), Some(cmp_b)) => {                                     // Some(2), Some(1) -> Some(3), Some(3)
                if cmp_a == cmp_b {                                             // skip             -> true
                    continue;                                                   // skip             -> continue
                } else if cmp_a == already_compared_a && cmp_b == already_compared_b {
                    if duplicated {
                        return false;
                    } else {
                        duplicated = true;
                    }
                    continue;
                } else {
                    if swapped {                                                // skip
                        return false;                                           // skip
                    } else {
                        let mut count = 0usize;                                 // count = 0
                        
                        let mut iter_a_cloned = iter_a.clone();
                        let mut iter_b_cloned = iter_b.clone();
                        
                        loop {
                            match iter_b_cloned.next() {                        // Some(3) -> Some(2)
                                Some(is_cmp_a) => {                             // Some(3) -> Some(2)
                                    count += 1;                                 // count=1 -> count=2                      
                                    if cmp_a == is_cmp_a {                      // false   -> true
                                        already_compared_b = cmp_a;
                                        for i in 0 .. count - 1 {               //         -> for i in 0 .. 1
                                            iter_a_cloned.next();               //         -> Some(3)
                                        }
                                        match iter_a_cloned.next() {            //         -> Some(1)
                                            Some(is_cmp_b) => {                 //         -> Some(1)
                                                if is_cmp_b == cmp_b {          //         -> Some(1) == Some(1)
                                                    swapped = true;             //         -> swapped = true
                                                    already_compared_a = cmp_b;
                                                    break;                      //         -> break
                                                } else {
                                                    return false;
                                                }
                                            },
                                            None => return false,
                                        }
                                    } else {
                                        continue;                               // continue
                                    }
                                },
                                None => return false,
                            }
                        }
                    }
                }
            },
            (None, _) => break,                                                 //
            (_, None) => break,
        }
    }
    true
}
