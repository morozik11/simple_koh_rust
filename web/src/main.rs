extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate params;

use iron::status;
use iron::{Iron, Request, Response, IronResult};
use iron::prelude::*;

use mount::Mount;
use router::Router;
use staticfile::Static;
use params::{Params, Value};
use iron::method;
use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn upload(req: &mut Request) -> IronResult<Response> {

    use iron::mime;
    
    let mut bool_ = true;

    let content_type = "application/json".parse::<mime::Mime>().unwrap();
    
    match req.get_ref::<Params>().unwrap().find(&["file"]) {
        
        Some(&Value::File(ref file)) => {   
            
            let ext = &file.content_type.1;
            let s = ext.as_str();
            
            match s {    
                "csv" =>  {

                    let ref_path = PathBuf::from(&file.path);
                    let os_str = ref_path.into_os_string(); 
                    let mut data = String::new();
                    let mut f = File::open(os_str).expect("Unable to open file");
                    f.read_to_string(&mut data).expect("Unable to read string");
                    println!("{:?}",data);

                },
                _ =>    { bool_ = false; }
            }
        },
        _ =>  { bool_ = false; } 
       
    }
    
    if bool_ == true{
        Ok(Response::with((content_type , status::Ok, "{\"status\":\"ok\"}")))
    } else {
        Ok(Response::with((content_type , status::Ok, "{\"status\":\"error file\"}")))
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
