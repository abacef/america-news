// #[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};

use rocket_contrib::serve::StaticFiles;

// #[get("/hello")]
// fn index() -> &'static str {
//     "Hello, world!"
// }


fn main() {
    let base_project_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"))).parent().unwrap().parent().unwrap().to_str().unwrap();
    println!("base_project_path: {:?}", base_project_path);

    let mut static_files_path = PathBuf::from(base_project_path);
    static_files_path.push("frontend");
    static_files_path.push("public");
    
    let static_files_path_str = static_files_path.to_str().unwrap();
    println!("static_files_path_str: {:?}", static_files_path_str);

    rocket::ignite()
        .mount("/", StaticFiles::from(static_files_path.to_str().unwrap()))
        .launch();
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../frontend/public")))
// }