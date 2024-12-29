# Fast Fibonacci Calculator

A blazingly fast command-line Fibonacci calculator using matrix exponentiation.

This project was inspired by [this YouTube video](https://www.youtube.com/watch?v=KzT9I1d-LlQ).  
I created it to explore Rust's capabilities and, admittedly, to flex a bit!



## Features

- Computes Fibonacci numbers efficiently using matrix exponentiation.
- Supports large Fibonacci numbers with blazing speed.
- Provides options to display calculation time and detailed help.

## Installation

cargo install blazing_fibonacci

## Benchmarks

~ ❯ blazing_fibonacci 7000000 -t 

Fibonacci number of 7000000 calculated, use -p to display

Time taken: 1.37s

~ ❯ blazing_fibonacci 650000 -t                                          

Fibonacci number of 650000 calculated, use -p to display

Time taken: 59.20ms


~ ❯ blazing_fibonacci 1000000 -t           

Fibonacci number of 1000000 calculated, use -p to display

Time taken: 81.59ms

## Changelog 0.2

Added Rayon as a dependency, it's pretty much in every crate anyway.
It can now calculate up to 11million in less than a second on a shitty computer!

blazing_fibonacci  11000000 -t
Fibonacci number of 11000000 calculated
Time taken: 935.18ms

## Usage

To install the calculator, use the following command:

blazing_fibonacci 1000           # Calculate the 1000th Fibonacci number

blazing_fibonacci 1000 -p     # Calculate and print the full number

blazing_fibonacci  1000 -t     # Show the calculation time

blazing_fibonacci -h             # Show help and usage instructions

## Verifications

 for i in {1..1000}; do                                                                            
  blazing_fibonacci $i -p -t | sed -n '2,3p' | awk -v num=$i '{print num, $0}' >> ~/TESTFIB.txt
done
