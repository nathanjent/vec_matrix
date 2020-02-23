use vec_matrix::IntoVecMatrix;

#[test]
fn into_vec_matrix_test() {
    let m1 = vec![1, 2, 3, 4].into_vec_matrix(2);
    let m2 = [1, 2, 3, 4].into_vec_matrix(2);
    assert_eq!(m1, m2);
}

#[test]
fn add_matrix_test() {
    let m1 = [1, 0, 0, 1].into_vec_matrix(2);
    let m2 = [0, 1, 1, 0].into_vec_matrix(2);
    assert_eq!(m1 + m2, [1, 1, 1, 1].into_vec_matrix(2),);
}

#[test]
fn sub_matrix_test() {
    let m1 = [1, 0, 0, 1].into_vec_matrix(2);
    let m2 = [0, 1, 1, 0].into_vec_matrix(2);
    assert_eq!(m1 - m2, [1, -1, -1, 1].into_vec_matrix(2),);
}

#[test]
fn add_value_test() {
    let m1 = [1, 0, 0, 1].into_vec_matrix(2);
    assert_eq!([2, 1, 1, 2].into_vec_matrix(2), m1 + 1);
}

#[test]
fn sub_value_test() {
    let m1 = [1, 0, 0, 1].into_vec_matrix(2);
    assert_eq!([0, -1, -1, 0].into_vec_matrix(2), m1 - 1);
}

#[test]
fn mul_value_test() {
    let m1 = [0, 1, 2, 3].into_vec_matrix(2);
    assert_eq!([0, 2, 4, 6].into_vec_matrix(2), m1 * 2);
}

#[test]
fn div_value_test() {
    let m1 = [0, 1, 2, 3].into_vec_matrix(2);
    assert_eq!([0, 0, 1, 1].into_vec_matrix(2), m1 / 2);
}

#[test]
fn add_assign_matrix_test() {
    let mut m1 = [1, 0, 0, 1].into_vec_matrix(2);
    let m2 = [0, 1, 1, 0].into_vec_matrix(2);
    m1 += m2;
    assert_eq!([1, 1, 1, 1].into_vec_matrix(2), m1);
}
