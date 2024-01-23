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
