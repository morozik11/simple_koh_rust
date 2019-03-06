extern crate iron;
extern crate params;
 

use iron::prelude::*;

fn handle_user(req: &mut Request) -> IronResult<Response> {
    
    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    match map.find(&["name"]) {
        Some(&Value::String(ref name)) if name == "Marie" => {
            Ok(Response::with((iron::status::Ok, "Welcome back, Marie!")))
                                            
        },
        _ => Ok(Response::with(iron::status::NotFound)),
                            
    }

}

fn main() {
        Iron::new(handle_user).http("localhost:3000").unwrap();

}


