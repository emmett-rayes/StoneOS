pub trait Cpu {
    fn park_core() -> !;
}
