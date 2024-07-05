use std::fs::File; // Importa o módulo File para manipulação de arquivos
use std::io::{self, BufReader, BufRead}; // Importa módulos para leitura de buffer e entrada/saída
use std::time::Instant; // Importa o módulo Instant para medir tempo
use criterion::{black_box, criterion_group, criterion_main, Criterion}; // Importa o módulo criterion para benchmarking

fn main() -> io::Result<()> {
    // Abrir o arquivo
    let file = File::open("src/entrada.txt")?; // Abre o arquivo "src/entrada.txt"
    let reader = BufReader::new(file); // Cria um leitor de buffer para o arquivo

    // Ler as linhas do arquivo
    let mut lines = reader.lines(); // Lê as linhas do arquivo

    // Primeira linha: tamanho do vetor
    let size = lines.next().unwrap()?.parse::<usize>().unwrap(); // Lê e parseia a primeira linha como o tamanho do vetor

    // Criar o vetor
    let mut vec = vec![0; size]; // Inicializa o vetor com zeros do tamanho especificado

    // Preencher o vetor com valores aleatórios (ou conforme necessário)
    for i in 0..size {
        vec[i] = rand::random::<i32>() % 100000; // Preenche o vetor com números aleatórios entre 0 e 99999
    }
    println!("{:?}", vec); // Imprime o vetor gerado

    // Segunda linha: tipo de ordenação
    let sort_type = lines.next().unwrap()?; // Lê a segunda linha que define o tipo de ordenação

    // Tempo inicial
    let start = Instant::now(); // Marca o tempo de início

    // Escolher a função de ordenação baseada na entrada e ordenar o vetor
    match sort_type.as_str() {
        "quick_sort" => quick_sort(&mut vec), // Chama quick_sort se a entrada for "quick_sort"
        "selection_sort" => selection_sort(&mut vec), // Chama selection_sort se a entrada for "selection_sort"
        "gnome_sort" => gnome_sort(&mut vec), // Chama gnome_sort se a entrada for "gnome_sort"
        _ => println!("Método de ordenação não reconhecido"), // Mensagem de erro para métodos de ordenação não reconhecidos
    }
    let end = Instant::now(); // Marca o tempo de fim

    // Exibir o vetor ordenado
    println!("{:?}", vec); // Imprime o vetor ordenado
    println!("Tempo: {:?}", end - start); // Imprime o tempo de execução

    // Cria um grupo de benchmarks e executa
    criterion_group!(benches, benchmark_sorts);
    criterion_main!(benches);
    Ok(()) // Retorna Ok para indicar sucesso
}

// Função para quicksort
fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 { // Se o vetor tiver 1 ou 0 elementos, está ordenado
        return;
    }
    let pivot_index = partition(arr); // Particiona o vetor
    quick_sort(&mut arr[0..pivot_index]); // Ordena recursivamente a primeira parte
    quick_sort(&mut arr[pivot_index + 1..]); // Ordena recursivamente a segunda parte
}

// Função para particionar o vetor no quicksort
fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() / 2; // Escolhe o pivô no meio do vetor
    arr.swap(pivot_index, arr.len() - 1); // Move o pivô para o final
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] { // Se o elemento atual for menor ou igual ao pivô
            arr.swap(i, j); // Troca os elementos
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1); // Coloca o pivô na posição correta
    i
}

// Função para selection sort
fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] { // Encontra o menor elemento na sublista não ordenada
                min_index = j;
            }
        }
        arr.swap(i, min_index); // Troca o menor elemento encontrado com o primeiro elemento não ordenado
    }
}

// Função para gnome sort
fn gnome_sort(arr: &mut [i32]) {
    let mut index = 0;
    while index < arr.len() {
        if index == 0 || arr[index] >= arr[index - 1] { // Se estiver na posição inicial ou o elemento atual for maior ou igual ao anterior
            index += 1; // Avança para o próximo elemento
        } else {
            arr.swap(index, index - 1); // Troca os elementos
            index -= 1; // Retrocede para comparar novamente
        }
    }
}

// Função de benchmark para comparar diferentes algoritmos de ordenação
fn benchmark_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms"); // Cria um grupo de benchmarks
    let mut vec = (0..1000).map(|_| rand::random::<i32>() % 1000).collect::<Vec<_>>(); // Gera um vetor aleatório para o benchmark
    group.bench_function("Quick Sort", |b| b.iter(|| quick_sort(black_box(&mut vec.clone())))); // Benchmark para quicksort
    group.bench_function("Selection Sort", |b| b.iter(|| selection_sort(black_box(&mut vec.clone())))); // Benchmark para selection sort
    group.bench_function("Gnome Sort", |b| b.iter(|| gnome_sort(black_box(&mut vec.clone())))); // Benchmark para gnome sort
    group.finish(); // Finaliza o grupo de benchmarks
}
