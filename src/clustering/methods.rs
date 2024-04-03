use rand::seq::SliceRandom;

use super::core::ClusteringMethod;
use super::ops::{find_closest_point, get_mean};
use super::types::{Dataset, Result};
use itertools::{self, Itertools};

pub enum InitMethod {
    Basic,
}

pub struct KMeans {
    pub n_clusters: i32,
    pub init_method: InitMethod,
    pub centers: Option<Dataset>,
}

impl KMeans {
    pub fn new(n_clusters: i32, init_method: InitMethod) -> Self {
        KMeans {
            n_clusters,
            init_method,
            centers: None,
        }
    }
}

impl ClusteringMethod for KMeans {
    fn fit(&mut self, dataset: &Dataset) -> () {
        let mut centers = match self.init_method {
            InitMethod::Basic => {
                let mut points = vec![];
                let indexes: Vec<_> = (0..dataset.len()).collect();
                (0..self.n_clusters).for_each(|_| {
                    let point_idx = indexes.choose(&mut rand::thread_rng()).unwrap();
                    points.push(dataset[*point_idx].clone());
                });
                points
            }
        };
        let mut labels: Vec<u32> = Vec::with_capacity(dataset.len());
        loop {
            let mut changed: bool = false;
            for (i, label) in labels.iter_mut().enumerate() {
                let current_point = &dataset[i];
                let distances = find_closest_point(current_point, &centers);

                let new_label = distances.first().unwrap().0;

                if *label != new_label {
                    changed = true;
                    *label = new_label;
                }
            }

            if !changed {
                break;
            }
            let grouped_by_label = labels.iter().enumerate().into_group_map_by(|a| a.1);

            grouped_by_label.iter().for_each(|(label, indexes)| {
                let objs: Dataset = indexes
                    .iter()
                    .map(|(_, idx)| dataset[**idx as usize].clone())
                    .collect();

                centers[**label as usize] = get_mean(&objs);
            });
        }
        self.centers = Some(centers);
    }

    fn transform(self, dataset: &Dataset) -> Option<Result> {
        match self.centers {
            None => None,
            Some(centers) => {
                let results = dataset
                    .iter()
                    .map(|x| find_closest_point(x, &centers).first().unwrap().0)
                    .collect();

                Some(results)
            }
        }
    }
}
