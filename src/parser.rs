use ansi_parser::{AnsiParser, AnsiSequence, Output};

use crate::smartlog::SelectableCommit;

const SELECTION_COLOR_CODE: u8 = 35;
const LOCAL_COMMIT_HASH_COLOR_CODE: u8 = 93;
const REMOTE_COMMIT_HASH_COLOR_CODE: u8 = 33;

pub struct SmartLogParser {}
impl SmartLogParser {
    pub fn parse(raw_lines: &[String]) -> Option<Vec<SelectableCommit>> {
        let mut selectable_commits = Vec::new();
        let mut parsed_lines: Vec<Vec<Output>> =
            raw_lines.iter().map(|x| x.ansi_parse().collect()).collect();

        while !parsed_lines.is_empty() {
            let mut line = parsed_lines.remove(0);
            Self::preprocess_line(&mut line);

            if Self::is_commit_line(&line) {
                // commit hash and metadata
                let selected = Self::has_line_selection_coloring(&line);
                selectable_commits.push(SelectableCommit::new(vec![line], true, selected));
            } else if Self::parsed_line_to_string(&line).trim().contains(' ') {
                // commit message
                selectable_commits
                    .last_mut()
                    .unwrap()
                    .parsed_lines
                    .push(line);
            } else {
                // only a graph element
                selectable_commits.push(SelectableCommit::new(vec![line], false, false));
            }
        }

        Some(selectable_commits)
    }

    pub fn get_hash_from_commit_line<'a>(line: &'a [Output]) -> Option<&'a str> {
        let mut commit_hash_index = 0;
        for (index, block) in line.iter().enumerate() {
            if let Output::Escape(AnsiSequence::SetGraphicsMode(codes)) = block {
                if codes.contains(&REMOTE_COMMIT_HASH_COLOR_CODE)
                    | codes.contains(&LOCAL_COMMIT_HASH_COLOR_CODE)
                {
                    commit_hash_index = index + 1;
                    break;
                }
            }
        }
        if let Output::TextBlock(text) = &line[commit_hash_index] {
            return Some(text);
        }
        None
    }

    pub fn parsed_line_to_string(line: &[Output]) -> String {
        line.iter()
            .map(|block| match block {
                Output::TextBlock(text) => text.to_string(),
                Output::Escape(seq) => seq.to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn has_line_selection_coloring(line: &[Output]) -> bool {
        for block in line.iter() {
            match block {
                Output::Escape(AnsiSequence::SetGraphicsMode(codes)) => {
                    if codes.contains(&SELECTION_COLOR_CODE) {
                        return true;
                    }
                }
                Output::TextBlock(text) => {
                    if text.contains("\u{1b}[0;35m") {
                        return false;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn is_commit_line(line: &[Output]) -> bool {
        let mut first_text_block =
            Self::get_first_text_block_contents(line).unwrap_or("".to_string());

        first_text_block = first_text_block.trim().to_string();
        if first_text_block.chars().collect::<Vec<char>>().len() == 3
            && first_text_block.contains(' ')
        {
            first_text_block = first_text_block.split(' ').last().unwrap().to_string();
        }

        if ["@", "o"].contains(&first_text_block.as_str()) {
            return true;
        }
        false
    }

    fn get_first_text_block_contents(line: &[Output]) -> Option<String> {
        for block in line.iter() {
            if let Output::TextBlock(text) = block {
                return Some(text.trim().to_string());
            }
        }
        None
    }

    fn preprocess_line(line: &mut Vec<Output>) {
        if line.len() == 1 {
            if let Output::TextBlock(text) = &line[0] {
                let (graph, new_text) = Self::split_graph_from_text(text).unwrap();
                line[0] = Output::TextBlock(graph);
                line.push(Output::TextBlock(new_text))
            }
        }
    }

    fn split_graph_from_text(text: &str) -> Option<(&str, &str)> {
        let mut idx = 0;
        let mut found = false;
        for (i, char) in text.char_indices() {
            if found {
                idx = i;
                break;
            }
            if ["│", "╯", "╷"].contains(&char.to_string().as_str()) {
                found = true;
            }
        }
        Some(text.split_at(idx))
    }
}
