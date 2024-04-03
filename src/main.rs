mod clustering;
use clustering::methods::{InitMethod, KMeans};

use crate::clustering::core::ClusteringMethod;

fn main() {
    let mut k_means = KMeans::new(2, InitMethod::Basic);

    k_means.fit(&vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![6.0, 7.0, 8.0]]);

    println!("{:?}", k_means.centers);
}
