use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

const CONFIG_FILE: &str = ".rgrc.toml";

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

fn join_strings_to_pathbuf(strings: &[&str]) -> PathBuf {
    let mut path = PathBuf::new();
    for s in strings {
        path.push(s);
    }
    path
}

fn generate_path_string(root_folder: &str, dir: &str, name: &str) -> String {
    let path = join_strings_to_pathbuf(&[root_folder, dir, name]).display().to_string();

    path
}

const DEFAULT_CONFIG_CONTENTS: &str = r#"
root_folder = "src"
typescript = true
"#;


fn get_config_value(config: &Value, key: &str) -> String {
    let value = config[key].as_str().unwrap();

    value.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags: Vec<String> = args.iter().filter(|arg| arg.starts_with("-")).map(|arg| arg.to_string()).collect();

    let config_exists: bool = Path::new(CONFIG_FILE).exists();

    let config_contents: String;

    if config_exists {
        // Read a config file if it exists
        config_contents = fs::read_to_string(CONFIG_FILE).unwrap();
    } else {
        // Otherwise, use the default config
        config_contents = DEFAULT_CONFIG_CONTENTS.to_string();
    }

    // Parse the config file
    let config: Value = toml::from_str(&config_contents).expect("Unable to parse config file");

    let mut root_folder: String = get_config_value(&config, "root_folder");
    if root_folder == "/" || root_folder == "\\" {
        root_folder = ".".to_string();
    }

    println!("{}", root_folder);

    println!("Flags: {:?}", flags);

    if args.len() < 3 {
        println!("Usage: rg <operation> <name>");
        return;
    }

    let operation = &args[1];
    let name = &args[2];

    match operation.as_str() {
        "component" => generate_component(name, &root_folder),
        "layout" => generate_layout(name, &root_folder),
        "module" => generate_module(name, &root_folder),
        _ => println!("Unknown operation: {}", operation),
    }
}

// Generate a new React component
fn generate_component(name: &str, root_folder: &str) {
    let path = generate_path_string(root_folder, "components", name);
    let file_path = format!("{}/index.tsx", path);

    let name_first_upper_letter = to_first_upper_letter(name);

    fs::create_dir_all(&path).expect("Failed to create folder for component");

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
fn generate_layout(name: &str, root_folder: &str) {
    let path = generate_path_string(root_folder, "layouts", name);
    let file_path = format!("{}/index.tsx", path);

    let name_first_upper_letter = to_first_upper_letter(name);

    fs::create_dir_all(&path).expect("Failed to create folder for layout");

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
fn generate_module(name: &str, root_folder: &str) {
    let path = generate_path_string(root_folder, "modules", name);
    let index_path = format!("{}/index.ts", path);

    fs::create_dir_all(&path).expect("Failed to create folder for module");

    let mut pages = String::new();
    let mut api = String::new();

    for folder_name in MODULE_FOLDERS.iter() {
        let path = Path::new(&folder_name);
        let full_path = format!("{}/{}", folder_name, name);

        let folder_path = Path::new(&full_path);
        let folder_full_path = join_strings_to_pathbuf(&[root_folder, "modules", name, folder_name]).display().to_string();

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
