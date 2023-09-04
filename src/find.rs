use std::fs;
use std::path::Path;
use regex::Regex;
pub fn find<P: AsRef<Path>>(root: P,regexs: &Vec<Regex>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regexs,&mut matches)?;
    matches.sort();
    Ok(matches)

}

fn walk_tree(
    dir: &Path,
    regexs: &Vec<Regex>,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regexs, matches)?;
            }else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                for regex in regexs {
                    if regex.is_match(filename) || regex.as_str() == "--verbose" || regex.as_str() == "-v"{
                        matches.push(path.to_string_lossy().to_string());
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
