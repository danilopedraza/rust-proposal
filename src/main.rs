struct Block {
    size: u64,
}

struct Allocator {
    size: u64,
    block: Block,
}

impl Allocator {
}

fn build_allocator(size: u64) -> Allocator {
    Allocator {
        size,
        block: Block {
            size,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::build_allocator;
    #[test]
    fn init_allocator() {
        let allocator = build_allocator(0);

        assert_eq!(allocator.size, 0);
    }

    #[test]
    fn init_allocator_with_first_block() {
        let allocator = build_allocator(1);

        assert_eq!(allocator.block.size, 1);
    }
}
