//use core::num;
use rand::Rng;
use nalgebra::{DVector, DMatrix};

fn main() {
    // Generate synthetic data
    let data = generate_data();

    // Initialize parameters randomly
    let num_components = 2;
    let (mut weights, mut means, mut covariances) = initialize_parameters(&data, num_components);

    // Run the EM algorithm
    let num_iterations = 100;
    for _ in 0..num_iterations {
        // Expectation step
        let responsibilities = expectation_step(&data, &weights, &means, &covariances);

        // Maximization step
        maximization_step(&data, &responsibilities, &mut weights, &mut means, &mut covariances);
    }
    // calculate the responsibilities one last time
    let responsibilities = expectation_step(&data, &weights, &means, &covariances);

    // Print the final parameters
    println!("Final weights: {:?}", weights);
    println!("Final means: {:?}", means);
    println!("Final covariances: {:?}", covariances);
    for i in 0..10{
        println!("Data: {:?}, responsibilities: {:?}.", data.row(i).to_scalar(), responsibilities.row(i).to_string());
    
    }

}

fn generate_data() -> DVector<f64> {
    let mut rng = rand::thread_rng();
    let num_samples = 1000;
    let data: DVector<f64> = DVector::from_iterator(num_samples, 
        (0..num_samples)
        .map(|_| rng.gen_range(-5.0..5.0))
    );
    //.collect()
    data
}

fn initialize_parameters(data: &DVector<f64>, num_components: usize) -> 
    (DVector<f64>, DVector<f64>, DVector<f64>) {
    
    let mut rng = rand::thread_rng();
    // Initialize weights uniformly
    let weights = DVector::from_element(num_components,
        1.0 / num_components as f64);

    // Randomly initialize means and covariances based on data statistics
    let mean_min = data.min();
    let mean_max = data.max();
    let means: DVector<f64> = DVector::from_iterator(num_components, 
        (0..num_components)
        .map(|_| rng.gen_range(mean_min..mean_max))
    );

    let covariance_min = 0.1;
    let covariance_max = 1.0;
    let covariances: DVector<f64> = DVector::from_iterator(num_components, 
        (0..num_components)
        .map(|_| rng.gen_range(covariance_min..covariance_max))
    );

    (weights, means, covariances)
}

fn expectation_step(data: &DVector<f64>, weights: &DVector<f64>, 
    means: &DVector<f64>, covariances: &DVector<f64>) -> DMatrix<f64> {
    let num_samples = data.len();
    let num_components = weights.len();

    let mut responsibilities = DMatrix::zeros(num_samples, num_components);

    for i in 0..num_samples {
        for j in 0..num_components {
            responsibilities[(i, j)] = weights[j]
                * gaussian_pdf(data[i], means[j], covariances[j])
                / (0..num_components).map(|k| weights[k]
                    * gaussian_pdf(data[i], means[k], covariances[k])).sum::<f64>();
        }
    }

    responsibilities
}

fn maximization_step(data: &DVector<f64>, responsibilities: &DMatrix<f64>, 
    weights: &mut DVector<f64>, means: &mut DVector<f64>, covariances: &mut DVector<f64>) {
    
    let num_samples = data.len();
    let num_components = weights.len();

    // Update weights
    for j in 0..num_components {
        weights[j] = responsibilities.column(j).sum() / num_samples as f64;
    }

    // Update means
    for j in 0..num_components {
        means[j] = responsibilities.column(j).dot(&data) / responsibilities.column(j).sum();
    }

    // Update covariances
    for j in 0..num_components {
        let covariance_sum = responsibilities.column(j).dot(
            &(data.map(|element| element - means[j])).map(|x| x.powi(2))) / responsibilities.column(j).sum();
        covariances[j] = covariance_sum.max(1e-6); // Ensure covariance is not too small
    }
}

fn gaussian_pdf(x: f64, mean: f64, covariance: f64) -> f64 {
    let exponent = -((x - mean).powi(2) / (2.0 * covariance));
    let coefficient = 1.0 / ((2.0 * std::f64::consts::PI * covariance).sqrt());
    coefficient * f64::exp(exponent)
}
