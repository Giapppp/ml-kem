use crate::field::FieldElement as FF;
use crate::poly::Polynomial;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub matrix: Vec<Vec<Polynomial>>
}

impl Matrix {
    pub fn init(matrix: Vec<Vec<Polynomial>>) -> Matrix {
        Matrix { matrix }
    }

    pub fn zero_matrix(row: usize, column: usize) -> Matrix {
        Matrix::init(vec![vec![Polynomial::zero_polynomial(); column]; row])
    }

    pub fn transpose(&self) -> Matrix {
        let mut matrix = vec![vec![Polynomial::zero_polynomial(); self.matrix.len()]; self.matrix[0].len()];
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                matrix[j][i] = self.matrix[i][j].clone();
            }
        }
        Matrix::init(matrix)
    }
}

pub fn mul(a: &Matrix, b: Vec<Polynomial>) -> Vec<Polynomial> {
    let mut c = vec![Polynomial::zero_polynomial(); a.matrix.len()];
    for i in 0..a.matrix.len() {
        for j in 0..a.matrix[i].len() {
            c[i] = c[i].clone() + a.matrix[i][j].clone() * b[j].clone();
        }
    }
    c
}

pub fn add(a: Vec<Polynomial>, b: Vec<Polynomial>) -> Vec<Polynomial> {
    let mut c = vec![Polynomial::zero_polynomial(); a.len()];
    for i in 0..a.len() {
        c[i] = a[i].clone() + b[i].clone();
    }
    c
}

pub fn vec_mul(a: Vec<Polynomial>, b: Vec<Polynomial>) -> Polynomial {
    let mut c = Polynomial::zero_polynomial();
    for i in 0..a.len() {
        c = c.clone() + a[i].clone() * b[i].clone();
    }
    c
}