[![Build Status](https://travis-ci.org/replicatedu/replicatEdu_lib_skeleton-parser.svg?branch=master)](https://travis-ci.org/replicatedu/replicatEdu_lib_skeleton-parser) [![codecov](https://codecov.io/gh/replicatedu/replicatEdu_lib_skeleton-parser/branch/master/graph/badge.svg)](https://codecov.io/gh/replicatedu/replicatEdu_lib_skeleton-parser)

# replicatEdu_lib_skeleton-parser
This is a library to generate skeleton code for assignments using markup embedded in the assignment

# Example

The following example shows how this can be used to output code samples and skelton code:

```
//main.c
#include <stdio.h>
int main()
{
   ///!_SKELETON
   //!_ //change the below code to print "Hello, World!"
   //!_ printf("change me")
   ///!_SKELETON

   ///!_SOLUTION
   // printf() displays the string inside quotation
   printf("Hello, World!");
   ///!_SOLUTION

   return 0;
}
```

and the following code will break it apart:

```
use replicatEdu_lib_skeleton_parser::{SkeletonCode,SkeletonDelimiters};
use std::fs;

let filename = "example/main.c";
 
let delims = SkeletonDelimiters {
    skeleton_tag: "!_SKELETON".to_string(),
    skeleton_delimiter: "//!_ ".to_string(),
    solution_tag: "!_SOLUTION".to_string(),
};

let contents = match fs::read_to_string(&filename) {
    Ok(contents) => contents,
    Err(_) => panic!("parsing error!"),
};

let parsed_code = SkeletonCode::new(delims, contents);
println!("{}", parsed_code.unwrap());
```

and display:

```
Original:
#include <stdio.h>
int main()
{
   ///!_SKELETON
   //!_ //change the below code to print "Hello, World!"
   //!_ printf("change me")
   ///!_SKELETON

   ///!_SOLUTION
   // printf() displays the string inside quotation
   printf("Hello, World!");
   ///!_SOLUTION

   return 0;
}
Skeleton:
#include <stdio.h>
int main()
{
   //change the below code to print "Hello, World!"
   printf("change me")


   return 0;
}

Solution:
#include <stdio.h>
int main()
{

   // printf() displays the string inside quotation
   printf("Hello, World!");

   return 0;
}
```