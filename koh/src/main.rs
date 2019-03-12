extern crate rustc_serialize;
extern crate koh; 

use rustc_serialize;
use rustc_serialize::json;


#[derive(rustc_serialize::RustcDecodable, rustc_serialize::RustcEncodable)]
pub struct ClassVector {
    vector: Vec<f32>,
    class: usize,
}

fn main() {
	
    let f = "/home/stas/rust/files/test.csv";
	
    let vs = koh::wrap_koh::create_vectors(&f);
	
    match vs {
        Ok(v) => {	
                
            let bool_ = koh::wrap_koh::check_leng_vectors(&v);
                    
            if bool_ == true {
                            
                /* training */
                let mut train_vectors = koh::koh::get_max_min_vectors(&v);
                let average = koh::koh::get_average_vectors(&train_vectors,&v);
                train_vectors.extend(average);
                let max_min = koh::koh::get_max_min(&train_vectors);
                let a_b = koh::koh::get_a_b(&max_min);
                let mut normalized_train = koh::koh::wrap_normalize(&train_vectors, &a_b);

                let mut new_vs = koh::koh::generate_rand_vs(normalized_train.len(),normalized_train[0].len());
                koh::koh::wrap_training(&mut normalized_train, &mut new_vs);
                /* training */
                
                /*classification*/
                let max_min_ = koh::koh::get_max_min(&v);
                let a_b_ = koh::koh::get_a_b(&max_min_);
                let mut normalized = koh::koh::wrap_normalize(&v, &a_b_);
                let result = koh::koh::classification(&mut new_vs, &mut normalized);
                let encoded = json::encode(&result).unwrap();
                println!("{}",encoded);
                /*classification*/
                    
            } else {
                 println!("error");            
            }
                    
        },
        Err(e) => {
            println!("{:?}",e);        
        }
    };
    
}


