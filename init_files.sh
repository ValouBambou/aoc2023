#! /bin/bash

for n in $(seq 1 25);
do 
    padded_n=n # will be mutated with the line below
    printf -v padded_n "%02d" $n
    mkdir -p "src/day$padded_n"
    filename="src/day$padded_n/mod.rs"
    filename_in="src/day$padded_n/input.txt"
    filename_ex="src/day$padded_n/example1.txt"
    filename_ex2="src/day$padded_n/example2.txt"
    echo $filename_in $filename_ex
    touch $filename_in
    touch $filename_ex
    touch $filename_ex2

    echo "use crate::DaySolution;" > $filename
    echo "" >> $filename
    echo "pub struct Day$n;" >> $filename
    echo "" >> $filename
    echo "impl DaySolution for Day$n {" >> $filename
    echo "    const DAY: u8 = $n;" >> $filename
    echo "    const INPUT: &'static str = include_str!(\"input.txt\");" >> $filename
    echo "    const EXAMPLE_1: &'static str = include_str!(\"example1.txt\");" >> $filename
    echo "    const EXAMPLE_2: &'static str = include_str!(\"example2.txt\");" >> $filename
    echo "    const EXAMPLE_ANSWER_1: usize = 42;" >> $filename
    echo "    const EXAMPLE_ANSWER_2: usize = 42;" >> $filename
    echo "" >> $filename
    echo "    fn solve_part1(_input:&str) -> usize {" >> $filename
    echo "        todo!()" >> $filename
    echo "    }" >> $filename
    echo "" >> $filename
    echo "    fn solve_part2(_input:&str) -> usize {" >> $filename
    echo "        todo!()" >> $filename
    echo "    }" >> $filename
    echo "}" >> $filename
done
