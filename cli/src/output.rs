use crate::types::*;
use serde::Serialize;

pub fn print_json<T: Serialize + ?Sized>(val: &T) {
    match serde_json::to_string_pretty(val) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("JSON serialization error: {e}"),
    }
}

pub fn print_context(context: &Context, json: bool) {
    if json {
        print_json(context);
    } else {
        // name (id)
        println!(
            "{} ({})",
            context.name.as_deref().unwrap_or("-"),
            context.id.as_deref().unwrap_or("-")
        );
    }
}

pub fn print_contexts(contexts: &[ContextWithStats], json: bool) {
    if json {
        print_json(contexts);
    } else if contexts.is_empty() {
        println!("No contexts found.");
    } else {
        for a in contexts {
            println!(
                "{} {}",
                a.context.id.as_deref().unwrap_or("-"),
                a.context.name.as_deref().unwrap_or("-"),
            );
        }
    }
}

#[allow(dead_code)]
pub fn print_task(task: &Task, json: bool) {
    if json {
        print_json(task);
    } else {
        // id workflow state
        println!(
            "{} {} {}",
            task.id.as_deref().unwrap_or("-"),
            task.workflow.as_deref().unwrap_or("-"),
            task.state.as_deref().unwrap_or("-"),
        );
    }
}

pub fn print_global_stats(stats: &GlobalStats, json: bool) {
    if json {
        print_json(stats);
    } else {
        println!(
            "{} total, {} active, {} completed",
            stats.tasks_total.unwrap_or(0),
            stats.tasks_active.unwrap_or(0),
            stats.tasks_completed.unwrap_or(0),
        );
    }
}
