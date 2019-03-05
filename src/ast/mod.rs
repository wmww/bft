mod root;
mod section;
mod segment;

pub use self::root::parse;
use self::section::Section;
use self::segment::Segment;

#[cfg(test)]
mod tests;
