struct Allocator {
    length: u64,
}

impl Allocator {
    pub fn size(&self) -> u64 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use crate::Allocator;
    #[test]
    fn init_manager() {
        let allocator = Allocator {
            length: 0,
        };

        assert_eq!(allocator.size(), 0);
    }
}
