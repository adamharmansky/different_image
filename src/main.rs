use iron::prelude::*;
use iron::status::Status;
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::io::Read;

fn handler(req: &mut Request, pictures: &Vec<String>) -> IronResult<Response> {
    let mut hasher = DefaultHasher::new();
    let ip = req.headers.get_raw("X-Real-IP").unwrap();
    ip.hash(&mut hasher);
    let hash = hasher.finish() as usize % pictures.len();
    let file = fs::File::open(format!("pictures/{}", pictures[hash])).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    Ok(Response::with((
        "image/png".parse::<iron::mime::Mime>().unwrap(),
        Status::Ok,
        buf.as_slice(),
    )))
}

fn main() {
    let mut pictures = Vec::new();

    let re = Regex::new(r"\.png").unwrap();
    let dir = fs::read_dir("pictures").unwrap();

    for path in dir {
        let path = path.unwrap().file_name();
        let path = path.to_str().unwrap();
        if re.is_match(path) {
            pictures.push(String::from(path));
        }
    }

    Iron::new(move |x: &mut Request| handler(x, &pictures))
        .http("127.0.0.1:8888")
        .unwrap();
}
