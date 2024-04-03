use super::types::{Dataset, Result};

pub trait ClusteringMethod {
    fn fit(&mut self, dataset: &Dataset) -> ();

    fn transform(self, dataset: &Dataset) -> Option<Result>;
}
