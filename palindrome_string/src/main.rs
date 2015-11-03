trait Palindrome{
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        let temp = self.clone();
        *self == temp.chars().rev().collect::<String>()
    }
        
}
fn main() {
}

#[test]
fn some_test() {
    let right = "level".to_string();
    assert_eq!(true, right.is_palindrome());
}

#[test]
fn another_test(){
    let wrong = "Sgkdjfjs".to_string();
    assert_eq!(false, wrong.is_palindrome());
    
}

