use crate::{
    graph::{GraphItem, GraphItemEnum},
    parser::SmartLogParser,
};

#[derive(Debug)]
pub struct SmartLog {
    pub graph_items: Vec<GraphItemEnum>,
    selection_idx: usize,
}

impl SmartLog {
    pub fn new(raw_lines: &[String]) -> Self {
        let graph_items = SmartLogParser::parse(raw_lines).unwrap();
        let selection_idx = Self::get_selected_item_index(&graph_items).unwrap();
        Self {
            graph_items,
            selection_idx,
        }
    }

    pub fn get_selected_commit_hash(&self) -> Option<&str> {
        let item = self.graph_items.get(self.selection_idx).unwrap();
        if let GraphItemEnum::GraphCommit(graph_commit) = item {
            return graph_commit.hash();
        }
        None
    }

    pub fn move_up(&mut self) {
        if self.selection_idx > 0 {
            let mut selection_candidate = self.selection_idx;
            for i in (0..self.selection_idx).rev() {
                if let GraphItemEnum::GraphCommit(_) = self.graph_items[i] {
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
        if self.selection_idx < self.graph_items.len() - 1 {
            let mut selection_candidate = self.selection_idx;
            for i in (self.selection_idx + 1)..self.graph_items.len() {
                if let GraphItemEnum::GraphCommit(_) = self.graph_items[i] {
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
        self.graph_items
            .iter()
            .flat_map(|item| item.to_string_vec())
            .collect()
    }

    pub fn select_line_index(&mut self, item_idx: usize) {
        let item = self.graph_items.get_mut(item_idx).unwrap();
        if let GraphItemEnum::GraphCommit(graph_commit) = item {
            graph_commit.select();
            self.selection_idx = item_idx;
        }
    }

    pub fn deselect_line_idx(&mut self, item_idx: usize) {
        let item = self.graph_items.get_mut(item_idx).unwrap();
        if let GraphItemEnum::GraphCommit(graph_commit) = item {
            graph_commit.deselect();
        }
    }

    fn get_selected_item_index(graph_items: &[GraphItemEnum]) -> Option<usize> {
        for (idx, item) in graph_items.iter().enumerate() {
            if let GraphItemEnum::GraphCommit(graph_commit) = item {
                if graph_commit.selected {
                    return Some(idx);
                }
            }
        }
        None
    }
}
