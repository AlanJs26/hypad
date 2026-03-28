use std::collections::HashMap;

use crate::domain::{GroupSource, ScratchpadGroup, WindowSelector};

#[derive(Debug, Default)]
pub struct RuntimeState {
    groups: HashMap<String, ScratchpadGroup>,
}

impl RuntimeState {
    pub fn register(&mut self, group: ScratchpadGroup) {
        self.groups.insert(group.id.clone(), group);
    }

    pub fn upsert_cli_group(&mut self, id: String, selector: WindowSelector) {
        self.register(ScratchpadGroup {
            id,
            selector,
            source: GroupSource::Cli,
        });
    }

    pub fn apply_config_snapshot(&mut self, groups: Vec<(String, WindowSelector)>) {
        self.groups.retain(|_, group| group.source != GroupSource::Config);

        for (id, selector) in groups {
            self.register(ScratchpadGroup {
                id,
                selector,
                source: GroupSource::Config,
            });
        }
    }

    pub fn remove(&mut self, id: &str) -> bool {
        self.groups.remove(id).is_some()
    }

    pub fn get(&self, id: &str) -> Option<&ScratchpadGroup> {
        self.groups.get(id)
    }

    pub fn list(&self) -> impl Iterator<Item = &ScratchpadGroup> {
        self.groups.values()
    }

    pub fn source_counts(&self) -> (usize, usize, usize) {
        let total = self.groups.len();
        let cli = self
            .groups
            .values()
            .filter(|group| group.source == GroupSource::Cli)
            .count();
        let config = total.saturating_sub(cli);
        (total, cli, config)
    }

}
