use std::collections::VecDeque;

use crate::fact::LabeledFact;

pub trait DependsOn {
    fn depends_on(&self, maybe_dependency: &Self) -> bool;
}

impl DependsOn for LabeledFact {
    fn depends_on(&self, maybe_dependency: &Self) -> bool {
        maybe_dependency
            .exports()
            .keys()
            .any(|key| self.imports().contains(key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SortDependenciesError {
    #[error("dependency cycle detected")]
    DependencyCycle,
}

pub fn sort_dependencies<T: DependsOn>(items: Vec<T>) -> Result<Vec<T>, SortDependenciesError> {
    let n = items.len();

    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut dependency_count = vec![0usize; n];

    for dependent in 0..n {
        for dependency in 0..n {
            if dependent != dependency && items[dependent].depends_on(&items[dependency]) {
                dependents[dependency].push(dependent);
                dependency_count[dependent] += 1;
            }
        }
    }

    let mut ready: VecDeque<usize> = dependency_count
        .iter()
        .enumerate()
        .filter_map(|(i, &count)| (count == 0).then_some(i))
        .collect();

    let mut order = Vec::with_capacity(n);

    while let Some(item) = ready.pop_front() {
        order.push(item);

        for &dependent in &dependents[item] {
            dependency_count[dependent] -= 1;

            if dependency_count[dependent] == 0 {
                ready.push_back(dependent);
            }
        }
    }

    if order.len() != n {
        return Err(SortDependenciesError::DependencyCycle);
    }

    let mut items: Vec<Option<T>> = items.into_iter().map(Some).collect();

    let sorted = order
        .into_iter()
        .map(|i| items[i].take().unwrap())
        .collect();

    Ok(sorted)
}
