use crate::memory::*;
use easy_ml::matrices::Matrix;

pub struct NeuralInstance {
    pub memory: MemoryInstance,
    pub weights: Matrix<f32>,
}

impl NeuralInstance {
    pub fn new() -> Self {
        let instance = Self {
            memory: MemoryInstance::new(),
            weights: Matrix::column(vec![1.0])
        };
        return instance;
    }
    pub fn algo_train(&mut self, x: Matrix<f32>, y: Matrix<f32>) {
        self.train(x, y);
    }
    pub fn train(&mut self, x: Matrix<f32>, y: Matrix<f32>) {
        let mut X = x.clone();
        X.insert_column(0, 1.0);
        X.insert_column_with(2, x.column_iter(0).map(|x| x * x));
        println!("{:?}", &X);

        self.weights = (X.transpose() * &X).inverse().unwrap() * (X.transpose() * &y);
        let predictions = &X * &self.weights;
        let errors = &y - &predictions;
        let mean_squared_error = (errors.transpose() * &errors).get(0, 0) / x.rows() as f32;

        println!("MSE: {}", mean_squared_error);
        //assert!(mean_squared_error > 0.41);
        //assert!(mean_squared_error < 0.42);
        println!("Predicted y values:\n{:?}", &predictions);
        println!("Actual y values:\n{:?}", &y);

        let test_x: Matrix<f32> = Matrix::column(vec![-3.0, -1.0, 0.5, 2.5, 13.0, 14.0]);
        let test_y = test_x.map(|x| x.powi(2) + x.sin());
        let mut test_X = test_x.clone();
        test_X.insert_column(0, 1.0);
        test_X.insert_column_with(2, test_x.column_iter(0).map(|x| x * x));

        println!("Unseen x values:\n{:?}", test_x);
        println!("Unseen y predictions:\n{:?}", &test_X * &self.weights);
        println!("Unseen y actual values:\n{:?}", test_y);
        let errors = &test_y - (&test_X * &self.weights);
        let mean_squared_error = (errors.transpose() * &errors).get(0, 0) / test_x.rows() as f32;
        println!("MSE on unseen values: {}", mean_squared_error);
        // assert!(mean_squared_error < 1.0);
        // assert!(mean_squared_error > 0.99);
    }
}

pub fn delta_init() -> NeuralInstance {
    let instance = NeuralInstance::new();
    return instance;
}