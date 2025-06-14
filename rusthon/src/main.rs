use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
fn main() {
    // Create output file
    let mut file = match File::create("output.rs") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create a file: {}", e);
            std::process::exit(1);
        }
    };
    let mut tabs = 0;
    let start_content = "fn main() {\n";
    if let Err(e) = file.write_all(start_content.as_bytes()) {
        eprintln!("Failed to write into a file: {}", e);
    } else {
        println!("Start code written to a file");
    }
    tabs = tabs + 1;

    // Parse command line args
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <dir_path> <filename>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    let file_name = &args[2];
    let file_path = Path::new(dir_path).join(file_name);

    // Regex to extract content inside ("...")
    let re = Regex::new(r#"\("([^"]*)"\)"#).unwrap();
    let re_var = Regex::new(r#"\b([a-zA-Z_]\w*)\s*=\s*([^\n#]+)"#).unwrap();
    // Read and process lines
    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            println!("File contents:\n");

            for (i, line) in contents.lines().enumerate() {
                let first_word = line.split_whitespace().next();

                if let Some(word) = first_word {
                    if word == "print" || word.starts_with("print(") {
                        println!("{}. {}  <-- starts with 'print'", i + 1, line);

                        if let Some(caps) = re.captures(line) {
                            let inside = &caps[1];
                            println!("     └─ inside (\"...\"): {}", inside);
                            let mut spaces = "".to_string();
                            for _i in 0..(tabs * 4) {
                                spaces.push_str(" ");
                            }
                            let content = format!("{}println!(\"{}\");\n", spaces, inside);
                            if let Err(e) = file.write_all(content.as_bytes()) {
                                eprintln!("Failed to wirtie into a file: {}", e);
                            } else {
                                println!("Println written to a file");
                            }
                        }
                    } else if let Some(caps) = re_var.captures(line) {
                        let var_name = &caps[1];
                        let var_val = &caps[2];
                        let mut spaces = "".to_string();
                        for _i in 0..(tabs * 4) {
                            spaces.push_str(" ");
                        }
                        let content = format!("{}let {} = {};\n", spaces, var_name, var_val);
                        if let Err(e) = file.write_all(content.as_bytes()) {
                            eprintln!("Failed to write into a file: {}", e);
                        } else {
                            println!("Var added to a file");
                        }
                    } else {
                        println!("{}. {}", i + 1, line);
                    }
                } else {
                    println!("{}. (empty line)", i + 1);
                }
            }
        }
        Err(e) => eprintln!("Failed to read file '{}': {}", file_path.display(), e),
    }
    let content_end = "\n}";
    if let Err(e) = file.write_all(content_end.as_bytes()) {
        eprintln!("Error writing to a file: {}", e);
    } else {
        println!("Written to a file");
    }

    let output = Command::new("rustc")
        .arg("output.rs")
        .output()
        .expect("Failed to execute compilation");
    if output.status.success() {
        println!("Compiled succesfully");
    } else {
        eprintln!("Compilation error");
        eprintln!("{:?}", String::from_utf8(output.stderr));
    }
}
