use ndarray::{arr1, arr2, Array1, Array};

fn main() {
    sum_arrays();
    multiply_scalar_vector();
    vector_comparison();
}

fn sum_arrays() -> () {
    let a = arr2(&[[1, 2, 3],
        [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4],
            [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);
}

fn multiply_scalar_vector() -> () {
    let scalar = 4;

    let vector = arr1(&[1, 2, 3]);

    let matrix = arr2(&[[4, 5, 6],
                        [7, 8, 9]]);

    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}

fn vector_comparison() -> () {
    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);
  
    let _z = a + b;
    let _w =  &c + &d;
    
    println!("c = {}", c);
    c[0] = 10.;
    d[1] = 10.;
  
}