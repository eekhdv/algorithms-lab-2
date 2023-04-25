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
The below graphics provide an explanation of the complexity and comparison of the algorithms:

| Algorithm on map | Algorithm on persistent tree |
| ------ | ------ |
| ![Скорость-подготовки-алгоритма-на-Карте](https://raw.githubusercontent.com/khadievedem/algorithms-lab-2/main/graphics/%D0%A1%D0%BA%D0%BE%D1%80%D0%BE%D1%81%D1%82%D1%8C%20%D0%BF%D0%BE%D0%B4%D0%B3%D0%BE%D1%82%D0%BE%D0%B2%D0%BA%D0%B8%20%D0%B0%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC%D0%B0%20%D0%BD%D0%B0%20%D0%9A%D0%B0%D1%80%D1%82%D0%B5.png) | ![Скорость-подготовки-алгоритма-на-персистентном-дереве](https://raw.githubusercontent.com/khadievedem/algorithms-lab-2/main/graphics/%D0%A1%D0%BA%D0%BE%D1%80%D0%BE%D1%81%D1%82%D1%8C%20%D0%BF%D0%BE%D0%B4%D0%B3%D0%BE%D1%82%D0%BE%D0%B2%D0%BA%D0%B8%20%D0%B0%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC%D0%B0%20%D0%BD%D0%B0%20%D0%BF%D0%B5%D1%80%D1%81%D0%B8%D1%81%D1%82%D0%B5%D0%BD%D1%82%D0%BE%D0%BC%20%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%B5.png) |

| Comparison of algorithm on the persistence tree and on the map |
| ----- |
| ![Сравнение-графиков-подготовки-алгоритма-на-персистентном-дереве-и-на-карте](https://raw.githubusercontent.com/khadievedem/algorithms-lab-2/main/graphics/%D0%A1%D1%80%D0%B0%D0%B2%D0%BD%D0%B5%D0%BD%D0%B8%D0%B5%20%D0%B3%D1%80%D0%B0%D1%84%D0%B8%D0%BA%D0%BE%D0%B2%20%D0%BF%D0%BE%D0%B4%D0%B3%D0%BE%D1%82%D0%BE%D0%B2%D0%BA%D0%B8%20%D0%B0%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC%D0%B0%20%D0%BD%D0%B0%20%D0%BF%D0%B5%D1%80%D1%81%D0%B8%D1%81%D1%82%D0%B5%D0%BD%D1%82%D0%BD%D0%BE%D0%BC%20%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%B5%20%D0%B8%20%D0%BD%D0%B0%20%D0%BA%D0%B0%D1%80%D1%82%D0%B5.png) |

| Average time complexity of algorithms |
| ----- |
| ![Время-поиска-ответа-на-один-запрос-(сред.)](https://raw.githubusercontent.com/khadievedem/algorithms-lab-2/main/graphics/%D0%92%D1%80%D0%B5%D0%BC%D1%8F%20%D0%BF%D0%BE%D0%B8%D1%81%D0%BA%D0%B0%20%D0%BE%D1%82%D0%B2%D0%B5%D1%82%D0%B0%20%D0%BD%D0%B0%20%D0%BE%D0%B4%D0%B8%D0%BD%20%D0%B7%D0%B0%D0%BF%D1%80%D0%BE%D1%81%20(%D1%81%D1%80).png) |

## Conclusion
The results showed that the map algorithm was the slowest in the preparation phase, taking O(n^3) to build the 2D map on compressed coordinates. On the other hand, the brute force algorithm had a complexity of O(1) in preparation (because doesn't have it :) ), while the persistent tree algorithm prepared itself in O(n log(n)). The persistent tree and the algorithm on the map did not overlap in terms of data preparation complexity, which tells us that the algorithm on the map takes much longer to prepare the data.

The graph indicates that the brute force algorithm has a consistent time complexity of O(n) for data queries, whereas the other two algorithms have time complexities of around O(log(n)). Based on this, we can infer that all three algorithms perform equally well in responding to requests until the number of rectangles reaches approximately 500.

In my analysis, I found that the brute force algorithm may be the most efficient for smaller datasets, while the persistent tree and algorithm on map may be more efficient for larger datasets.
