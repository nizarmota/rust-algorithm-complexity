use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::time::Instant;
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn main() -> io::Result<()> {
    // Abrir o arquivo
    let file = File::open("src/entrada.txt")?;
    let reader = BufReader::new(file);

    // Ler as linhas do arquivo
    let mut lines = reader.lines();

    // Primeira linha: tamanho do vetor
    let size = lines.next().unwrap()?.parse::<usize>().unwrap();

    // Criar o vetor
    let mut vec = vec![0; size]; // Inicializa o vetor com zeros

    // Preencher o vetor com valores aleatórios (ou conforme necessário)
    for i in 0..size {
        vec[i] = rand::random::<i32>() % 100; // Números aleatórios entre 0 e 99
    }
    println!("{:?}", vec);

    // Segunda linha: tipo de ordenação
    let sort_type = lines.next().unwrap()?;

    //tempo inicial
    let start = Instant::now();
    //tempo final

    // Escolher a função de ordenação baseada na entrada e ordenar o vetor
    match sort_type.as_str() {
        "quick_sort" => quick_sort(&mut vec),
        "selection_sort" => selection_sort(&mut vec),
        "gnome_sort" => gnome_sort(&mut vec),
        _ => println!("Método de ordenação não reconhecido"),
    }
    let end = Instant::now();
    // Exibir o vetor ordenado
    //println!("{:?}", vec);
   // println!("Tempo: {:?}", end - start);




    criterion_group!(benches, benchmark_sorts);
    criterion_main!(benches);
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


// Critério para comparar as performances de diferentes funções de ordenação
fn benchmark_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    let mut vec = (0..1000).map(|_| rand::random::<i32>() % 1000).collect::<Vec<_>>();
    group.bench_function("Quick Sort", |b| b.iter(|| quick_sort(black_box(&mut vec.clone()))));
    group.bench_function("Selection Sort", |b| b.iter(|| selection_sort(black_box(&mut vec.clone()))));
    group.bench_function("Gnome Sort", |b| b.iter(|| gnome_sort(black_box(&mut vec.clone()))));
    group.finish();
}
