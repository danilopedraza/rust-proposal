struct Manager {
    length: u64,
}

impl Manager {
    pub fn size(&self) -> u64 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use crate::Manager;
    #[test]
    fn init_manager() {
        let manager = Manager {
            length: 0,
        };

        assert_eq!(manager.size(), 0);
    }
}
