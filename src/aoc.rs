pub trait AOCPart {
    fn new() -> Self;
    fn solve(&mut self, input: &str) -> String;
}
