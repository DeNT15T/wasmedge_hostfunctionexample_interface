use wasmedge_hostfunctionmatrix_interface::*;

fn main() {
    let mut index: u32 = 0; // mock user input data
    print(index);
    let mut res: f64 = determinant(index);
    println!("Determinant of the No.{:.?} matrix is: {:.?}", index, res);
    println!();
    index = 1;
    print(index);
    res = determinant(index);
    println!("Determinant of the No.{:.?} matrix is: {:.?}", index, res);
    println!();
    index = 2;
    print(index);
    res = determinant(index);
    println!("Determinant of the No.{:.?} matrix is: {:.?}", index, res);
    println!();
    println!("after pushing one matrix, the matrix queue have {} matrices", input("11 23 5 -5 123 4 9 88 7 "));
    index = 3;
    print(index);
    res = determinant(index);
    println!("Determinant of the No.{:.?} matrix is: {:.?}", index, res);
    println!();
    println!("after pushing one matrix, the matrix queue have {} matrices", input("1 3 5 -5 12 4 9 8 7 "));
    index = 4;
    print(index);
    res = determinant(index);
    println!("Determinant of the No.{:.?} matrix is: {:.?}", index, res);
}
