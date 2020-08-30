
use crate::mat::Matrix;

#[test]
fn create_empty_matrix() {
	let m = Matrix::empty();
	assert_eq!(m.values, [0.0; 16]);
}

#[test]
fn create_fill_matrix() {
	let m = Matrix::fill(2.2);
	assert_eq!(m.values, [2.2; 16]);
}

#[test]
fn create_ident_matrix() {
	let m = Matrix::identity();
	assert_eq!(m.values, [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn create_from_matrix() {
	let m = Matrix::from([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]);
	assert_eq!(m.values, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]);
}

#[test]
fn matrix_add() {
	let m = Matrix::fill(3.2);
	let m2 = Matrix::identity();
	let res = Matrix::from([4.2, 3.2, 3.2, 3.2, 3.2, 4.2, 3.2, 3.2, 3.2, 3.2, 4.2, 3.2, 3.2, 3.2, 3.2, 4.2]);
	assert_eq!(m + m2, res);
}

#[test]
fn matrix_sub() {
	let m = Matrix::identity();
	let m2 = Matrix::fill(3.2);
	let res = Matrix::from([-2.2, -3.2, -3.2, -3.2, -3.2, -2.2, -3.2, -3.2, -3.2, -3.2, -2.2, -3.2, -3.2, -3.2, -3.2, -2.2]);
	assert_eq!(m - m2, res);
}

#[test]
fn matrix_mult() {
	let m = Matrix::fill(3.2);
	let m2 = Matrix::identity() + Matrix::fill(1.0);
	let res = Matrix::from([6.4, 3.2, 3.2, 3.2, 3.2, 6.4, 3.2, 3.2, 3.2, 3.2, 6.4, 3.2, 3.2, 3.2, 3.2, 6.4]);
	assert_eq!(m * m2, res);
}

#[test]
fn matrix_div() {
	let m = Matrix::fill(3.2);
	let m2 = Matrix::identity() + Matrix::fill(1.0);
	let res = Matrix::from([1.6, 3.2, 3.2, 3.2, 3.2, 1.6, 3.2, 3.2, 3.2, 3.2, 1.6, 3.2, 3.2, 3.2, 3.2, 1.6]);
	assert_eq!(m / m2, res);
}