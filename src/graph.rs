use enum_dispatch::enum_dispatch;

const LOCAL_COMMIT_HASH_COLOR: &str = "\u{1b}[0;93;1m";
const REMOTE_COMMIT_HASH_COLOR: &str = "\u{1b}[0;33m";

#[derive(Debug)]
pub struct GraphCommit {
    lines: Vec<Vec<String>>,
    pub selected: bool,
}
impl GraphCommit {
    pub fn new(parsed_lines: Vec<Vec<String>>, selected: bool) -> Self {
        Self {
            lines: parsed_lines,
            selected,
        }
    }

    pub fn hash(&self) -> Option<&str> {
        let first_line = self.parsed_lines().first().unwrap();

        let mut commit_hash_index = 0;
        for (index, text) in first_line.iter().enumerate() {
            if (text == REMOTE_COMMIT_HASH_COLOR) | (text == LOCAL_COMMIT_HASH_COLOR) {
                commit_hash_index = index + 1;
                break;
            }
        }
        Some(&first_line[commit_hash_index])
    }

    pub fn select(&mut self) {
        if self.selected {
            return;
        }

        self.selected = true;
        for line in self.lines.iter_mut() {
            Self::add_selection_color(line);
        }
    }

    pub fn deselect(&mut self) {
        if !self.selected {
            return;
        }

        self.selected = false;
        for line in self.lines.iter_mut() {
            Self::remove_selection_color(line);
        }
    }

    fn add_selection_color(line: &mut Vec<String>) {
        if line.len() > 4 {
            line.insert(4, Self::selection_formatter());
        } else if line.len() > 1 {
            line.insert(1, Self::selection_formatter());
        } else {
            let text = line.pop().unwrap();
            for block in text.splitn(2, ' ') {
                line.push(block.to_string());
            }
            // restore the space we removed with the split above
            line.insert(1, " ".to_string());
            line.insert(1, Self::selection_formatter());
        }
        line.push(Self::stop_formatter());
    }

    fn remove_selection_color(line: &mut Vec<String>) {
        line.retain(|text| !text.contains(&Self::selection_formatter()));
    }

    fn selection_formatter() -> String {
        "\u{1b}[0;35m".to_string()
    }

    fn stop_formatter() -> String {
        "\u{1b}[0m".to_string()
    }
}

#[derive(Debug)]
pub struct GraphGlyph {
    lines: Vec<Vec<String>>,
}
impl GraphGlyph {
    pub fn new(parsed_lines: Vec<Vec<String>>) -> Self {
        Self {
            lines: parsed_lines,
        }
    }
}

#[enum_dispatch(GraphItemEnum)]
pub trait GraphItem {
    fn parsed_lines(&self) -> &Vec<Vec<String>>;
    fn add_parsed_line(&mut self, parsed_line: Vec<String>);
    fn to_string_vec(&self) -> Vec<String> {
        self.parsed_lines()
            .iter()
            .map(|line| line.join(""))
            .collect()
    }
}

impl GraphItem for GraphCommit {
    fn parsed_lines(&self) -> &Vec<Vec<String>> {
        &self.lines
    }

    fn add_parsed_line(&mut self, parsed_line: Vec<String>) {
        self.lines.push(parsed_line);
    }
}

impl GraphItem for GraphGlyph {
    fn parsed_lines(&self) -> &Vec<Vec<String>> {
        &self.lines
    }

    fn add_parsed_line(&mut self, parsed_line: Vec<String>) {
        self.lines.push(parsed_line);
    }
}

#[enum_dispatch]
#[derive(Debug)]
pub enum GraphItemEnum {
    GraphCommit,
    GraphGlyph,
}

#[cfg(test)]
mod tests {

    use crate::parser::SmartLogParser;

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
    fn test_graph_commit() {
        let mut graph_commit = GraphCommit::new(vec![vec!["a".to_string()]], true);
        graph_commit.add_parsed_line(vec!["b".to_string()]);
        assert_eq!(
            graph_commit.lines,
            vec![vec!["a".to_string()], vec!["b".to_string()]]
        );
        assert!(graph_commit.selected);
        graph_commit.selected = false;
        assert!(!graph_commit.selected);
    }

    #[test]
    fn test_graph_glyph() {
        let mut graph_glyph = GraphGlyph::new(vec![vec!["a".to_string()]]);
        graph_glyph.add_parsed_line(vec!["b".to_string()]);
        assert_eq!(
            graph_glyph.parsed_lines(),
            &vec![vec!["a".to_string()], vec!["b".to_string()]]
        );
    }

    #[test]
    fn test_graph_item_enum() {
        let mut graph_item_enum_vec: Vec<GraphItemEnum> = vec![
            GraphCommit::new(vec![vec!["a".to_string()]], true).into(),
            GraphGlyph::new(vec![vec!["a".to_string()]]).into(),
        ];

        for item in graph_item_enum_vec.iter_mut() {
            item.add_parsed_line(vec!["b".to_string()]);

            match item {
                GraphItemEnum::GraphCommit(graph_commit) => {
                    assert_eq!(
                        graph_commit.parsed_lines(),
                        &vec![vec!["a".to_string()], vec!["b".to_string()]]
                    );
                    assert!(graph_commit.selected);
                    graph_commit.selected = false;
                    assert!(!graph_commit.selected);
                }
                GraphItemEnum::GraphGlyph(graph_glyph) => {
                    assert_eq!(
                        graph_glyph.parsed_lines(),
                        &vec![vec!["a".to_string()], vec!["b".to_string()]]
                    );
                }
            }
        }
    }

    #[test]
    fn test_select() {
        let graph_items = &mut SmartLogParser::parse(&raw_lines()).unwrap();

        let commit = &mut graph_items[2];
        match commit {
            GraphItemEnum::GraphCommit(graph_commit) => {
                assert!(!graph_commit.selected);
                assert_eq!(
                    graph_commit.parsed_lines()[0][4],
                    "  Dec 08 at 09:46  royrothenberg  "
                );
                assert_eq!(
                    graph_commit.parsed_lines()[1][1],
                    "  [pr body update] fix reviewstack option breaking stack list detection"
                );
                graph_commit.select();
                assert!(graph_commit.selected);
                assert_eq!(
                    graph_commit.parsed_lines()[0][1],
                    "\u{1b}[0;35m", // This item was inserted by select()
                );
                assert_eq!(
                    graph_commit.parsed_lines()[1][1],
                    "\u{1b}[0;35m", // This item was inserted by select()
                );
            }
            _ => panic!("Expected GraphCommit"),
        }
    }

    #[test]
    fn test_deselect() {
        let graph_items = &mut SmartLogParser::parse(&raw_lines()).unwrap();

        let commit = &mut graph_items[0];
        match commit {
            GraphItemEnum::GraphCommit(graph_commit) => {
                assert!(graph_commit.selected);
                assert!(graph_commit.parsed_lines()[0].contains(&"\u{1b}[0;35m".to_string()));
                assert!(graph_commit.parsed_lines()[1].contains(&"\u{1b}[0;35m".to_string()));
                graph_commit.deselect();
                assert!(!graph_commit.selected);
                assert!(!graph_commit.parsed_lines()[0].contains(&"\u{1b}[0;35m".to_string()));
                assert!(!graph_commit.parsed_lines()[1].contains(&"\u{1b}[0;35m".to_string()));
            }
            _ => panic!("Expected GraphCommit"),
        }
    }

    #[test]
    fn test_get_hash() {
        let graph_items = &mut SmartLogParser::parse(&raw_lines()).unwrap();

        let local_commit = &mut graph_items[0];
        match local_commit {
            GraphItemEnum::GraphCommit(graph_commit) => {
                assert_eq!(graph_commit.hash().unwrap(), "1cee5d55e");
            }
            _ => panic!("Expected GraphCommit"),
        }

        let remote_commit = &mut graph_items[4];
        match remote_commit {
            GraphItemEnum::GraphCommit(graph_commit) => {
                assert_eq!(graph_commit.hash().unwrap(), "ba27d4d13");
            }
            _ => panic!("Expected GraphCommit"),
        }
    }

    fn raw_lines() -> Vec<String> {
        RAW_LINES.iter().map(|x| x.to_string()).collect()
    }
}
