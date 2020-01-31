// use std::collections::{HashMap, HashSet};
use runner::common::*;

fn main() {
    let t1 = Task {
        id: "T1".to_string(),
        prio: 1,
        deadline: 100,
        inter_arrival: 100,
        trace: Trace {
            id: "T1".to_string(),
            start: 0,
            end: 10,
            inner: vec![],
        },
    };

    let t2 = Task {
        id: "T2".to_string(),
        prio: 2,
        deadline: 200,
        inter_arrival: 200,
        trace: Trace {
            id: "T2".to_string(),
            start: 0,
            end: 30,
            inner: vec![
                Trace {
                    id: "R1".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![Trace {
                        id: "R2".to_string(),
                        start: 12,
                        end: 16,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R1".to_string(),
                    start: 22,
                    end: 28,
                    inner: vec![],
                },
            ],
        },
    };

    let t3 = Task {
        id: "T3".to_string(),
        prio: 3,
        deadline: 50,
        inter_arrival: 50,
        trace: Trace {
            id: "T3".to_string(),
            start: 0,
            end: 30,
            inner: vec![Trace {
                id: "R2".to_string(),
                start: 10,
                end: 20,
                inner: vec![],
            }],
        },
    };

    // builds a vector of tasks t1, t2, t3
    let tasks: Tasks = vec![t1, t2, t3];

    let approx = false;
    for (task, result) in analyze_tasks(&tasks, approx) {
        match result {
            Ok((r, c, b, i)) => {
                println!(
                    "Task {}:\tR(t) = {}\tC(t) = {},\tB(t) = {},\tI(t) = {}",
                    task.id, r, c, b, i
                );
            }
            Err(r) => {
                println!(
                    "Task {}:\tNot schedulable; R(t) = {} >= D(t) = {}.",
                    task.id, r, task.deadline
                );
            }
        }
    }
}
