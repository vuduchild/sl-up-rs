use std::process::Command;

pub fn get_smartlog() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("sl")
        .args(vec!["ssl", "--color=always"])
        .output()
        .expect("Can't get repo smartlog");
    let result = String::from_utf8(output.stdout)
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    Ok(result)
}

pub fn goto_commit(hash: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new("sl").args(vec!["goto", hash]).output()
}
