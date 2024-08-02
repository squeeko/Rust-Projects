/*
Correlation
analysis emerges as a critical technique, serving as a compass to detect the
degree of linear relationship between variables. In the rugged terrain of
datasets, it is not the number of features that guides us to our destination but
the relevance and quality of those features. This is where feature selection
becomes indispensable, acting as a sieve to separate the wheat from the
chaff, enabling machine learning algorithms to focus on the most
informative predictors.
*/

/*
Principal components analysis (PCA) is a method that is used to select several linear combinations that capture most of the variation in your data. PCA is an unsupervised approach, since it involves only a set of features
, and no associated response
. Apart from producing derived variables for use in supervised learning problems, PCA also serves as a tool for data visualization.

PCA is scale sensitive. Before PCA is performed, the variables should be centered to have mean zero. Furthermore, the results obtained also depend on whether the variables have been individually scaled. Use use_correlation_matrix parameter to standardize your variables (to mean 0 and standard deviation 1).
*/

/*
Feature selection in Rust can be performed using techniques such as
recursive feature elimination provided by crates like `smartcore`. This
method iteratively constructs models and removes the weakest feature until
the desired number of features is reached.
*/

use smartcore::decomposition::pca::*;
use smartcore::linalg::basic::matrix::DenseMatrix;

fn main() {
    // Pearson correlation coefficient calculation using Smartcore which is like "scikit-learn and numpy combined"

    let features =
        DenseMatrix::from_2d_array(&[&[1.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0.0, 0.0, 1.0]]);

    let pca = PCA::fit(&features, PCAParameters::default()).unwrap();

    println!("Principal Component Analysis:\n{:?}", pca);
}

/*
Output

Principal Component Analysis:
PCA { eigenvectors: DenseMatrix { ncols: 3, nrows: 3, values: [-0.7071067811865475, 0.7071067811865475, 0.0, -0.40824829046386285, -0.40824829046386274, 0.816496580927726, -0.5773502691896257, -0.5773502691896255, -0.5773502691896256], column_major: true }, eigenvalues: [0.33333333333333337, 0.3333333333333333, -6.938893903907228e-17], projection: DenseMatrix { ncols: 2, nrows: 3, values: [-0.7071067811865475, -0.40824829046386285, 0.7071067811865475, -0.40824829046386274, 0.0, 0.816496580927726], column_major: false }, mu: [0.3333333333333333, 0.3333333333333333, 0.3333333333333333], pmu: [0.0, 1.6653345369377348e-16] }
*/
