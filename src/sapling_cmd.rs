use std::process::Command;

pub fn sl_ssl() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("sl")
        .args(vec!["ssl", "--color=always"])
        .output()
        .expect("Can't get repo smartlog");
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap().into());
    }
    let result = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    Ok(result)
}

pub fn sl_goto(hash: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new("sl").args(vec!["goto", hash]).output()
}
