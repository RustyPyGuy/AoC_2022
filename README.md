# Advent of Code 2022!
Hello Everyone! This is my second year with Advent of code and for a second time I'm programming in Rust.  Hopefully it will go smoother than last time!

### Overall Features and Goals
- launches from the command line and takes arguments for the day and an input file.  Calculates the two results.
- Using the Clap crate this year for command line processing.
- Going to implement tests for the daily challenges with the provided test input and output as test validation.

### Day 0 (T minus 1)
- Got the Git repository set up and prepared some input/output functions and command line processing.  Added nearly automatic unit tests with macro rules to write the body of the test functions.  Much code borrowed from last year, but a much better setup than 2021.

### Day 1
- It's begun!  First day complete!
- Nothing too special in this code.  Used vectors to store data.  Decided not to sort and select the top 3 for the second challenge, but used comparisons to select the top 3. Sorting would have been an option for less code (probably just 2-3 lines) but much more processor intensive.  I'll be using that in later days I'm sure.

### Day 2
- For This Day I built built struct with methods and intended to change the data in the struct between challange 1 and 2.  It didn't work out to resuse the struct in a meaningful way, so that was extra coding for no practical benefit. 
- I used conditional statements for challenge One and nested match statements for challenge Two.  Friends who opted to solve both with hashmaps had a faster answer and less code lines for sure!
### Day 3
- Hashmaps were my go-to data structure this day.  I think there are some better crates for getting, comparing, and setting values, but I used std only.  

### Day 4
- These two challenges involved character and line parsing as the primary programming challenge.  Since the data sets were short, I went with hard-wired line decoding.  Not much use of loops per line as there were only 4 integer data points per line.  
- With the data in integers, the remainder of the programming seemed easy as I did not find the logic to be difficult to visualize.

### Day 5
- Complete! Took way too long debugging input.
- Challenge 2 was a gift and I only had to change one line!

### Day 6
- The main approach was to read the input as an iterator, push the current value and set of comparisons into a HashSet, and then see if there were any dropped duplicates.
- *Added LOTS of comments to the code to explain it nearly line by line.*

### Day 7
- Solved finally!  This one took much longer that the others because I wanted to do it the "right" way. Challenge 2 went very quickly.
- Data is stored in a true tree structure with the `id_tree` crate.
- Made use of `Rc` and `RefCell` to deal with mutable and immutable references to the same data source in a loop that were on different arms of a match statement.  This is a personal first for me and there was a lot of fighting with the borrow checker.
- Did not use a recurive function for adding values in the tree.  An earlier try revealed that this would create a stack overflow.  Used loops and temporary storage to accumulate data.Don't judge me harshly. 

### Day 8
- Solved.  Mainly a lot of iterations through rows and columns with loop breaks where possible.
- used the `grid` crate for 2 dimensional addressing.
- update - It's failing unit tests days later and I don't know why. Puzzle output is still bcorrect. Tests to be debugged.

### Day 9
- Solved! This was a fun one. 
- Several nested loops.  Probably could make this one more performant with some early loop breakouts and definitely by sharing mutable variables to reduce memory operations.

### Day 10
- Skipped for now. Check back later.

### Day 11
- Solved part One! Rumors are part 2 has a non-obvious mathemetical concept that I'll come back to.
- Reformatted the input into YAML with string slicing. Used `serde` and `serde_yaml` to process the input and then procesed it further to get in the structs I wanted. This was the long way around inputting the data, but I wanted to learn the concepts of serialization and deserialization.  I spent a lot of extra time on that.
- Also a good use of a `VecDeque`.  

### Day 14
- Solved! I tend to prefer the map puzzles, so I liked this one.
- Used the `Grid` crate and matricies of `bool` data types for the rocks and sands.  I was overal pleased with the organization of this one.

### More Days Ahead!
If I get much beyond Day 10, I be doing better than 2021!
