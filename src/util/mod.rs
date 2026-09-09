pub mod arena;

pub enum Completion {
    Unfinished,
    Finished,
}

pub fn bi_range<T: PartialOrd>(a: T, b: T) -> std::ops::Range<T> {
    if a < b { a..b } else { b..a }
}
