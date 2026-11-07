// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// Critical-path method: forward pass (early start/finish), backward pass (late
// start/finish, float), critical-path flag (float == 0). Pure compute — no HTTP/storage
// concerns, matching tool-construction-core's own compute/render separation discipline.
// Units are caller-supplied day-offsets, never real dates (same posture as
// tool-typeset's TimeAxis) — the caller maps offsets back to real dates if it wants them.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Task {
    pub task_id: String,
    pub cost_code: String,
    pub description: String,
    pub duration_days: f64,
    pub depends_on: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ComputedTask {
    pub task_id: String,
    pub early_start: f64,
    pub early_finish: f64,
    pub late_start: f64,
    pub late_finish: f64,
    pub float: f64,
    pub critical: bool,
}

#[derive(Debug, PartialEq)]
pub enum CpmError {
    UnknownDependency { task_id: String, missing: String },
    Cycle,
}

impl fmt::Display for CpmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpmError::UnknownDependency { task_id, missing } => {
                write!(f, "task {task_id} depends on unknown task {missing}")
            }
            CpmError::Cycle => write!(f, "dependency graph contains a cycle"),
        }
    }
}

/// Topologically sorts tasks by dependency (Kahn's algorithm), then runs the
/// forward and backward passes. Errors on an unknown dependency or a cycle
/// rather than silently producing a wrong schedule.
pub fn compute(tasks: &[Task]) -> Result<Vec<ComputedTask>, CpmError> {
    let ids: BTreeSet<&str> = tasks.iter().map(|t| t.task_id.as_str()).collect();
    for t in tasks {
        for dep in &t.depends_on {
            if !ids.contains(dep.as_str()) {
                return Err(CpmError::UnknownDependency {
                    task_id: t.task_id.clone(),
                    missing: dep.clone(),
                });
            }
        }
    }

    let order = topo_order(tasks)?;
    let by_id: BTreeMap<&str, &Task> = tasks.iter().map(|t| (t.task_id.as_str(), t)).collect();

    // Forward pass: early_start = max(early_finish of all dependencies), 0 if none.
    let mut early_start: BTreeMap<String, f64> = BTreeMap::new();
    let mut early_finish: BTreeMap<String, f64> = BTreeMap::new();
    for id in &order {
        let t = by_id[id.as_str()];
        let es = t
            .depends_on
            .iter()
            .map(|d| early_finish[d])
            .fold(0.0_f64, f64::max);
        early_start.insert(id.clone(), es);
        early_finish.insert(id.clone(), es + t.duration_days);
    }

    let project_finish = early_finish.values().cloned().fold(0.0_f64, f64::max);

    // Successors, for the backward pass.
    let mut successors: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for t in tasks {
        for dep in &t.depends_on {
            successors
                .entry(dep.as_str())
                .or_default()
                .push(t.task_id.as_str());
        }
    }

    // Backward pass, in reverse topological order: late_finish = min(late_start of
    // successors), or project_finish for a sink task with no successors.
    let mut late_start: BTreeMap<String, f64> = BTreeMap::new();
    let mut late_finish: BTreeMap<String, f64> = BTreeMap::new();
    for id in order.iter().rev() {
        let t = by_id[id.as_str()];
        let succs = successors.get(id.as_str());
        let lf = match succs {
            Some(s) if !s.is_empty() => s.iter().map(|s| late_start[*s]).fold(f64::MAX, f64::min),
            _ => project_finish,
        };
        late_finish.insert(id.clone(), lf);
        late_start.insert(id.clone(), lf - t.duration_days);
    }

    Ok(order
        .iter()
        .map(|id| {
            let es = early_start[id];
            let ef = early_finish[id];
            let ls = late_start[id];
            let lf = late_finish[id];
            let float = ls - es;
            ComputedTask {
                task_id: id.clone(),
                early_start: es,
                early_finish: ef,
                late_start: ls,
                late_finish: lf,
                float,
                critical: float.abs() < 1e-9,
            }
        })
        .collect())
}

fn topo_order(tasks: &[Task]) -> Result<Vec<String>, CpmError> {
    let mut in_degree: BTreeMap<&str, usize> =
        tasks.iter().map(|t| (t.task_id.as_str(), 0)).collect();
    let mut dependents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for t in tasks {
        *in_degree.get_mut(t.task_id.as_str()).unwrap() += t.depends_on.len();
        for dep in &t.depends_on {
            dependents
                .entry(dep.as_str())
                .or_default()
                .push(t.task_id.as_str());
        }
    }
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(id, _)| *id)
        .collect();
    queue.sort(); // deterministic order for equal-priority ready tasks
    let mut order = Vec::with_capacity(tasks.len());
    let mut idx = 0;
    while idx < queue.len() {
        let id = queue[idx];
        idx += 1;
        order.push(id.to_string());
        if let Some(deps) = dependents.get(id) {
            let mut newly_ready = Vec::new();
            for &d in deps {
                let e = in_degree.get_mut(d).unwrap();
                *e -= 1;
                if *e == 0 {
                    newly_ready.push(d);
                }
            }
            newly_ready.sort();
            queue.extend(newly_ready);
        }
    }
    if order.len() != tasks.len() {
        return Err(CpmError::Cycle);
    }
    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(id: &str, duration: f64, deps: &[&str]) -> Task {
        Task {
            task_id: id.into(),
            cost_code: "TEST".into(),
            description: id.into(),
            duration_days: duration,
            depends_on: deps.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn single_task_has_zero_float_and_is_critical() {
        let tasks = vec![task("A", 5.0, &[])];
        let result = compute(&tasks).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].early_start, 0.0);
        assert_eq!(result[0].early_finish, 5.0);
        assert!(result[0].critical);
    }

    /// Classic 3-task diamond: A -> B, A -> C, B+C -> D. B (3d) and C (7d) run in
    /// parallel off A; D can't start until both finish. C is on the critical path;
    /// B has 4 days of float. Known-answer fixture per the approved plan.
    #[test]
    fn diamond_dependency_known_critical_path() {
        let tasks = vec![
            task("A", 2.0, &[]),
            task("B", 3.0, &["A"]),
            task("C", 7.0, &["A"]),
            task("D", 1.0, &["B", "C"]),
        ];
        let result = compute(&tasks).unwrap();
        let by_id: BTreeMap<_, _> = result.iter().map(|t| (t.task_id.as_str(), t)).collect();

        assert_eq!(by_id["A"].early_start, 0.0);
        assert_eq!(by_id["A"].early_finish, 2.0);
        assert!(by_id["A"].critical);

        assert_eq!(by_id["B"].early_start, 2.0);
        assert_eq!(by_id["B"].early_finish, 5.0);
        assert_eq!(by_id["B"].float, 4.0);
        assert!(!by_id["B"].critical);

        assert_eq!(by_id["C"].early_start, 2.0);
        assert_eq!(by_id["C"].early_finish, 9.0);
        assert_eq!(by_id["C"].float, 0.0);
        assert!(by_id["C"].critical);

        assert_eq!(by_id["D"].early_start, 9.0);
        assert_eq!(by_id["D"].early_finish, 10.0);
        assert!(by_id["D"].critical);

        // Project finish is 10 days — the longest path (A->C->D), not the A->B->D
        // path, confirming the forward pass takes the max, not the first-seen dep.
        let project_finish = result
            .iter()
            .map(|t| t.early_finish)
            .fold(0.0_f64, f64::max);
        assert_eq!(project_finish, 10.0);
    }

    #[test]
    fn unknown_dependency_is_an_error_not_a_silent_zero() {
        let tasks = vec![task("A", 1.0, &["ghost"])];
        assert_eq!(
            compute(&tasks),
            Err(CpmError::UnknownDependency {
                task_id: "A".into(),
                missing: "ghost".into()
            })
        );
    }

    #[test]
    fn cycle_is_detected_not_infinite_looped() {
        let tasks = vec![task("A", 1.0, &["B"]), task("B", 1.0, &["A"])];
        assert_eq!(compute(&tasks), Err(CpmError::Cycle));
    }
}
