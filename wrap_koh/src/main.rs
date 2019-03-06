use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;


fn main() {
    
    let f = "/home/stas/rust/wrap_koh/src/test.csv";
    
    let mut vectors = create_vectors(&f);
    
    let bool_ = check_leng_vectors(&vectors);
    if bool_ == false {
        
        process::exit(1);
        
    }
    
    println!("{:?}",vectors);
    
}

fn check_leng_vectors(vectors: &Vec<Vec<f32>>) -> bool{
    
    let leng = vectors[0].len();
    let mut result = true;
    
    for v in vectors {
        
        if v.len() < leng {
            
            result = false;
            break;
            
        }
        
    }
    
    result
    
}

fn create_vectors(path: &str) -> Vec<Vec<f32>>{
    
    let f = File::open(path).expect("Unable to open file");
    let f = BufReader::new(f);
    
    let mut vectors = vec![];
    let mut vector = vec![];
    
    for line in f.lines() {
        
        let line = line.expect("Unable to read line");
        let words: Vec<&str> = line.split(",").collect();
        
        for w in words {
            
            let n:f32 = w.parse().expect("Not a number!");
            vector.push(n);
            
        }
        
        vectors.push(vector);
        vector = vec![];
        
    }
    
    vectors
    
}
