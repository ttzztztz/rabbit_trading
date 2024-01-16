use std::sync::atomic::{AtomicI64, Ordering};

use super::common_trait::IdGeneratorTrait;

pub struct AutoIncrementIdGenerator {
    next_id: AtomicI64,
}

impl AutoIncrementIdGenerator {
    pub fn new(start: i64) -> Self {
        AutoIncrementIdGenerator {
            next_id: AtomicI64::new(start),
        }
    }
}

impl IdGeneratorTrait for AutoIncrementIdGenerator {
    fn generate(&self) -> String {
        self.next_id.fetch_add(1i64, Ordering::Relaxed).to_string()
    }
}

#[cfg(test)]
mod test_auto_increment_id_generator {
    use super::AutoIncrementIdGenerator;
    use crate::utils::id_generator::common_trait::IdGeneratorTrait;

    #[test]
    fn test_auto_increment_id_generator() {
        let auto_increment_id_generator = AutoIncrementIdGenerator::new(1i64);
        assert_eq!("1".to_owned(), auto_increment_id_generator.generate());
        assert_eq!("2".to_owned(), auto_increment_id_generator.generate());
        assert_eq!("3".to_owned(), auto_increment_id_generator.generate());
    }
}
