# Fast Fibonacci Calculator

A blazingly fast command-line Fibonacci calculator using matrix exponentiation and fast doubling.

This project was inspired by [this YouTube video](https://www.youtube.com/watch?v=KzT9I1d-LlQ).  
I created it to explore Rust's capabilities and, admittedly, to flex a bit!

## Features

- Computes Fibonacci numbers efficiently using matrix exponentiation and fast doubling algorithm.
- Supports large Fibonacci numbers
- Provides options to display calculation time and detailed help.

## Installation

cargo install blazing_fibonacci

## Benchmarks

blazing_fibonacci 18000000 -t
Fibonacci number of 18000000 calculated, use -p to display
Time taken: 956.54ms

(BENCHMARKS DONE ON )
 CPU: 11th Gen Intel(R) Core(TM) i5-1135G7 @ 2.40GHz @ 20
    GPU: TigerLake-LP GT2 [Iris Xe Graphics]
 Memory: 7392/19778 MiB

## Changelog 0.25

Decided to remove Rayon dependency as wanted to stay true to original goals,
Implemented the fast doubling algorithm
Error handling, avoiding a panic basically if no number entered.

## Changelog 0.2

Added Rayon as a dependency, it's pretty much in every crate anyway.
It can now calculate up to 11million in less than a second on a shitty computer!

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
