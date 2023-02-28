pub mod utils {
  use std::path::PathBuf;

  pub fn to_first_upper_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
  }

  pub fn join_strings_to_pathbuf(strings: &[&str]) -> PathBuf {
      let mut path = PathBuf::new();
      for s in strings {
          path.push(s);
      }
      path
  }

  pub fn generate_path_string(root_folder: &str, dir: &str, name: &str) -> String {
      let path = join_strings_to_pathbuf(&[root_folder, dir, name]).display().to_string();

      path
  }
}