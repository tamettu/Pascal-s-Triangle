use std::io::{self,Write};
use std::time::Instant;
fn idk(row:i32){
    if row == 0 {()}
    else if row==1{println!("1");}
    else if row==2{println!("1\n1 1");}
    else{
        println!("1\n1 1\n1 2 1");
        
        let mut number:Vec<u128> = vec!();
        let mut new_number:Vec<u128> = vec!();
        number.push(2);
        for _ in 2..row-1{
            new_number.clear();
            new_number.push(number[0]+1);
            for j in 0..(number.len()-1){
                new_number.push(number[j]+number[j+1]);
            }
            new_number.push(number[0]+1);
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
fn combination(n: u128, r: u128) -> u128 {
    let mut result = 1_u128;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}
fn idk2(row:i32){
    if !row>0{();}
    else{
        for i in 0..row{
            for j in 0..i{
                print!("{} ",combination(i as u128, j as u128));
                io::stdout().flush().unwrap();
            }
            println!();
        }
    }
}
fn main() {
    let start = Instant::now();
    idk(100);
    let end = start.elapsed();
    
    let start = Instant::now();
    idk2(100);
    let end = start.elapsed();

    println!("idk {:?}",end);
    println!("idk2 {:?}",end);
}
