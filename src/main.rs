struct Block {
    size: u64,
    occupied: bool,
    right: Option<Box<Block>>,
}

impl Block {
    fn split(&mut self, size: u64) {
        self.right = Some(Box::new(build_block(self.size - size)));

        self.size = size;
    }
    
    fn free(&mut self) {
        self.occupied = false;

        self.right.take().map(|next| {
            if !next.occupied {
                self.size += next.size;
                self.right = next.right;
            } else {
                self.right = Some(next);
            }
        });
    }
}

fn build_block(size: u64) -> Block {
    Block {
        size,
        occupied: false,
        right: None,
    }
}

struct Allocator {
    size: u64,
    block: Block,
}

impl Allocator {
    fn alloc(&mut self, size: u64) {
        let mut block = &mut self.block;
        while block.occupied {
            block = block.right.as_mut().unwrap();
        }

        if size < block.size {
            block.split(size);
        }

        block.occupied = true;
    }
}

fn build_allocator(size: u64) -> Allocator {
    Allocator {
        size,
        block: build_block(size),
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
    fn allocator_starts_with_free_block() {
        let allocator = build_allocator(1);

        assert_eq!(allocator.block.occupied, false);        
    }

    #[test]
    fn allocator_reserves_block() {
        let mut allocator = build_allocator(1);
        allocator.alloc(1);

        assert_eq!(allocator.block.occupied, true);        
    }

    #[test]
    fn allocator_splits_block() {
        let mut allocator = build_allocator(2);
        allocator.alloc(1);

        assert_eq!(allocator.block.right.unwrap().occupied, false);        
    }

    #[test]
    fn allocator_splits_block_with_new_sizes() {
        let mut allocator = build_allocator(2);
        allocator.alloc(1);

        assert_eq!(allocator.block.size, 1);
        assert_eq!(allocator.block.right.unwrap().size, 1);      
    }

    #[test]
    fn allocator_searches_for_free_block() {
        let mut allocator = build_allocator(2);
        allocator.alloc(1);
        allocator.alloc(1);

        assert_eq!(allocator.block.right.unwrap().occupied, true);
    }

    #[test]
    fn allocator_deallocates() {
        let mut allocator = build_allocator(1);
        allocator.alloc(1);
        allocator.block.free();

        assert_eq!(allocator.block.occupied, false);   
    }

    #[test]
    fn deallocation_triggers_merge() {
        let mut allocator = build_allocator(2);
        allocator.alloc(1);
        allocator.block.free();

        assert!(allocator.block.right.is_none());
    }

    #[test]
    fn merge_restores_size() {
        let mut allocator = build_allocator(2);
        allocator.alloc(1);
        allocator.block.free();

        assert_eq!(allocator.block.size, 2);
    }

    #[test]
    fn merge_occurs_only_when_possible() {
        let mut allocator = build_allocator(2);
        allocator.alloc(1);
        allocator.alloc(1);
        allocator.block.free();

        assert_eq!(allocator.block.size, 1);
    }

    #[test]
    fn merge_preserves_next_blocks() {
        let mut allocator = build_allocator(3);
        allocator.alloc(1);
        allocator.alloc(1);
        allocator.alloc(1);

        allocator.block.right.as_mut().unwrap().free();
        allocator.block.free();

        assert!(allocator.block.right.is_some());
    }
}

fn main() {}
