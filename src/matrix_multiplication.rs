use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct ThreadReturn {
    row: usize,
    column: usize,
    value: f64,
}

#[allow(dead_code)]
fn row_col_mult(
    mtx1: Arc<Mutex<Vec<Vec<f64>>>>,
    mtx2: Arc<Mutex<Vec<Vec<f64>>>>,
    row: usize,
    column: usize,
) -> f64 {
    let mut result: f64 = 0f64;
    let mtx1 = mtx1.lock().unwrap();
    let mtx2 = mtx2.lock().unwrap();
    for i in 0..mtx1.len() {
        result += mtx1[row][i] * mtx2[i][column];
        println!("[{}{}]", row, column);
    }
    result
}
#[allow(unused_must_use)]
#[allow(dead_code)]
fn thread_mult(mtx1: &Vec<Vec<f64>>, mtx2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let r = mtx1.len();
    let c = mtx2[0].len();
    let mtx1 = mtx1.clone();
    let mtx2 = mtx2.clone();
    if mtx1[0].len() != mtx2.len() {
        // mxt1 column number must be equal to mtx2 line number
        panic!("Matrize com dimensões não compatíveis");
    }

    let mut result: Vec<Vec<f64>> = vec![vec![0f64; mtx2[0].len()]; mtx1.len()];
    let mut handles = vec![];
    let (sender, receiver) = mpsc::channel::<ThreadReturn>();

    let t_mtx1 = Arc::new(Mutex::new(mtx1));
    let t_mtx2 = Arc::new(Mutex::new(mtx2));

    /*for _ in 0..4 {
        let t_mtx1 = Arc::clone(&t_mtx1);
        let t_mtx2 = Arc::clone(&t_mtx2);
        thread::spawn(move || {
            println!("MTX {:?}", t_mtx1);
        });
    }*/
    for row in 0..r {
        for column in 0..c {
            let sender = sender.clone();
            let t_mtx1 = Arc::clone(&t_mtx1);
            let t_mtx2 = Arc::clone(&t_mtx2);
            handles.push(thread::spawn(move || {
                let value = row_col_mult(t_mtx1, t_mtx2, row, column);
                sender.send(ThreadReturn { row, column, value })
            }));
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..r*c {
        let received = receiver.recv().unwrap();
        println!("Received: {} from [{},{}]", received.value, received.row, received.column);
        result[received.row][received.column] = received.value;
    }
    /*for received in receiver {
        println!("Received: {} from [{},{}]", received.value, received.row, received.column);
        result[received.row][received.column] = received.value;
    }*/
    result
}


pub fn run() {
    let m1 = vec![
        vec![1.0, 2.0, 5.0],
        vec![3.0, 4.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let m2 = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let r = thread_mult(&m1, &m2);
    assert_eq!(
        vec![
            vec![1.0, 2.0, 5.0],
            vec![3.0, 4.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ],
        r
    );
}




#[allow(dead_code)]
fn simple_mult(mtx1: &Vec<Vec<f64>>, mtx2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if mtx1[0].len() != mtx2.len() {
        // mxt1 column number must be equal to mtx2 line number
        panic!("Matrize com dimensões não compatíveis");
    }
    let mut result: Vec<Vec<f64>> = vec![vec![0f64; mtx2[0].len()]; mtx1.len()];
    for i in 0..mtx1.len() {
        for j in 0..mtx2[0].len() {
            for k in 0..mtx1.len() {
                result[i][j] += mtx1[i][k] * mtx2[k][j];
            }
        }
    }
    result
}

#[test]
fn simple_mult_test() {
    let m1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let m2 = vec![vec![1.0], vec![2.0]];
    let r = simple_mult(&m1, &m2);
    assert_eq!(vec![vec![5.0], vec![11.0]], r);
    let t = thread_mult(&m1, &m2);
    assert_eq!(vec![vec![5.0], vec![11.0]], t);

    let m1 = vec![
        vec![1.0, 2.0, 5.0],
        vec![3.0, 4.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let m2 = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let r = simple_mult(&m1, &m2);
    assert_eq!(
        vec![
            vec![1.0, 2.0, 5.0],
            vec![3.0, 4.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ],
        r
    );

    let t = thread_mult(&m1, &m2);
    assert_eq!(
        vec![
            vec![1.0, 2.0, 5.0],
            vec![3.0, 4.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ],
        t
    );
}
