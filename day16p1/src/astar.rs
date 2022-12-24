use std::collections::HashMap;
use std::hash::Hash;

use num::{Bounded, Zero};
use priority_queue::DoublePriorityQueue;

pub trait AStar {
    type Node: Hash + Eq + Clone + std::fmt::Debug;
    type Weight: Copy + Ord + Bounded + Zero + std::fmt::Debug;

    fn heuristic(&self, from: &Self::Node, to: &Self::Node) -> Self::Weight;
    fn weight(&self, from: &Self::Node, to: &Self::Node) -> Self::Weight;
    fn neighbours(&self, node: &Self::Node) -> Vec<Self::Node>;

    fn find_path(&self, from: &Self::Node, to: &Self::Node) -> Option<Vec<Self::Node>> {
        let mut open_set = DoublePriorityQueue::new();
        let mut scores = HashMap::new();
        let mut came_from = HashMap::new();

        open_set.push(from.clone(), self.heuristic(from, to));
        scores.insert(from.clone(), Self::Weight::zero());

        while let Some((current, _)) = open_set.pop_min() {
            if &current == to {
                return Some(reconstruct_path(from, to, came_from));
            }

            let score = scores
                .get(&current)
                .copied()
                .unwrap_or_else(|| Self::Weight::max_value());

            for neighbour in self.neighbours(&current) {
                let tentative_score = score + self.weight(&current, &neighbour);
                let neighbour_score = scores
                    .get(&neighbour)
                    .copied()
                    .unwrap_or_else(|| Self::Weight::max_value());
                if tentative_score < neighbour_score {
                    came_from.insert(neighbour.clone(), current.clone());
                    scores.insert(neighbour.clone(), tentative_score);
                    open_set.push_decrease(
                        neighbour.clone(),
                        tentative_score + self.heuristic(&current, &neighbour),
                    );
                }
            }
        }

        None
    }
}

fn reconstruct_path<N>(from: &N, to: &N, came_from: HashMap<N, N>) -> Vec<N>
where
    N: Hash + Eq + Clone,
{
    let mut path = Vec::from([to.clone()]);
    let mut here = to;

    while here != from {
        here = came_from.get(here).unwrap();
        path.insert(0, here.clone());
    }

    path
}
