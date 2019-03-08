
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::status;
use iron::{Iron, Request, Response, IronResult};

use mount::Mount;
use router::Router;
use staticfile::Static;

use std::path::Path;

fn say_hello(req: &mut Request) -> IronResult<Response> {
    
    println!("Running send_hello handler, URL path: {}", req.url.path().join("/"));
    
    use iron::mime;
    let content_type = "application/json".parse::<mime::Mime>().unwrap();

    Ok(Response::with((content_type , status::Ok, "{}")))

}

fn main() {
       
    let mut router = Router::new();
    router.get("", say_hello, "");

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("src/assets")) );
    mount.mount("/upload", router);
    
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
    


}
