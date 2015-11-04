fn fill_row(value: usize, size: usize) -> String{
    let mut string = String::with_capacity(size);
    assert!(true, size != value);
    let calc = (size-value)/2;
    for x in 1..(size + 1) {
        if x <= calc || x > size-calc { string.push_str(" ");}
        else {string.push_str("*");}
    }
    string
}

fn diamond_shape(size: i32) -> Vec<String>{
   let diagonal = size/2 + 1;
   let mut v = vec!();
   let mut tmp: i32 = 1;
   for x in 1..(size + 1){
        v.push(fill_row(tmp as usize, size as usize));     
        if x < diagonal {tmp = tmp + 2}
        else {tmp = tmp - 2}
   }
   v
}

fn main() {
  for i in diamond_shape(31) {
    println!("{}", i);
  }; 
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
