use std::io::{self,Write};
fn idk(row:i32){
    if row==1{println!("1");}
    else if row==2{println!("1\n1 1");}
    else{
        println!("1\n1 1\n1 2 1");
        
        let mut number:Vec<i32> = vec!();
        let mut new_number:Vec<i32> = vec!();
        number.push(2);
        for i in 2..row-1{
            new_number.clear();
            new_number.push((number[0]+1));
            for j in 0..(number.len()-1){
                new_number.push((number[j]+number[j+1]))
            }
            new_number.push((number[0]+1));
            number.clear();
            print!("1 ");
            io::stdout().flush().unwrap();
            for item in &new_number{
                number.push(*item);
                print!("{} ",item);
                io::stdout().flush().unwrap();
            }
            println!("1");
        }
    }
}
fn main() {
    idk(10);

}
