use std::error::Error;

use sl_up::{
    sapling_cmd::{sl_goto, sl_ssl},
    smartlog::SmartLog,
    ui::start_ui_and_get_selected_commit,
};

fn main() -> Result<(), Box<dyn Error>> {
    let raw_smartlog_result = sl_ssl();
    if raw_smartlog_result.is_err() {
        print!("{}", raw_smartlog_result.unwrap_err());
        std::process::exit(1);
    }

    let mut smartlog = SmartLog::new(&raw_smartlog_result.unwrap());

    let commit_hash = start_ui_and_get_selected_commit(&mut smartlog);

    if let Some(commit_hash) = commit_hash {
        let output = sl_goto(commit_hash).unwrap();
        print!("{}", String::from_utf8(output.stdout).unwrap());
    }

    Ok(())
}
