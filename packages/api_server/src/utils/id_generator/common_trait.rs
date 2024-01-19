pub trait IdGeneratorTrait: Send + Sync {
    fn generate(&self) -> String;
}
