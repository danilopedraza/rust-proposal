struct Block {
    size: u64,
    occupied: bool,
}

struct Allocator {
    size: u64,
    block: Block,
}

impl Allocator {
    fn alloc(&self, _size: u64) {

    }
}

fn build_allocator(size: u64) -> Allocator {
    Allocator {
        size,
        block: Block {
            size,
            occupied: true,
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

    #[test]
    fn allocator_reserves_block() {
        let allocator = build_allocator(1);
        allocator.alloc(1);

        assert_eq!(allocator.block.occupied, true);        
    }
}
