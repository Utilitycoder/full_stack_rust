use crate::{
    state::{TaskAction, TaskState},
    todo_api,
};

use yew::UseReducerHandle;

pub struct TaskController {
    pub state: UseReducerHandle<TaskState>,
}

impl TaskController {
    pub fn new(state: UseReducerHandle<TaskState>) -> TaskController {
        TaskController { state }
    }

    pub fn init_tasks(&self) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_tasks = todo_api::get_tasks().await.unwrap();
            tasks.dispatch(TaskAction::Set(fetched_tasks));
        });
    }

    pub fn fetch_tasks(&self) {
        let state = self.state.clone();
        let callback = move |tasks| {
            state.dispatch(TaskAction::Set(tasks));
        };
        todo_api::get_tasks(callback);
    }

    pub fn add_task(&self, title: String) {
        let state = self.state.clone();
        let callback = move |task| {
            state.dispatch(TaskAction::Add(task));
        };
        todo_api::add_task(title, callback);
    }

    pub fn delete_task(&self, id: String) {
        let state = self.state.clone();
        let callback = move |_| {
            state.dispatch(TaskAction::Delete(id));
        };
        todo_api::delete_task(id, callback);
    }

    pub fn toggle_task(&self, id: String) {
        let state = self.state.clone();
        let callback = move |_| {
            state.dispatch(TaskAction::Toggle(id));
        };
        todo_api::toggle_task(id, callback);
    }
}