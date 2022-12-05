# Advent of Code 2022!
Hello Everyone! This is my second year with Advent of code and for a second time I'm programming in Rust.  Hopefully it will go smoother than last time!

### Overall Features and Goals
- launches from the command line and takes arguments for the day and an input file.  Calculates the two results.
- Using the Clap crate this year for command line processing.
- Going to implement tests for the daily challenges with the provided test input and output as test validation.

### Day 0 (T minus 1)
- Got the Git repository set up and prepared some input/output functions and command line processing.  Added nearly automatic unit tests with macro rules to write the body of the test functions.  Much code borrowed from last year, but a much better setup than 2021.

### Day 1
#### Approach and Unique features
- It's begun!  First day complete!
- Nothing too special in this code.  Used vectors to store data.  Decided not to sort and select the top 3 for the second challenge, but used comparisons to select the top 3. Sorting would have been an option for less code (probably just 2-3 lines) but much more processor intensive.  I'll be using that in later days I'm sure.

### Day 2
#### Approach and Unique features
- For This Day I built built struct with methods and intended to change the data in the struct between challange 1 and 2.  It didn't work out to resuse the struct in a meaningful way, so that was extra coding for no practical benefit. 
- I used conditional statements for challenge One and nested match statements for challenge Two.  Friends who opted to solve both with hashmaps had a faster answer and less code lines for sure!
### Day 3
#### Approach and Unique features
- Hashmaps were my go-to data structure this day.  I think there are some better crates for getting, comparing, and setting values, but I used std only.  

### Day 4
#### Approach and Unique features
- These two challenges involved character and line parsing as the primary programming challenge.  Since the data sets were short, I went with hard-wired line decoding.  Not much use of loops per line as there were only 4 integer data points per line.  
- With the data in integers, the remainder of the programming seemed easy as I did not find the logic to be difficult to visualize.

### Day 5
#### Approach and Unique features
- Complete! Took way too long debugging input.  Challenge 2 was a gift!

### Day 6
#### Approach and Unique features
- TBC.

### Day 7
#### Approach and Unique features
- TBC.

### Day 8
#### Approach and Unique features
- TBC.

### Day 9
#### Approach and Unique features
- TBC.

### Day 10 and beyond!
If I get this far, I will be doing better than 2021!
