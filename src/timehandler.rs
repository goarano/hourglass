use std::collections::HashMap;

use model::project::*;
use model::task::*;
use model::timeentry::*;
use model::datetime::*;

pub struct TimeHandler {
    pub projects_map: HashMap<String,Project>,
    pub entries: Vec<TimeEntry>
}

pub fn add_task(timehandler: &mut TimeHandler, project: &Project, task: Task) {
    timehandler.projects_map.get_mut(&project.name).unwrap().tasks.push(task);
}

pub fn add_project(timehandler: &mut TimeHandler, project: Project) {
    timehandler.projects_map.insert(project.name.clone(), project);
}

pub fn is_timer_running(timehandler: &TimeHandler) -> bool {
    !timehandler.entries.is_empty() && timehandler.entries[timehandler.entries.len() - 1].end.is_none()
}

//pub fn start_timer(timeHandler: &mut TimeHandler, task: Task, start: DateTime) {
//    if is_timer_running(&timeHandler) {
//        end_timer(&mut timeHandler, start.clone());
//    }
//    let id = timeHandler.entries[timeHandler.entries.len() - 1].id + 1;
//    let entry = TimeEntry{
//        id: id,
//        task: task,
//        start: start,
//        end: None
//    };
//    timeHandler.entries.push(entry);
//}
//
//pub fn end_timer(timeHandler: &mut TimeHandler, end: DateTime) {
//    if timeHandler.entries.is_empty() || is_timer_running(&timeHandler) {
//        //TODO throw an exception or something
//    }
//    timeHandler.entries[timeHandler.entries.len() - 1].end = Some(end);
//}