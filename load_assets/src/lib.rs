use std::{fs::read_dir, path::Path};

use proc_macro::TokenStream;

/// Generates an expression with collects all asset paths in a vector and returns it.
#[proc_macro]
pub fn load_assets(_item: TokenStream) -> TokenStream {
    let paths = load_asset_paths();
    let mut expression = "{let mut paths = Vec::new();".to_string();

    for path in paths {
        expression += format!("paths.push(\"{path}\");").as_str();
    }

    expression += "paths}";

    expression.parse().unwrap()
}

fn load_asset_paths() -> Vec<String> {
    load_asset_paths_recursive(Path::new("./assets")).expect("the assets folder should exist")
}

fn load_asset_paths_recursive(path: &Path) -> std::io::Result<Vec<String>> {
    let mut files = vec![];

    if path.is_dir() {
        for entry in read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(load_asset_paths_recursive(&path)?);
            } else {
                let path_str = path
                    .to_str()
                    .unwrap()
                    .replace('\\', "/")
                    .replace("./assets/", "")
                    .to_string();
                files.push(path_str);
            }
        }
    }

    Ok(files)
}
