#![allow(non_snake_case)]

use std::fmt;

#[derive(Debug)]
pub struct SkeletonCode {
    pub original: String,
    pub skeleton_code: String,
    pub solution_code: String,
}

impl SkeletonCode {
    /// parses a code sample using provided delimeters into skeleton and solution code
    ///
    /// # Arguments
    ///
    /// * `delimeters` - customizable delimiters to segregate code into different sections
    /// * `contents` - the contents of the file to parse
    /// # Example
    ///
    /// ```
    /// use replicatEdu_lib_skeleton_parser::{SkeletonCode,SkeletonDelimiters};
    /// use std::fs;
    ///
    /// let filename = "example/main.c";
    /// 
    /// let delims = SkeletonDelimiters {
    ///     skeleton_tag: "///!_SKELETON".to_string(),
    ///     skeleton_delimiter: "//!_ ".to_string(),
    ///     solution_tag: "///!_SOLUTION".to_string(),
    /// };
    /// 
    /// let contents = match fs::read_to_string(&filename) {
    ///     Ok(contents) => contents,
    ///     Err(_) => panic!("parsing error!"),
    /// };
    /// 
    /// let parsed_code = SkeletonCode::new(delims, contents);
    /// println!("{}", parsed_code.unwrap());
    /// ```
    pub fn new(
        delimeters: SkeletonDelimiters,
        contents: String,
    ) -> Result<SkeletonCode, &'static str> {
        let mut skelly_code = SkeletonCode {
            original: contents.clone(),
            skeleton_code: String::new(),
            solution_code: String::new(),
        };

        //bools for state machine
        let mut in_skelly_block: bool = false;
        let mut in_solution_block: bool = false;

        //iterate through the solution code
        for (i, line) in skelly_code.original.lines().enumerate() {
            //check for invalid states
            if in_skelly_block && in_solution_block {
                let err = "cannot be in the solution and skeleton block at the same time";
                println!("error on line {}: {}", i, err);
                return Err(err);
            }
            if line.contains(&delimeters.skeleton_delimiter) && !in_skelly_block {
                let err = "skeleton delimeter contained outside of skeleton block";
                println!("error on line {}: {}", i, err);
                return Err(err);
            }

            //check to see if the line is the beginning of a tag or end
            if line.contains(&delimeters.skeleton_tag) {
                in_skelly_block ^= true;
                continue;
            }
            if line.contains(&delimeters.solution_tag) {
                in_solution_block ^= true;
                continue;
            }

            //add code accordingly to states

            //this is a line that should only be added to the skeleton code
            if line.contains(&delimeters.skeleton_delimiter) && in_skelly_block {
                let stripped_line = line.replace(&delimeters.skeleton_delimiter, "");
                skelly_code.skeleton_code.push_str(&stripped_line);
                skelly_code.skeleton_code.push_str("\r\n");
            }
            //its in neither state, so added to both
            if !in_skelly_block && !in_solution_block {
                skelly_code.solution_code.push_str(&line);
                skelly_code.solution_code.push_str("\r\n");
                skelly_code.skeleton_code.push_str(&line);
                skelly_code.skeleton_code.push_str("\r\n");
            }
            //add it to only the solution block
            if in_solution_block {
                skelly_code.solution_code.push_str(&line);
                skelly_code.solution_code.push_str("\r\n");
            }
        }

        //check to ensure it ended at a valid state
        if in_skelly_block {
            return Err("exited without terminating a skeleton block");
        }
        if in_solution_block {
            return Err("exited without terminating a solution block");
        }
        Ok(skelly_code)
    }
}

impl fmt::Display for SkeletonCode {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "Original:\r\n{}\r\nSkeleton:\r\n{}\r\nSolution:\r\n{}",
            self.original, self.skeleton_code, self.solution_code
        )
    }
}

#[derive(Debug)]
pub struct SkeletonDelimiters {
    pub skeleton_tag: String,
    pub skeleton_delimiter: String,
    pub solution_tag: String,
}

/// returns a default representation of the delimiters
///
/// # Arguments
///
/// * `delimeters` - customizable delimiters to segregate code into different sections
/// * `contents` - the contents of the file to parse
/// # Example
///
/// ```
/// use replicatEdu_lib_skeleton_parser::{SkeletonCode,return_default_delim};
/// use std::fs;
///
/// let filename = "example/main.c";
/// let contents = match fs::read_to_string(&filename) {
///     Ok(contents) => contents,
///     Err(_) => panic!("parsing error!"),
/// };
/// let delims = return_default_delim();
/// let parsed_code = SkeletonCode::new(delims, contents);
/// println!("{}", parsed_code.unwrap());
/// ```

pub fn return_default_delim() -> SkeletonDelimiters {
    SkeletonDelimiters {
        skeleton_tag: "///!_SKELETON".to_string(),
        skeleton_delimiter: "//!_ ".to_string(),
        solution_tag: "///!_SOLUTION".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_example_parsing() {
        let filename = "example/main.c";
        let contents = match fs::read_to_string(&filename) {
            Ok(contents) => contents,
            Err(_) => panic!("should not have happened"),
        };
        let delims = return_default_delim();
        let parsed_code = SkeletonCode::new(delims, contents);
        println!("{}", parsed_code.unwrap());
    }
    #[test]
    fn test_invalid_skeleton_closure() {
        let delims = return_default_delim();
        let contents =
            "///!_SKELETON\r\n//!_ //change the below code to print \"Hello, World!\"\r\n"
                .to_string();
        assert!(
            SkeletonCode::new(delims, contents).is_err(),
            "should error: did not close skeleton tags"
        );
    }
    #[test]
    fn test_invalid_solution_closure() {
        let delims = return_default_delim();
        let contents =
            "///!_SOLUTION\r\n//!_ //change the below code to print \"Hello, World!\"\r\n"
                .to_string();
        assert!(
            SkeletonCode::new(delims, contents).is_err(),
            "should error: did not close solution tags"
        );
    }

    #[test]
    fn test_invalid_same_time() {
        let delims = return_default_delim();
        let contents = "///!_SOLUTION\r\n///!_SKELETON".to_string();
        assert!(
            SkeletonCode::new(delims, contents).is_err(),
            "should error: cannot be in solution and skeleton same time"
        );
    }
    #[test]
    fn test_skeleton_outside_block() {
        let delims = return_default_delim();
        let contents = "///!_SOLUTION\r\n//!_ should_fail_on_this()\r\n///!_SOLUTION".to_string();
        assert!(
            SkeletonCode::new(delims, contents).is_err(),
            "should error: cannot have skeleton annotations outside a skeleton block"
        );
    }
}
