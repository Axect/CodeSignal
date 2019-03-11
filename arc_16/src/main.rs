fn main() {
    println!("{}", areSimilar(vec![1,3,2,2,4], vec![1,3,2,4,2]));
}

fn areSimilar(a: Vec<i32>, b: Vec<i32>) -> bool {
    are_similar_stack(a, b, 0)
}

fn are_similar_stack(a: Vec<i32>, b: Vec<i32>, stack: usize) -> bool {
    if stack >= 2 {
        return false;
    }
    
    let mut a_iter = a.clone().into_iter();
    let mut b_iter = b.clone().into_iter();
    
    match (a_iter.next(), b_iter.next()) {
        (None, _) => true,
        (_, None) => true,
        (Some(cmp_a), Some(cmp_b)) if cmp_a == cmp_b => are_similar_stack(a_iter.collect::<Vec<i32>>(), b_iter.collect::<Vec<i32>>(), stack),
        (Some(cmp_a), Some(cmp_b)) => {
            let mut a_iter_cloned = a_iter.clone();
            let mut b_iter_cloned = b_iter.clone();
            let mut count = 0usize;
            
            loop {
                match (a_iter_cloned.next(), b_iter_cloned.next()) {
                    (None, _) => return false,
                    (_, None) => return false,
                    (Some(x), Some(y)) if x == cmp_b && y == cmp_a => {
                        let mut a_left = a_iter.collect::<Vec<i32>>();
                        let mut b_left = b_iter.collect::<Vec<i32>>();
                        a_left.remove(count);
                        b_left.remove(count);
                        
                        return are_similar_stack(a_left, b_left, stack + 1);
                    },
                    (Some(x), Some(y)) => {
                        count += 1;
                    }
                }
            }
        }
    }
}
