use crate::parser::engine::Engine;

impl Engine {
    pub fn process_list_cmd(self) {
        println!(
            "Blocks ({}):\n{}",
            self.document.blocks_len(),
            self.document
                .get_blocks()
                .iter()
                .map(|b| format!("- {}", b.identifier()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
