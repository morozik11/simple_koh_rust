extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate params;
extern crate rustc_serialize;

use iron::{Iron, Request, Response, IronResult, status, prelude::*, method};

use mount::Mount;
use router::Router;
use staticfile::Static;
use params::{Params, Value};

use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};

use std::fs::File;
use std::io::{BufReader, prelude::*};

use rustc_serialize::json;

fn upload(req: &mut Request) -> IronResult<Response> {

    use iron::mime;
    
    let mut bool_ = true;
    let mut f:File;
    let mut os_str = OsString::new();

    let content_type = "application/json".parse::<mime::Mime>().unwrap();
    
    match req.get_ref::<Params>().unwrap().find(&["file"]) {
        
        Some(&Value::File(ref file)) => {   
            
            let ext = &file.content_type.1;
            let s = ext.as_str();
            
            match s {    
                "csv" =>  {
                    let ref_path = PathBuf::from(&file.path);
                    os_str = ref_path.into_os_string(); 
                },
                _ =>    { bool_ = false; }
            }
        },
        _ =>  { bool_ = false; } 
       
    }
    
    if bool_ == true{
        
        let vs = web::wrap_koh::create_vectors(&os_str);
        
        match vs {
            Ok(v) => {
                
                let bool_ = web::wrap_koh::check_leng_vectors(&v);
                    
                if bool_ == true {
                            
                    /* training */
                    let mut train_vectors = web::koh::get_max_min_vectors(&v);
                    let average = web::koh::get_average_vectors(&train_vectors,&v);
                    train_vectors.extend(average);
                    let max_min = web::koh::get_max_min(&train_vectors);
                    let a_b = web::koh::get_a_b(&max_min);
                    let mut normalized_train = web::koh::wrap_normalize(&train_vectors, &a_b);

                    let mut new_vs = web::koh::generate_rand_vs(normalized_train.len(),normalized_train[0].len());
                    web::koh::wrap_training(&mut normalized_train, &mut new_vs);
                    /* training */
                
                    /*classification*/
                    let max_min_ = web::koh::get_max_min(&v);
                    let a_b_ = web::koh::get_a_b(&max_min_);
                    let mut normalized = web::koh::wrap_normalize(&v, &a_b_);
                    let mut result = web::koh::classification(&mut new_vs, &mut normalized);
                    web::koh::wrap_un_normalize(&v,&mut result);
                    /*classification*/
                    
                    let encoded = json::encode(&result);
                    
                    match encoded {
                        Ok(en) => {
                            return Ok(Response::with((content_type , status::Ok, en)));
                        },
                        Err(e) => {
                            return Ok(Response::with((content_type , status::Ok, "{\"status\":\"error encode to json\"}"))); 
                        }
                    };
                    
                } else {
                     return Ok(Response::with((content_type , status::Ok, "{\"status\":\"error parsing file\"}")));          
                }
                    
            },
            Err(e) => {
                return Ok(Response::with((content_type , status::Ok, "{\"status\":\"file not fount\"}")));       
            }
        };
        
        
    } else {
        Ok(Response::with((content_type , status::Ok, "{\"status\":\"file expansion not csv\"}")))
    }

}

fn main() {
       
    let mut router = Router::new();
    router.post("", upload, "");

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("src/assets")) );
    mount.mount("/upload", router);
    
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
    
}
