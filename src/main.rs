use task_schedule::{schedule_tasks, ScheduledTask};
use task_schedule::utils::*;

fn task_without_params() {
    println!("I am running sec task.");
}

fn task_min() {
    println!("I am running min task.");
}

fn task_hours() {
    println!("I am running hours task.");
}

fn task_days() {
    println!("I am running days task.");
}

fn task_week() {
    println!("I am running week task.");
}

fn task_with_params(a: i32, b: &str) {
    println!("Task with parameters: a = {}, b = {}", a, b);
}

fn task_with_params_wrapper() {
    task_with_params(42, "Hello");
}


fn main() {
    let mut tasks = vec![
        ScheduledTask::new(convert_duration_to_seconds(1), task_without_params as fn()),
        ScheduledTask::new(convert_duration_to_seconds(2), task_with_params_wrapper as fn()),
        ScheduledTask::new(convert_duration_to_minutes(1), task_min as fn()),
        ScheduledTask::new(convert_duration_to_hours(1), task_hours as fn()),
        ScheduledTask::new(convert_duration_to_days(1), task_days as fn()),
        ScheduledTask::new(convert_duration_to_weeks(1), task_week as fn()),
    ];

    schedule_tasks(&mut tasks);
}