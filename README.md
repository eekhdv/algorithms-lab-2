# Algorithm's course 2nd lab

[Русская версия](https://github.com/khadievedem/algorithms-lab-2/blob/main/README-RU.md)

The task requires efficient processing of a large number of rectangles and points. Three different algorithms were implemented and tested to determine the most efficient one. The algorithms were tested on different volumes of initial data and points to determine their efficiency. The results of the tests were analyzed to determine the most efficient algorithm for this task.

## Table of Contents

- [Introduction](#introduction)
- [How to Run](#how-to-run)
- [Performance Comparison](#performance-comparison)
- [Graphics](#graphics)
- [Conclusion](#conclusion)

## Introduction

The laboratory work consists of implementing and comparing the performance of three different algorithms:

1. Brute Force Algorithm
2. Compressed Map Algorithm
3. Persistent Tree Algorithm

Each algorithm was implemented in Rust and tested on a dataset of [1..200000] points and [1..200000] rectangles.

## How to Run

To run tests, follow these steps:

1. Clone the repository
2. Read [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html) from ['The Rust Programming Language' book](https://doc.rust-lang.org/book/index.html).
3. Navigate to the project directory in your terminal
4. Run `cargo build --release` to build the project
5. Run `cargo test -r -- --show-output` to run the project

## Performance Comparison
The measured complexity that was given before:

| Algorithm | Time Complexity | Preparation |
|-----------|----------------|------------------|
| Brute Force | O(N) | O(1) |
| Compressed Map | O(log(N)) | O(N^3) |
| Persistent Tree | O(log(N)) | O(Nlog(N)) |

## Graphics
The graphics below show the results of running the algorithms on the dataset:
> here gonna be graphics
> soon...
> I promise

## Conclusion
