use std::time::Instant;
use std::thread;

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
fn idk2(row:i32,output:bool){ //the way i think
    if row == 0 {()}
    else if row==1 && output {println!("1");}
    else if row==2 && output {println!("1\n1 1");}
    else{
        if output{println!("1\n1 1\n1 2 1");}
        let mut number:Vec<u128> = Vec::with_capacity(row as usize);
        let mut new_number:Vec<u128> = Vec::with_capacity(row as usize);
        number.push(2);
        for i in 2..row-1{
            new_number.clear();
            new_number.push(number[0]+1);
            for j in 0..(number.len())/2{
                if number.len()!=1{
                new_number.push(number[j]+number[j+1]);
                }
            }
            let skip_count = if i%2 == 0 {0} else {1};
            for item in 0..new_number.len()-skip_count{
                new_number.push(new_number[item])
            }
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
fn idk3(row:i32,output:bool){  //the way i find on internet
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
    let run_time: u128 = 10000;
    let row: i32 = 12;
    let output: bool = true;

    let func1 = thread::spawn(move ||{
        let start = Instant::now();
        for _ in 0..run_time{
        idk(row,output);
        }
        Instant::now() - start
    });

    let func2 = thread::spawn(move ||{
        let start = Instant::now();
        for _ in 0..run_time{
        idk2(row,output);
        }
        Instant::now() - start
    });

    let func3 = thread::spawn(move ||{
        let start = Instant::now();
        for _ in 0..run_time{
        idk3(row,output);
        }
        Instant::now() - start
    });

    let end = func1.join().unwrap();
    let end2 = func2.join().unwrap();
    let end3 = func3.join().unwrap();

    println!("idk run {} times / {} = {:?}µs",run_time,run_time,end.as_micros() as f64/run_time as f64);
    println!("idk2 run {} times / {} = {:?}µs",run_time,run_time,end2.as_micros() as f64/run_time as f64);
    println!("idk3 run {} times / {} = {:?}µs",run_time,run_time,end3.as_micros() as f64/run_time as f64);
}