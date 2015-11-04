fn fill_row(value: usize, size: usize) -> String{
    let mut string = String::new();
    assert!(true, size != value);
    let calc = (size-value)/2;
    for x in 1..(size + 1) {
        if x <= calc || x > size-calc { string.push_str(" ");}
        else {string.push_str("*");}
    }
    string
}

fn diamond_shape(size: i32) -> Vec<String>{
   let diagonal = size%2 + 1;
   let mut v = vec!();
   let mut row: Vec<String> = Vec::with_capacity(size as usize);
   let mut tmp: i32 = 1;
   for x in 0..size {
        if x <= diagonal + 1 {tmp = x*2}
        else {tmp = size-x/2 }
        v.push(fill_row(tmp as usize, size as usize));     
   }
   v
}

fn main() {
  for i in diamond_shape(9) {
    println!("{}", i);
  } 
}

#[test]
fn test_fill_row(){
    assert_eq!(" ***** ".to_string(), fill_row(5,7));
}
#[test]
fn test_output(){
    assert_eq!(vec!("  *  ", " *** ", "*****", " *** ", "  *  "),
               diamond_shape(5));

}
