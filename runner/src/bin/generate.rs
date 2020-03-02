// use std::collections::{HashMap, HashSet};
use runner::common::*;
use std::error::Error;
use std::io;

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
    let tasks = analyze_tasks(&tasks, approx);

    fn dump_to_csv(tasks: &Vec<TaskInfo>) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(&[
            "Task",            // T*
            "Resources",       // R*
            "Prio",            // P(t)
            "Deadline",        // D(t)
            "WCET",            // C(t)
            "Inter-arrival",   // A(t)
            "Schedulable",     // true or false
            "Response time",   // R(t)
            "Blocking time",   // B(t)
            "Preemption time", // I(t)
        ])?;

        for (task, resources, wcet, schedulable, r, b, i) in tasks {
            wtr.write_record(&[
                task.id.clone(),
                resources
                    .iter()
                    .fold("".to_string(), |prefix, r| prefix + " " + r)
                    .trim()
                    .to_string(),
                task.prio.to_string(),
                task.deadline.to_string(),
                wcet.to_string(),
                task.inter_arrival.to_string(),
                schedulable.to_string(),
                r.to_string(),
                b.to_string(),
                i.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    dump_to_csv(&tasks).unwrap();
}
