use std::cmp;
use std::collections::{HashMap, HashSet};

// common data structures

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: String,
    pub prio: u8,
    pub deadline: u32,
    pub inter_arrival: u32,
    pub trace: Trace,
}

//#[derive(Debug, Clone)]
#[derive(Debug, PartialEq)]
pub struct Trace {
    pub id: String,
    pub start: u32,
    pub end: u32,
    pub inner: Vec<Trace>,
}

// uselful types

// Our task set
pub type Tasks = Vec<Task>;

// A map from Task/Resource identifiers to priority
pub type IdPrio = HashMap<String, u8>;

// A map from Task identifiers to a set of Resource identifiers
pub type TaskResources = HashMap<String, HashSet<String>>;

// Derives the above maps from a set of tasks
pub fn pre_analysis(tasks: &Tasks) -> (IdPrio, TaskResources) {
    let mut ip = HashMap::new();
    let mut tr: TaskResources = HashMap::new();
    for t in tasks {
        update_prio(t.prio, &t.trace, &mut ip);
        for i in &t.trace.inner {
            update_tr(t.id.clone(), i, &mut tr);
        }
    }
    (ip, tr)
}

// helper functions
fn update_prio(prio: u8, trace: &Trace, hm: &mut IdPrio) {
    if let Some(old_prio) = hm.get(&trace.id) {
        if prio > *old_prio {
            hm.insert(trace.id.clone(), prio);
        }
    } else {
        hm.insert(trace.id.clone(), prio);
    }
    for cs in &trace.inner {
        update_prio(prio, cs, hm);
    }
}

fn update_tr(s: String, trace: &Trace, trmap: &mut TaskResources) {
    if let Some(seen) = trmap.get_mut(&s) {
        seen.insert(trace.id.clone());
    } else {
        let mut hs = HashSet::new();
        hs.insert(trace.id.clone());
        trmap.insert(s.clone(), hs);
    }
    for trace in &trace.inner {
        update_tr(s.clone(), trace, trmap);
    }
}

fn wcet(t: &Task) -> u32 {
    t.trace.end - t.trace.start
}

fn compute_load_factor(tasks: &Tasks) -> f32 {
    tasks
        .iter()
        .map(|t| wcet(t) as f32 / (t.inter_arrival as f32))
        .sum()
}

// Implement a function that takes a `Task` and returns the corresponding blocking time.
fn compute_blocking_time(tasks: &Tasks, under_analysis: &Task) -> u32 {
    // Traverse the full trace of a task, recording data all the while.
    fn traverse_trace<F: FnMut(&Trace, &Task) -> ()>(task: &Task, f: &mut F) {
        fn inner<F: FnMut(&Trace, &Task) -> ()>(traces: &Vec<Trace>, task: &Task, f: &mut F) {
            for trace in traces {
                f(trace, task);
                if !trace.inner.is_empty() {
                    inner(&trace.inner, task, f);
                }
            }
        }
        inner(&task.trace.inner, task, f);
    }

    // Record resources used in task under analysis
    let mut blocking_resources = HashSet::new();
    traverse_trace(&under_analysis, &mut |trace, _| {
        blocking_resources.insert(trace.id.clone());
    });

    // Record resource ceilings and critical sections for each all resources used in all other
    // tasks.
    let mut records = Vec::new();
    for task in tasks
        .iter()
        .filter(|t| t.prio < under_analysis.prio && t != &under_analysis)
    {
        traverse_trace(&task, &mut |trace, parent| {
            records.push((trace.id.clone(), parent.prio, trace.end - trace.start));
        });
    }

    // Find the longest critical section of a resource from the other tasks that have sufficiently
    // large resource ceilings.
    records
        .iter()
        .filter_map(|(id, prio, wcet)| {
            if blocking_resources.contains(id) && prio < &under_analysis.prio {
                Some(wcet)
            } else {
                None
            }
        })
        .fold(0, |prev, wcet| cmp::max(prev, *wcet))
}

fn compute_preemption_time(tasks: &Tasks, task: &Task) -> u32 {
    tasks
        .iter()
        .filter(|t| t != &task && t.prio >= task.prio)
        .map(|h| {
            // We assume worst-case: busy-time = deadline
            let preemptions = (task.deadline as f32 / h.inter_arrival as f32).ceil() as u32;
            wcet(h) * preemptions
        })
        .sum()
}

fn compute_response_time(tasks: &Tasks, task: &Task) -> u32 {
    wcet(task) + compute_blocking_time(tasks, task) + compute_preemption_time(tasks, task)
}

fn analyze_tasks(tasks: &Tasks) -> Vec<(&Task, u32, u32, u32, u32)> {
    let mut info = Vec::new();
    for task in tasks {
        info.push((
            task,
            compute_response_time(tasks, task),
            wcet(task),
            compute_blocking_time(tasks, task),
            compute_preemption_time(tasks, task),
        ));
    }

    info
}
