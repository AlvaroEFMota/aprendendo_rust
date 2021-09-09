#[allow(dead_code)]
fn simple_mult(mtx1: &Vec<Vec<f64>>, mtx2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    if mtx1[0].len() != mtx2.len() { // mxt1 column number must be equal to mtx2 line number
        panic!("Matrize com dimensões não compatíveis");
    }
    let mut result: Vec<Vec<f64>> = vec![vec![0f64; mtx2[0].len()]; mtx1.len()];
    for i in 0..mtx1.len() {
        for j in 0..mtx2[0].len() {
            for k in 0..mtx1.len() {
                result[i][j] += mtx1[i][k]*mtx2[k][j];
            }
        }
    }
    result
}

pub fn run() {
    println!("matrix_multiplication");
}

#[test]
fn simple_mult_test() {
    let m1 = vec![vec![1.0,2.0],vec![3.0,4.0]];
    let m2 = vec![vec![1.0],vec![2.0]];
    let r = simple_mult(&m1, &m2);
    assert_eq!(vec![vec![5.0],vec![11.0]],r);
    
    let m1 = vec![vec![1.0, 2.0, 5.0], vec![3.0, 4.0, 6.0], vec![7.0, 8.0, 9.0]];
    let m2 = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
    let r = simple_mult(&m1, &m2);
    assert_eq!(vec![vec![1.0, 2.0, 5.0], vec![3.0, 4.0, 6.0], vec![7.0, 8.0, 9.0]],r);
}