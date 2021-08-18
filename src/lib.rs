// cargo wont stop complaining
pub mod delta;
pub mod memory;

#[cfg(test)]
mod tests {
    use crate::{delta::{delta_init}, memory::*};
    use futures::executor::block_on;
    use easy_ml::matrices::Matrix;
    #[test]
    fn test() {
        let mut instance = delta_init();
        // first create some data to fit a curve to
        let x: Matrix<f32> = Matrix::column(
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);

        let y = x.map(|x| x.powi(2) + x.sin());
        instance.algo_train(x, y);

        // for the lmai later
        instance.memory.memory.push(Memory::Text(String::from("lime")));
        instance.memory.reference.push(Ref::Text(String::from("a green bitter fruit")));
        instance.memory.similar.push(Ref::Similar(vec![Ref::Text(String::from("lemon"))]));
    }
}


