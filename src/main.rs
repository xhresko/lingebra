use lingebra::Matrix;


fn main() {
    let vec_a = vec![0.0, 1.0, 2.0, 3.0];
    let vec_b = vec![1.0, 0.0, 1.0, 0.0];
    let vec_c = vec![5.0, 5.0, 5.0, 5.0];

    let matrix = lingebra::Matrix::new(vec![vec_a, vec_b, vec_c]);

    println!("{}", &matrix);
}
