use std::fs;
use std::path::Path;

fn main() {
    if Path::new("__template__.cpp").exists() == false {
        println!("Doesn't found __template__.cpp!");
        return;
    }

    let files = fs::read_dir(".").unwrap();
    let source: String = String::from("__template__.cpp");

    let mut count_success = 0;
    let mut count_fail = 0;

    for file in files {
        let file_source = &source;
        let file_dir: String = file.unwrap().path().display().to_string();
        let file_name = &file_dir.clone()[file_dir.rfind(if cfg!(windows) {'\\'} else {'/'}).unwrap() + 1..];
        let file_dot = file_name.find('.');

        // Avoid folder or executable file on linux
        if file_dot == None {
            continue;
        }
        let file_dot_location = file_dot.unwrap();

        // Avoid reset template file and files that are not c++ source code
        let prefix = &file_name[..file_dot_location];
        let suffix = &file_name[file_dot_location + 1..];

        if prefix == "__template__" || suffix != "cpp" {
            continue;
        }

        match fs::copy(file_source, file_dir) {
            Ok(_) => {
                println!("Reseted {file_name}");
                count_success += 1;
            }
            Err(e) => {
                println!("Failed to reset {file_name} with error {e:?}");
                count_fail += 1;
            }
        }
    }

    println!("Successfully reseted {count_success} file{}", if count_success > 1 {"s"} else {""});
    if count_fail > 0 {
        println!("found {count_fail} error{}", if count_fail > 1 {"s"} else {""});
    }
}
