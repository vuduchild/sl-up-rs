use ansi_parser::{AnsiParser, AnsiSequence, Output};

use crate::graph::{Commit, Glyph, Item, ItemType};

const SELECTION_COLOR_CODE: u8 = 35;

pub struct SmartLogParser {}
impl SmartLogParser {
    pub fn parse(raw_lines: &[String]) -> Option<Vec<ItemType>> {
        let mut items: Vec<ItemType> = Vec::new();
        let mut parsed_lines: Vec<Vec<Output>> =
            raw_lines.iter().map(|x| x.ansi_parse().collect()).collect();

        while !parsed_lines.is_empty() {
            let mut line = parsed_lines.remove(0);
            Self::pre_process_line(&mut line);

            if Self::is_commit_line(&line) {
                // commit hash and metadata
                let selected = Self::has_line_selection_coloring(&line);
                items.push(
                    Commit::new(vec![line.iter().map(|x| x.to_string()).collect()], selected)
                        .into(),
                );
            } else if Self::parsed_line_to_string(&line).trim().contains(' ') {
                // commit message
                items
                    .last_mut()
                    .unwrap()
                    .add_parsed_line(line.iter().map(|x| x.to_string()).collect());
            } else {
                // only a graph element
                items.push(Glyph::new(vec![line.iter().map(|x| x.to_string()).collect()]).into());
            }
        }
        Some(items)
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

    fn pre_process_line(line: &mut Vec<Output>) {
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

#[cfg(test)]
mod tests {

    use super::*;

    const RAW_LINES: [&str; 15] = [
        "  @  \u{1b}[0;35m\u{1b}[0;93;1m1cee5d55e\u{1b}[0m\u{1b}[0;35m  Dec 08 at 09:46  royrothenberg  \u{1b}[0;36m#780 Closed\u{1b}[0m\u{1b}[0;35m \u{1b}[0;31m✗\u{1b}[0m",
        "  │  \u{1b}[0;35m[pr body update] update stack list without overwriting PR title and body\u{1b}[0m",
        "  │",
        "  o  \u{1b}[0;93;1mc3bd9e5fa\u{1b}[0m  Dec 08 at 09:46  royrothenberg  \u{1b}[0;38;2;141;148;158m#779 Unreviewed\u{1b}[0m \u{1b}[0;31m✗\u{1b}[0m",
        "╭─╯  [pr body update] fix reviewstack option breaking stack list detection",
        "│",
        "o  \u{1b}[0;33mba27d4d13\u{1b}[0m  Dec 07 at 22:20  \u{1b}[0;32mremote/main\u{1b}[0m",
        "╷",
        "╷ o  \u{1b}[0;93;1m2f85065e7\u{1b}[0m  Nov 28 at 11:49  royrothenberg  \u{1b}[0;36m#781 Closed\u{1b}[0m \u{1b}[0;32m✓\u{1b}[0m",
        "╭─╯  [isl] increase width of diff window in split stack edit panel",
        "│",
        "o  \u{1b}[0;33m0e069ab09\u{1b}[0m  Nov 21 at 13:16",
        "│",
        "~",
        "",
    ];

    #[test]
    fn graph_items() {
        let items = SmartLogParser::parse(&raw_lines()).unwrap();
        assert!(items.len() == 12);
        assert_eq!(items[0].parsed_lines().len(), 2);
        assert_eq!(items[1].parsed_lines().len(), 1);
        let commit = if let ItemType::Commit(commit) = &items[0] {
            commit
        } else {
            panic!("Expected GraphCommit");
        };
        assert!(commit.selected);
    }

    fn raw_lines() -> Vec<String> {
        RAW_LINES.iter().map(|x| x.to_string()).collect()
    }
}
