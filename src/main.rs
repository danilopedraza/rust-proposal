struct Allocator {
    size: u64,
}

impl Allocator {
}

#[cfg(test)]
mod tests {
    use crate::Allocator;
    #[test]
    fn init_manager() {
        let allocator = Allocator {
            size: 0,
        };

        assert_eq!(allocator.size, 0);
    }
}
