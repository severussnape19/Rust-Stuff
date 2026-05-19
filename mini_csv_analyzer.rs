#![allow(unused)]
#![allow(dead_code)]

use std::collections::{ BinaryHeap, HashMap, VecDeque, binary_heap };

#[derive(Eq, PartialEq, Debug, Clone)]
struct Item {
    data: String,
    count: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn word_frequency_analyzer(string: &str) -> (Vec<(Item)>, Vec<String>, usize) {
    // Count frequency of each word
    // Return top N most frequent words in order
    // Return words that appear exactly once
    // Return total unique words

    // Returns a string
    let data: String = string.trim().to_lowercase().replace(['.', ';', '!', '?', '\'', '\"', ':', '/'], "");
    let data: Vec<&str> = data.split_whitespace().collect(); // Rturns Vec<&str>
    let mut data_map: HashMap<&str, usize> = HashMap::with_capacity(data.len());
    for word in data {
        data_map.entry(word).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut bin_heap: BinaryHeap<Item> = data_map
    .iter()
    .map(|(word, count)| Item {
        data: word.to_string(),
        count: *count,
    })
    .collect();

    let mut common_words: Vec<Item> = Vec::with_capacity(5);
    for _ in 0..5 {
        if let Some(item) = bin_heap.pop() {
            common_words.push(item)
        }
    }

    let mut appear_once: Vec<String> = Vec::new();
    for d in &bin_heap {
        if d.count == 1 {
            appear_once.push(d.data.clone());
        }
    }

    (common_words, appear_once, data_map.len())
}

struct Dataframe {
    dim: (usize, usize),
    data: Vec<Vec<String>>,
    dtype_vec: Vec<DataType>,
}

#[derive(Debug)]
enum DataType {
    Int,
    Float,
    Text
}

impl Dataframe {
    fn new() -> Self {
        Dataframe {
            dim: (0, 0),
            data: Vec::new(),
            dtype_vec: Vec::new(),
        }
    }

    fn from(csv_data: String) -> Self {
        let mut data = Self::new();
        data.df_from(csv_data);
        data
    }

    fn df_from(&mut self, csv_data: String) {
        let raw_data: Vec<&str> = csv_data.split('\n').collect();
        let mut dataset: Vec<Vec<String>> = Vec::new();
        for d_block in raw_data {
            dataset.push(d_block.trim().split(',').map(String::from).collect());
        }

        for i in 0..dataset.len() {
            if dataset[i].len() != dataset[0].len() {
                if dataset[i].len() > dataset[0].len() {
                    while dataset[i].len() != dataset[0].len() {
                        dataset[i].pop();
                    }
                    self.data.push(dataset[i].clone());
                } else {
                    while dataset[i].len() != dataset[0].len() {
                        dataset[i].push("".to_string());
                    }
                    self.data.push(dataset[i].clone())
                }
            } else {
                self.data.push(dataset[i].clone());
            }
        }

        for i in 1..self.data.len() {
            if self.data[i].contains(&"".to_string()) {
                continue;
            }
            if self.dtype_vec.len() == self.data[0].len() {
                break;
            }
            for data in &self.data[i] {
                if let Ok(val) = data.parse::<i32>() {
                    self.dtype_vec.push(DataType::Int)
                } else if let Ok(val) = data.parse::<f64>() {
                    self.dtype_vec.push(DataType::Float)
                } else {
                    self.dtype_vec.push(DataType::Text)
                }
            }
        }
        self.dim.0 = self.data.len();
        self.dim.1 = self.data[0].len();
    }

    fn info(&self) {
        println!("Column names: {:?}", self.data[0]); //  Assuming column names exist
        println!("Row Count: {}", self.data.len() - 1);
    }

    // sum, mean, min, max
    fn describe_col(&self, index: usize) /*-> Result<(String, String, String, String), String> */ {
        if index >= self.dtype_vec.len() {
            // Err("Column Does Not Exist!".to_string())
            println!("Column does not exist");
        } else {
            match self.dtype_vec[index] {
                DataType::Int => {
                    let mut sum: i32 = 0;
                    let mut max: i32 = i32::MIN;
                    let mut min: i32 = i32::MAX;
                    for i in 1..self.data.len() {
                        let parsed = self.data[i][index].parse::<i32>().ok();
                        if let Some(val) = parsed {
                            sum += val;
                            if val > max { max = val }
                            if val < min { min = val }
                        }
                    }
                    let mean =  sum as f64 / (self.data.len() - 1) as f64;
                    println!("Sum: {}\nMax: {}\nMin: {}\nMean: {:.2}",
                        sum, max, min, mean)
                },
                DataType::Float => {
                    let mut sum: f64 = 0.0;
                    let mut max: f64 = f64::MIN;
                    let mut min: f64 = f64::MAX;
                    for i in 1..self.data.len() {
                        let parsed = self.data[i][index].parse::<f64>().ok();
                        if let Some(val) = parsed {
                            sum += val;
                            if val > max { max = val }
                            if val < min { min = val }
                        }
                    }
                    let mean =  sum / (self.data.len() - 1) as f64;
                    println!("Sum: {}\nMax: {}\nMin: {}\nMean: {:.2}",
                        sum, max, min, mean)
                },
                DataType::Text => {
                    let mut empty_entries: Vec<usize> = Vec::new();
                    let mut smallest_string = "".to_string();
                    let mut longest_string: String = String::new();
                    let mut map: HashMap<String, usize> = HashMap::new();
                    for i in 1..self.data.len() {
                        if self.data[i][index].is_empty() {
                            empty_entries.push(i)
                        } else {
                            if self.data[i][index].len() > longest_string.len() {
                                longest_string = self.data[i][index].clone()
                            }
                            if smallest_string.is_empty() || self.data[i][index].len() < smallest_string.len() {
                                smallest_string = self.data[i][index].clone()
                            }

                            map.entry(self.data[i][index].clone().to_lowercase())
                               .and_modify(|v| *v += 1)
                               .or_insert(0);
                        }
                    }
                    let mut bin_heap: BinaryHeap<Item> = map.iter()
                        .map(|(string, num)| Item {
                        data: string.clone(),
                        count: *num,
                        })
                    .collect();

                    println!("Strings appearing more than once in {} column: ", self.data[0][index]);
                    let mut more_than_one_appear = 0;
                    while let Some(data) = bin_heap.pop() {
                        if data.count > 1 {
                            more_than_one_appear += 1;
                            print!("{}: {}, ", data.data, data.count);
                        } else {
                            break;
                        }
                    }

                    if more_than_one_appear == 0 {
                        println!("NONE!");
                    }

                    println!("\nLongest: {}\nShortest: {}\nEmpty entries indices: {:?}\nEmpty entries: {}\nUnique Entries: {}",
                        longest_string, smallest_string, empty_entries,
                        empty_entries.len(), map.len()
                        );
                }
            }
        }
    }

    fn sum(&self, index: usize) -> Option<f64> {
        match self.dtype_vec[index] {
            DataType::Int | DataType::Float => {
                self.data.iter() // Iterates over rows
                    .skip(1) // skips first row
                    .filter_map(|row| row.get(index)) // gets element from row -> Option<&String>
                    .filter_map(|val| val.parse::<f64>().ok()) //
                    .reduce(|a, b| a + b)
            },
            _ => None
        }
    }

    fn max(&self, index: usize) -> Option<f64> {
        match self.dtype_vec[index] {
            DataType::Float | DataType::Int => {
                self.data.iter()
                    .skip(1)
                    .filter_map(|row| row.get(index))
                    .filter_map(|val| val.parse::<f64>().ok())
                    .reduce(|max, cur| if max < cur { cur } else { max })
            },
            _ => None
        }
    }

    fn min(&self, index: usize) -> Option<f64> {
        match self.dtype_vec[index] {
            DataType::Float | DataType::Int => {
                self.data.iter()
                    .skip(1)
                    .filter_map(|row| row.get(index))
                    .filter_map(|val| val.parse::<f64>().ok())
                    .reduce(|min, cur| if min > cur { cur } else { min })
            },
            _ => None
        }
    }

    fn mean(&self, index: usize) -> Option<f64> {
        match self.dtype_vec[index] {
            DataType::Int | DataType::Float => {
                self.sum(index)
                    .map(|val| val / ((self.data.len() as f64) - 1.0))
            },
            _ => None
        }
    }

    fn test(&self) {
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                println!("{:?}", self.data[i][1].parse::<i32>())
            }
            println!()
        }
    }
}

impl std::fmt::Display for Dataframe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            write!(f, "|")?;
            for data in row {
                write!(f, "{:<10}|", data);
            }
            writeln!(f);
        }
        writeln!(f, "Dimensions: ({}, {})", self.dim.0, self.dim.1)
    }
}

fn main() {
    let csv_data: String = "\
    name,age,language
    man,,
    Lakshya,20,Rust
    Alice,25,Python
    Bob,30,C++
    Emma,28,Go
    David,35,Zig
    Sophia,22,Java".into();

    let csv_data = "\
id,name,age,country,occupation,salary,experience,remote,language
1,Lakshya,20,India,Student,0,2,true,Rust
2,Alice,25,USA,Backend Engineer,85000,4,true,Python
3,Bob,30,Canada,Game Developer,92000,6,false,C++
4,Emma,28,Germany,DevOps Engineer,78000,5,true,Go
5,David,35,Australia,Security Researcher,120000,10,false,Zig
6,Sophia,22,UK,Frontend Developer,65000,2,true,TypeScript
7,Noah,,France,Data Analyst,70000,3,true,R
8,Olivia,31,Japan,Mobile Developer,95000,7,false,Kotlin
9,Liam,not_a_number,Brazil,Systems Engineer,88000,5,true,C
10,Ava,27,India,,72000,4,true,Java
11,James,29,USA,Compiler Engineer,130000,8,false,Rust
12,Charlotte,18,Canada,Intern,15000,0,true,Python
13,Benjamin,45,Germany,CTO,250000,20,false,Java
14,Amelia,33,Australia,ML Engineer,145000,9,true,Julia
15,Lucas,21,UK,Student,0,1,true,C#
16,Harper,26,France,UI Designer,68000,3,false,Figma
17,Henry,39,Japan,Embedded Engineer,110000,12,false,C
18,Evelyn,,India,Researcher,99000,6,true,Haskell
19,Alexander,50,USA,Architect,180000,25,false,Scala
20,Ella,23,Brazil,QA Engineer,60000,2,true,JavaScript
21,Daniel,34,Canada,Reverse Engineer,125000,11,false,Assembly
22,Scarlett,invalid,Australia,Data Scientist,135000,7,true,Python
23,Matthew,40,Germany,Kernel Developer,150000,15,false,Rust
24,Aria,22,UK,Game Designer,58000,1,true,Unity
25,Joseph,,France,Cloud Engineer,115000,8,true,Go
".into();

    let dataset = Dataframe::from(csv_data);
    //dataset.describle();
    dataset.describe_col(2);
    let sum = dataset.sum(2);
    println!("sum: {:?}", sum)
    // println!("Data:\n{dataset}")
}
