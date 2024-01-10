use sl_up::{
    sapling_cmd::{get_smartlog, goto_commit},
    smartlog::SmartLog,
    ui::start_ui_and_get_selected_commit,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_smartlog = get_smartlog().unwrap();
    let mut smartlog = SmartLog::new(&raw_smartlog);

    let commit_hash = start_ui_and_get_selected_commit(&mut smartlog);

    if let Some(commit_hash) = commit_hash {
        let output = goto_commit(commit_hash).unwrap();
        println!("{}", String::from_utf8(output.stdout).unwrap());
    }

    Ok(())
}
