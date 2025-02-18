fn main() {
    // Contoh matriks 3x3
    let matrix = vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0],
        vec![5.0, 6.0, 0.0],
    ];

    match inverse(&matrix) {
        Some(result) => {
            println!("Matriks original:");
            print_matrix(&matrix);
            println!("\nMatriks invers:");
            print_matrix(&result);
        }
        None => println!("Matriks tidak memiliki invers (determinan = 0)"),
    }
}

fn inverse(matrix: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = matrix.len();
    
    // Pastikan matriks adalah matriks persegi
    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return None;
    }

    // Buat matriks augmented [A|I]
    let mut augmented = vec![vec![0.0; 2 * n]; n];
    for i in 0..n {
        for j in 0..n {
            augmented[i][j] = matrix[i][j];
        }
        augmented[i][n + i] = 1.0;
    }

    // Eliminasi Gauss-Jordan
    for i in 0..n {
        // Cari pivot non-zero
        let mut pivot = augmented[i][i];
        let mut pivot_row = i;
        
        for j in i + 1..n {
            if augmented[j][i].abs() > pivot.abs() {
                pivot = augmented[j][i];
                pivot_row = j;
            }
        }

        // Jika tidak ada pivot non-zero, matriks singular
        if pivot.abs() < 1e-10 {
            return None;
        }

        // Tukar baris jika perlu
        if pivot_row != i {
            augmented.swap(i, pivot_row);
        }

        // Normalisasi baris pivot
        for j in 0..2 * n {
            augmented[i][j] /= pivot;
        }

        // Eliminasi kolom
        for j in 0..n {
            if i != j {
                let factor = augmented[j][i];
                for k in 0..2 * n {
                    augmented[j][k] -= factor * augmented[i][k];
                }
            }
        }
    }

    // Ekstrak bagian invers
    let mut result = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            result[i][j] = augmented[i][n + j];
        }
    }

    Some(result)
}

fn print_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        for &val in row {
            print!("{:8.4} ", val);
        }
        println!();
    }
}