use std::env;
use std::fs;
use std::path::Path;

// The contents of the index.ts file in a module subfolder
const INDEX_CONTENTS: &str = "export default {};";

// The names of the folders to create for a new module
const MODULE_FOLDERS: &[&str] = &["api", "components", "hooks", "layouts", "pages"];

fn to_first_upper_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags: Vec<String> = args.iter().filter(|arg| arg.starts_with("-")).map(|arg| arg.to_string()).collect();

    println!("Flags: {:?}", flags);

    if args.len() < 3 {
        println!("Usage: rg <type> <name>");
        return;
    }

    let operation = &args[1];
    let name = &args[2];

    match operation.as_str() {
        "component" => generate_component(name),
        "layout" => generate_layout(name),
        "module" => generate_module(name),
        _ => println!("Unknown operation: {}", operation),
    }
}

// Generate a new React component
fn generate_component(name: &str) {
    let folder_name = format!("src/components/{}", name);
    let file_path = format!("{}/index.tsx", folder_name);

    let name_first_upper_letter = to_first_upper_letter(name);

    fs::create_dir_all(&folder_name).expect("Failed to create folder for component");

    let contents = format!(
        "import React from 'react';

interface {}Props {{
    // Add props here
}}

const {} = ({{}}: {}Props) => {{
    return <div>{}</div>;
}};

export default {};",
      name_first_upper_letter, name_first_upper_letter, name_first_upper_letter, name_first_upper_letter, name_first_upper_letter
    );

    fs::write(&file_path, contents).expect("Failed to create component file");
}

// Generate a new React layout
fn generate_layout(name: &str) {
    let folder_name = format!("src/layouts/{}", name);
    let file_path = format!("{}/index.tsx", folder_name);

    let name_first_upper_letter = to_first_upper_letter(name);

    fs::create_dir_all(&folder_name).expect("Failed to create folder for layout");

    let contents = format!(
        "import React from 'react';

interface {}Props {{
    // Add props here
}}

const {} = ({{ ...props }}: {}Props) => {{
    return (
        <div>
            {} Layout
        </div>
    );
}};

export default {};",
        name_first_upper_letter, name_first_upper_letter, name_first_upper_letter, name_first_upper_letter, name_first_upper_letter
    );

    fs::write(&file_path, contents).expect("Failed to create layout file");
}

// Generate a new React module
fn generate_module(name: &str) {
    let folder_name = format!("src/modules/{}", name);
    let index_path = format!("{}/index.ts", folder_name);

    fs::create_dir_all(&folder_name).expect("Failed to create folder for module");

    let mut pages = String::new();
    let mut api = String::new();

    for folder_name in MODULE_FOLDERS.iter() {
        let path = Path::new(&folder_name);
        let full_path = format!("{}/{}", folder_name, name);

        let folder_path = Path::new(&full_path);
        let folder_full_path = format!("src/modules/{}/{}", name, folder_name);

        let index_path = format!("{}/index.ts", folder_full_path);

        match path.file_name().unwrap().to_str().unwrap() {
            "pages" => pages = format!("export {{}};\n"),
            "api" => api = format!("export {{}};\n"),
            _ => (),
        }

        fs::create_dir_all(&folder_full_path).expect("Failed to create folder for module subfolder");

        fs::write(&index_path, INDEX_CONTENTS)
        .expect("Failed to create index.ts file for module subfolder");
}

let contents = format!(
    "import * as api from './api';
import * as pages from './pages';

const {}Module = {{
  pages: {{ ...pages }},
  api: {{ ...api }},
}}

export default {}Module;\n",
    name,
    name
    );
    fs::write(&index_path, contents).expect("Failed to create index.ts file for module");
}
