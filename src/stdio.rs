#![allow(non_snake_case)]

use std::fmt;


#[derive(Debug)]
pub struct SkeletonDelimiters {
    pub skeleton_tag: String,
    pub skeleton_delimiter: String,
    pub solution_tag: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_example_parsing() {
        assert!(true);
    }
}
