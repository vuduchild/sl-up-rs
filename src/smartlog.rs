use ansi_parser::{AnsiSequence, Output};

use crate::{parser::SmartLogParser, sapling_cmd::goto_commit};

const SELECTION_COLOR_CODE: u8 = 35;

#[derive(Debug)]
pub struct SmartLog<'a> {
    pub selectable_commits: Vec<SelectableCommit<'a>>,
    selected_commit_idx: usize,
}

impl<'a> SmartLog<'a> {
    pub fn new(raw_lines: &'a Vec<String>) -> Self {
        let selectable_commits = SmartLogParser::parse(raw_lines).unwrap();
        let selected_commit_idx = Self::get_selected_commit_index(&selectable_commits).unwrap();
        Self {
            selectable_commits,
            selected_commit_idx,
        }
    }
    pub fn checkout_selected_commit(&self) -> Result<std::process::Output, std::io::Error> {
        let hash = self.get_selected_commit_hash().unwrap();
        goto_commit(hash)
    }
    pub fn get_selected_commit_hash(&self) -> Option<&str> {
        let commit = self
            .selectable_commits
            .get(self.selected_commit_idx)
            .unwrap();
        Some(commit.get_hash().unwrap())
    }
    pub fn move_up(&mut self) {
        if self.selected_commit_idx > 0 {
            let mut selection_candidate = self.selected_commit_idx;
            for i in (0..self.selected_commit_idx).rev() {
                if self.selectable_commits[i].selectable {
                    selection_candidate = i;
                    break;
                }
            }
            if selection_candidate == self.selected_commit_idx {
                return;
            }

            self.deselect_line_idx(self.selected_commit_idx);
            self.select_line_index(selection_candidate);
        }
    }
    pub fn move_down(&mut self) {
        if self.selected_commit_idx < self.selectable_commits.len() - 1 {
            let mut selection_candidate = self.selected_commit_idx;
            for i in (self.selected_commit_idx + 1)..self.selectable_commits.len() {
                if self.selectable_commits[i].selectable {
                    selection_candidate = i;
                    break;
                }
            }
            if selection_candidate == self.selected_commit_idx {
                return;
            }

            self.deselect_line_idx(self.selected_commit_idx);
            self.select_line_index(selection_candidate);
        }
    }
    pub fn to_string_vec(&self) -> Vec<String> {
        self.selectable_commits
            .iter()
            .map(|commit| commit.to_string_vec())
            .flatten()
            .collect()
    }
    pub fn select_line_index(&mut self, line_number: usize) {
        let commit = self.selectable_commits.get_mut(line_number).unwrap();
        commit.select();
        self.selected_commit_idx = line_number;
        println!("selected_commit_idx: {}", self.selected_commit_idx);
    }
    pub fn deselect_line_idx(&mut self, line_idx: usize) {
        let commit = self.selectable_commits.get_mut(line_idx).unwrap();
        commit.deselect();
        println!("deselected_commit_idx: {}", line_idx);
    }
    fn get_selected_commit_index(selectable_commits: &Vec<SelectableCommit>) -> Option<usize> {
        for (idx, commit) in selectable_commits.iter().enumerate() {
            if commit.selected {
                return Some(idx);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct SelectableCommit<'a> {
    pub parsed_lines: Vec<Vec<Output<'a>>>,
    pub selectable: bool,
    pub selected: bool,
}

impl<'a> SelectableCommit<'a> {
    pub fn new(parsed_lines: Vec<Vec<Output<'a>>>, selectable: bool, selected: bool) -> Self {
        Self {
            parsed_lines,
            selectable,
            selected,
        }
    }
    pub fn to_string_vec(&self) -> Vec<String> {
        self.parsed_lines
            .iter()
            .map(|line| SmartLogParser::parsed_line_to_string(line))
            .collect::<Vec<String>>()
    }
    pub fn deselect(&mut self) {
        if !self.selected {
            return;
        }

        self.selected = false;
        for line in self.parsed_lines.iter_mut() {
            Self::remove_selection_color(line);
        }
    }
    pub fn select(&mut self) {
        if self.selected {
            return;
        }

        self.selected = true;
        for line in self.parsed_lines.iter_mut() {
            Self::add_selection_color(line);
        }
    }
    pub fn get_hash(&self) -> Option<&str> {
        if !self.selectable {
            return None;
        }
        let first_line = self.parsed_lines.first().unwrap();
        SmartLogParser::get_hash_from_commit_line(first_line)
    }
    fn selection_formatter() -> Output<'a> {
        return Output::TextBlock("\u{1b}[0;35m");
    }
    fn stop_formatter() -> Output<'a> {
        return Output::TextBlock("\u{1b}[0m");
    }
    fn add_selection_color(line: &mut Vec<Output<'a>>) {
        if line.len() > 4 {
            line.insert(4, Self::selection_formatter());
        } else if line.len() > 1 {
            line.insert(1, Self::selection_formatter());
        } else {
            if let Output::TextBlock(text) = line[0] {
                line.pop();
                for block in text.splitn(2, " ") {
                    line.push(Output::TextBlock(block));
                }
                // restore the space we removed with the split above
                line.insert(1, Output::TextBlock(" "));
            };
            line.insert(1, Self::selection_formatter());
        }
        line.push(Self::stop_formatter());
    }
    fn remove_selection_color(line: &mut Vec<Output<'a>>) {
        line.retain(|block| {
            match block {
                Output::Escape(AnsiSequence::SetGraphicsMode(codes)) => {
                    if codes.contains(&SELECTION_COLOR_CODE) {
                        return false;
                    }
                }
                Output::TextBlock(text) => {
                    if text.contains("\u{1b}[0;35m") {
                        return false;
                    }
                }
                _ => {}
            }
            true
        });
    }
}
