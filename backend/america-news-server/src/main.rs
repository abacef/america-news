#![feature(proc_macro_hygiene, decl_macro)]

mod Hello;

use std::path::{Path, PathBuf};

use rocket_contrib::serve::StaticFiles;

#[macro_use] extern crate rocket;

use serde::Serialize;

use Hello::Greeting;
use Hello::MyResponder;
use Hello:: Body;


#[get("/hello")]
fn hello() -> MyResponder {
    let greeting = Greeting {
        greeting: "¡Hola, mundo!".to_string()
    };

    MyResponder {
        body: Body {
            content: serde_json::to_string(&greeting).unwrap()
        }
    }
}

fn get_frontend_output_dir() -> String {
    let base_project_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"))).parent().unwrap().parent().unwrap().to_str().unwrap();
    println!("base_project_path: {:?}", base_project_path);

    let mut static_files_path = PathBuf::from(base_project_path);
    static_files_path.push("frontend");
    static_files_path.push("public");
    
    let static_files_path_str = static_files_path.to_str().unwrap();
    println!("static_files_path_str: {:?}", static_files_path_str);

    static_files_path_str.to_string()
}

fn main() {
    let frontend_path = get_frontend_output_dir();

    rocket::ignite()
        .mount("/", StaticFiles::from(frontend_path))
        .mount("/api", routes![hello])
        .launch();
}
