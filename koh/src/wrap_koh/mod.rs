use std::{fs, io, path::Path, error::Error};
use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn check_leng_vectors(vectors: &Vec<Vec<f32>>) -> bool{
	
    let mut result = true;
	
    match vectors.len() {
        0 => {
            return false;
        },
        1 => {
            return false;
        },
        _ => {
                    
        }
    }
    
    let leng = vectors[0].len();
		
    for v in vectors {
		
        match v.len() == leng {
            false => {
               return false;
            },
            true => {
                            
            }
        }
		
    }
	
    result
	
}

pub fn create_vectors(path: &str) -> Result<Vec<Vec<f32>>, Box<Error>>{
    
    let path = Path::new(path);
    
    if !path.is_file(){
        return Err(Box::from("Файл не найден"));
    }
    
    let file = File::open(path);
    
    let mut vectors = vec![];
    let mut vector = vec![];
    
    match file {
        Ok(f) => {
            let buff = BufReader::new(f);
            for line in buff.lines() {
                match line {
                    Ok(l) => {
                        let s = l.trim().to_string();
                        if s != "" {
                            
                            let words: Vec<&str> = s.split(",").collect();
                            for w in words {
                                match w.parse::<f32>() {
                                    Ok(n) => {
                                        vector.push(n);
                                    },
                                    Err(e) => {
                                        return Err(Box::from("Ошибка парсинга файла"));
                                    }
                                };
                            }
                            vectors.push(vector);
                            vector = vec![];
                            
                        }            
                    },
                    Err(e) => {
                        return Err(Box::from("Ошибка чтения файла"));
                    },
                };
            }
        },
        Err(e) => {
            return Err(Box::from("Файл не найден"));
        },
    };
    
    Ok(vectors)
    
}
