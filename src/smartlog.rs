use crate::{
    graph::{Item, ItemType},
    parser::SmartLogParser,
};

#[derive(Debug)]
pub struct SmartLog {
    pub items: Vec<ItemType>,
    selection_idx: usize,
}

impl SmartLog {
    pub fn new(raw_lines: &[String]) -> Self {
        let items = SmartLogParser::parse(raw_lines).unwrap();
        let selection_idx = Self::get_selected_item_index(&items).unwrap();
        Self {
            items,
            selection_idx,
        }
    }

    pub fn get_selected_commit_hash(&self) -> Option<&str> {
        let item = self.items.get(self.selection_idx).unwrap();
        if let ItemType::Commit(commit) = item {
            return commit.hash();
        }
        None
    }

    pub fn move_up(&mut self) {
        if self.selection_idx > 0 {
            let mut selection_candidate = self.selection_idx;
            for i in (0..self.selection_idx).rev() {
                if let ItemType::Commit(_) = self.items[i] {
                    selection_candidate = i;
                    break;
                }
            }
            if selection_candidate == self.selection_idx {
                return;
            }

            self.deselect_line_idx(self.selection_idx);
            self.select_line_index(selection_candidate);
        }
    }

    pub fn move_down(&mut self) {
        if self.selection_idx < self.items.len() - 1 {
            let mut selection_candidate = self.selection_idx;
            for i in (self.selection_idx + 1)..self.items.len() {
                if let ItemType::Commit(_) = self.items[i] {
                    selection_candidate = i;
                    break;
                }
            }
            if selection_candidate == self.selection_idx {
                return;
            }

            self.deselect_line_idx(self.selection_idx);
            self.select_line_index(selection_candidate);
        }
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        self.items
            .iter()
            .flat_map(|item| item.to_string_vec())
            .collect()
    }

    pub fn select_line_index(&mut self, item_idx: usize) {
        let item = self.items.get_mut(item_idx).unwrap();
        if let ItemType::Commit(commit) = item {
            commit.select();
            self.selection_idx = item_idx;
        }
    }

    pub fn deselect_line_idx(&mut self, item_idx: usize) {
        let item = self.items.get_mut(item_idx).unwrap();
        if let ItemType::Commit(commit) = item {
            commit.deselect();
        }
    }

    fn get_selected_item_index(items: &[ItemType]) -> Option<usize> {
        for (idx, item) in items.iter().enumerate() {
            if let ItemType::Commit(commit) = item {
                if commit.selected {
                    return Some(idx);
                }
            }
        }
        None
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
    fn test_new() {
        let smartlog = SmartLog::new(&raw_lines());
        assert_eq!(smartlog.items.len(), 12);
        assert_eq!(smartlog.selection_idx, 0);
    }

    #[test]
    fn test_get_selected_commit_hash() {
        let smartlog = SmartLog::new(&raw_lines());
        assert_eq!(smartlog.get_selected_commit_hash().unwrap(), "1cee5d55e");
    }

    #[test]
    fn test_moves() {
        let mut smartlog = SmartLog::new(&raw_lines());
        assert_eq!(smartlog.selection_idx, 0);
        // first commit, shouldn't move
        smartlog.move_up(); // 0
        assert_eq!(smartlog.selection_idx, 0);
        smartlog.move_down(); // 2
        smartlog.move_down(); // 4
        smartlog.move_up(); // 2
        assert_eq!(smartlog.selection_idx, 2);
        smartlog.move_down(); // 4
        smartlog.move_down(); // 6
        smartlog.move_down(); // 8
        assert_eq!(smartlog.selection_idx, 8);
        smartlog.move_up(); // 6
        assert_eq!(smartlog.selection_idx, 6);
        smartlog.move_down(); // 8
        assert_eq!(smartlog.selection_idx, 8);
        // last commit, shouldn't move
        smartlog.move_down(); // 8
        assert_eq!(smartlog.selection_idx, 8);
    }

    #[test]
    fn test_to_string_vec() {
        let smartlog = SmartLog::new(&raw_lines());
        let string_vec = smartlog.to_string_vec();
        assert_eq!(string_vec.len(), 15);
    }

    fn raw_lines() -> Vec<String> {
        RAW_LINES.iter().map(|x| x.to_string()).collect()
    }
}
