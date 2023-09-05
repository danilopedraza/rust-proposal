struct MemoryBlock {
    length: u64,
}

impl MemoryBlock {
    pub fn size(&self) -> u64 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use crate::MemoryBlock;
    #[test]
    fn init_manager() {
        let block = MemoryBlock {
            length: 0,
        };

        assert_eq!(block.size(), 0);
    }
}
