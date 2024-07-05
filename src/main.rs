use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::time::Instant;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn main() -> io::Result<()> {
    let file = File::open("src/entrada.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let size = lines.next().unwrap()?.parse::<usize>().unwrap();
    let mut vec = vec![0; size];
    for i in 0..size {
        vec[i] = rand::random::<i32>() % 100000;
    }
    println!("{:?}", vec);
    let sort_type = lines.next().unwrap()?;
    let start = Instant::now();
    match sort_type.as_str() {
        "quick_sort" => quick_sort(&mut vec),
        "selection_sort" => selection_sort(&mut vec),
        "gnome_sort" => gnome_sort(&mut vec),
        _ => println!("Método de ordenação não reconhecido"),
    }
    let end = Instant::now();
    println!("{:?}", vec);
    println!("Tempo: {:?}", end - start);
    criterion_group!(benches, benchmark_sorts);
    criterion_main!(benches);
    println!("Pressione Enter para sair...");
    io::stdout().flush().unwrap(); // Garante que "Pressione Enter para sair..." seja mostrado antes da entrada
    io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn gnome_sort(arr: &mut [i32]) {
    let mut index = 0;
    while index < arr.len() {
        if index == 0 || arr[index] >= arr[index - 1] {
            index += 1;
        } else {
            arr.swap(index, index - 1);
            index -= 1;
        }
    }
}

fn benchmark_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    let sizes = [10, 100, 1000, 10000];
    for size in sizes.iter() {
        let mut vec = (0..*size).map(|_| rand::random::<i32>() % 1000).collect::<Vec<_>>();
        group.bench_with_input(BenchmarkId::new("Quick Sort", size), &vec, |b, v| {
            b.iter(|| quick_sort(black_box(&mut v.clone())))
        });
        group.bench_with_input(BenchmarkId::new("Selection Sort", size), &vec, |b, v| {
            b.iter(|| selection_sort(black_box(&mut v.clone())))
        });
        group.bench_with_input(BenchmarkId::new("Gnome Sort", size), &vec, |b, v| {
            b.iter(|| gnome_sort(black_box(&mut v.clone())))
        });
    }
    group.finish();
}
