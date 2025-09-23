use std::time::Instant;
fn idk(row:i32,output:bool){ //the way i think
    if row == 0 {()}
    else if row==1 && output {println!("1");}
    else if row==2 && output {println!("1\n1 1");}
    else{
        if output{println!("1\n1 1\n1 2 1");}
        let mut number:Vec<u128> = Vec::with_capacity(row as usize);
        let mut new_number:Vec<u128> = Vec::with_capacity(row as usize);
        number.push(2);
        for _ in 2..row-1{
            new_number.clear();
            new_number.push(number[0]+1);
            for j in 0..(number.len()-1){
                new_number.push(number[j]+number[j+1]);
            }
            new_number.push(number[0]+1);
            number.clear();
            let mut a_line:String = "1 ".to_string();
            for item in &new_number{
                number.push(*item);
                a_line.push_str(&format!("{} ",item));
            }
            a_line.push_str("1");
            if output{println!("{}",a_line);}
        }
    }
}
fn combination(n: u128, r: u128) -> u128 {  //ask ai
    let mut result = 1_u128;
    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}
fn idk2(row:i32,output:bool){  //the way i find on internet
    if !row>0{();}
    else{
        for i in 0..row{
            let mut a_line:String = String::new();
            for j in 0..=i{
                a_line.push_str(&format!("{} ",combination(i as u128, j as u128)));
            }
            if output{println!("{}",a_line);}
        }
    }
}
fn main() {
    let row=10;
    let start = Instant::now();
    idk(row,true);
    let end = start.elapsed();
    
    let start2 = Instant::now();
    idk2(row,true);
    let end2 = start2.elapsed();

    println!("idk {:?}",end);
    println!("idk2 {:?}",end2);
}
