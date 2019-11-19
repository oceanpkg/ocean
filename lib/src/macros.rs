/// Counts the number of input tokens.
macro_rules! count {
    () => (0);
    ($x:tt) => (1);
    ($x:tt $($xs:tt)+) => (1 $(+ count!($xs))+);
}
