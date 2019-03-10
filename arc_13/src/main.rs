pub fn main() {
    let mut a = (1 .. 1000000).map(|x| x.to_string()).collect::<Vec<String>>();
    a.push("(123)321(456)(789)(123)(321)321(123)(321)".to_string());
    let b = a.join("");

    println!("{}", reverseInParentheses(b));
}

pub fn reverseInParentheses(inputString: String) -> String {
    let mut result = String::new();
    
    let mut iter_str = inputString.chars();
    
    loop {
        match iter_str.next() {
            Some(target) => {
                if target != '(' {
                    result.push(target);
                } else {
                    let (body, tail) = iter_str.collect::<String>().take_skip_until_closed();
                    result.push_str(&reverseInParentheses(body).chars().rev().collect::<String>());
                    result.push_str(&reverseInParentheses(tail));
                    break;
                }
            },
            None => break,
        }
    }
    result
}

pub trait TakeSkipUntilClosed {
    fn take_skip_until_closed(&self) -> (String, String);
}

impl TakeSkipUntilClosed for String {
    fn take_skip_until_closed(&self) -> (String, String) {
        let mut take = String::new();
        let mut skip = String::new();
        
        let mut iter_str = self.chars();
        
        let mut open_stack = 1usize;
        let mut clos_stack = 0usize;
        
        loop {
            match iter_str.next() {
                Some(target) => {
                    if target != '(' && target != ')' {
                        take.push(target);
                        continue
                    } else if target == '(' {
                        open_stack += 1;
                    } else {
                        clos_stack += 1;
                        if open_stack == clos_stack {
                            skip.push_str(&iter_str.collect::<String>());
                            break;
                        }
                    }
                    take.push(target);
                    println!("take: {}", take);
                },
                None => {
                    break;
                }
            }
        }
        (take, skip)
    }
}

