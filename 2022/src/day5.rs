// NOTE: Stacks in memory will be Vec<Crate> where the beginning of the vec is the bottom of the stack
// and the end of the vec is the top of the stack.
// 1. Read in the file
// 2. Separate drawing from instructions based on empty line
// 3. Parse drawing into number of stacks:
// 3a. Instantiate 3 vecs with_capacity == number of lines minus the stack identifier line to prevent allocations
// 3b. Read last line to determine stacks to create
// 3c. Read from end to beginning, adding each crate_id to a stack
// 4. Parse instructions into a Vec<Instruction>
// 5. Return Operation struct containing map and instructions
// 6. Loop instructions, mutating map accordingly
// 7. Return resulting map
// 8. Get all top-level crates (end of each vec) in order and concatenate them into a message

fn main() {}
