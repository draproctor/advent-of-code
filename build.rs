use std::collections::HashMap;
use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

/// Build the file that allows us to dynamically select a solution function.
/// We always rerun this because compilation of this file should be cheap.
fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("module_selection.rs");

    let lookup_insert_statements = year_modules_to_day_modules()
        .into_iter()
        .flat_map(|(year_module, day_modules)| {
            day_modules.into_iter().map(move |day| {
                let key = format!("\"{year_module}::{day}\".to_string()");
                let value = format!("Box::new({year_module}::{day}::solve)");
                format!("    selector.insert({key}, {value});")
            })
        })
        .collect::<Vec<String>>()
        .join("\n");

    let mut line_writer = LineWriter::new(File::create(dest_path).unwrap());
    let lines = vec![
        "pub fn solution_selector() -> HashMap<String, Box<dyn Fn(PathBuf)>> {\n",
        "    let mut selector: HashMap<String, Box<dyn Fn(PathBuf)>> = HashMap::new();\n",
        &lookup_insert_statements,
        "\n",
        "    selector\n",
        "}",
    ];
    for line in &lines {
        line_writer.write_all(line.as_bytes()).unwrap();
    }
}

fn year_modules_to_day_modules() -> HashMap<String, Vec<String>> {
    read_dir("src")
        .unwrap()
        .into_iter()
        .filter_map(some_if_year_module)
        .map(|(year_module, year_module_path)| {
            (year_module, day_modules_for_year(year_module_path))
        })
        .collect()
}

fn some_if_year_module(entry: Result<DirEntry, std::io::Error>) -> Option<(String, PathBuf)> {
    entry.map_or(None, |entry| {
        is_year_module(&entry).then(|| (entry_file_name(&entry), entry.path()))
    })
}

fn entry_file_name(entry: &DirEntry) -> String {
    entry.file_name().to_str().unwrap().to_owned()
}

fn is_year_module(entry: &DirEntry) -> bool {
    entry.path().is_dir() && entry.file_name().to_str().unwrap().starts_with("year")
}

fn day_modules_for_year(path: PathBuf) -> Vec<String> {
    read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|entry| entry.map_or(None, some_if_day_module))
        .collect()
}

fn some_if_day_module(entry: DirEntry) -> Option<String> {
    let file_name = entry.file_name().to_str().unwrap().to_owned();
    file_name
        .starts_with("day")
        .then(|| file_name.trim_end_matches(".rs").to_owned())
}
