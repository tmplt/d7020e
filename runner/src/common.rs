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

fn compute_load_factor(tasks: &Tasks) -> f32 {
    tasks
        .iter()
        .map(|t| (t.trace.end - t.trace.start) as f32 / (t.inter_arrival as f32))
        .sum()
}

// Implement a function that takes a `Task` and returns the corresponding blocking time.
fn compute_blocking_time(tasks: &Tasks, task: &Task) -> u32 {
    // Record resource ceilings and critical sections for each all resources used in all other
    // tasks.
    fn record_resources(recs: &mut Vec<(u8, u32)>, traces: &Vec<Trace>, prio: u8) {
        for trace in traces {
            recs.push((prio, trace.end - trace.start));

            if !trace.inner.is_empty() {
                record_resources(recs, &trace.inner, prio);
            }
        }
    }
    let mut recs = Vec::new();
    for t in tasks.iter().filter(|t| t.prio < task.prio && t != &task) {
        record_resources(&mut recs, &t.trace.inner, t.prio);
    }

    // Find the longest critical section of a resource from the other tasks that have sufficiently
    // large resource ceilings.
    recs.iter()
        .filter_map(|(ceil, crit_len)| {
            if ceil >= &task.prio {
                Some(crit_len)
            } else {
                None
            }
        })
        .fold(0, |prev, crit_len| cmp::max(prev, *crit_len))
}

fn compute_preemption_time(tasks: &Tasks, task: &Task) -> u32 {
    tasks
        .iter()
        .filter(|t| t != &task && t.prio >= task.prio)
        .map(|h| {
            let wcet = h.trace.end - h.trace.start;
            let preemptions = (task.deadline as f32 / h.inter_arrival as f32).ceil() as u32;
            wcet * preemptions
        })
        .sum()
}
