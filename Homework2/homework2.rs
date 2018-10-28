use std::{ops, fmt};


#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        Matrix{
            data: values.to_owned(), //converts from borrowed to owned
            row: row,
            col: col,
        }
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        Matrix{
            data: Vec::new(),  //creats new empty vector   data : &'a mut Vec<T>
            row: row,
            col: col,
        }
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)

    }
}

impl<T: ops::Add<Output = T> + Copy + Default> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.row == rhs.row); //states the fact that self row = rhs row
        assert!(self.col == rhs.col); //states the fact that self col = rhs col

        let mut res = Matrix::new_empty(self.row, self.col);

        for i in 0..self.row*self.col{
            res.data.push(self.data[i] + rhs.data[i]);  //pushes the sub of the two numbers into new data of matrix
        }
        res
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.row == rhs.row); //states the fact that self row = rhs row
        assert!(self.col == rhs.col); //states the fact that self col = rhs col

        let mut res = Matrix::new_empty(self.row, self.col);

        for i in 0..self.row*self.col{
            res.data.push(self.data[i] - rhs.data[i]);  //pushes the sub of the two numbers into new data of matrix
        }
        res
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.col == rhs.row); //states the fact that self row = rhs row
//        assert!(self.col == rhs.col); //states the fact that self col = rhs col

        let mut res = Matrix::new_empty(self.row, self.col);
        res.row = self.row;
        res.col = rhs.col;
        let mut temporary: T;

        for i in 0..self.row {
            for j in 0..rhs.col {
                temporary=Default::default();
            }
                for k in rhs.row {
                    temporary = temporary + self.data[i * self.col * k] * rhs.data[k + rhs.col + j];
                }
            res.data.push(temporary);
        }

        
/*
        for i in 0..self.row*self.col{
            res.data.push(self.data[i] * rhs.data[i]);  //pushes the mul of the two numbers into new data of matrix
        }
*/
        res

    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {


        let mut rownumber = 0;

        for i in 0..self.row*self.col {

            if i != (self.col + rownumber) - 1 {
                write!(f, "{} ", self.data[i]);
            }
            else if i == (self.col + rownumber) - 1 {
                write!(f, "{}\n", self.data[i]);
                rownumber = self.col;
            }
        }
        write!(f, "{}", "")
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    let x = Matrix::new(1, 3, &[1, 4, 6]);
    let y = Matrix::new(3, 2, &[2, 3, ,5 ,8 ,7 ,9]);
    let z = Matrix::new(1, 2, &[64, 89]);
    assert_eq!(x * y, z);
    assert_eq!(format!("{}", z), "63 89\n");
/*
    fn it_works() {
        let x = Matrix::new(3, 3 &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(3, 3 &[1, 2, 3, 2, 3, 1]);
        let z = Matrix::new(3, 3 &[-1, 1, 3, 3, 5, 4]);

        assert_eq!(x + y, z);
        assert_eq!(format!("{}", z), "-1 1\n 3 3\n 5 4\n");
*/
/*
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
//        let y = Matrix::new(2, 3, &[0, 0, 0, 0, 0, 0]);
        let y = Matrix::new(2, 3, &[2, 1, 3, 1, 2, 3]);
//        let z = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let z = Matrix::new(2, 3, &[-4, -1, 0, 1, 4, 9]);
        assert_eq!(x * y, z);
        assert_eq!(format!("{}", z), "-4 -1 0\n1 4 9\n");
//        assert_eq!(format!("{}", z), "-2 -1 0\n1 2 3\n");
*/
    }
}
