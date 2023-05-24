use std::rc::Rc;
use yew::prelude::*;

mod components;
mod controllers;
mod models;
mod state;
mod todo_api;

use components::*;
use controllers::*;
use state::*;

#[function_component(App)]
fn app() -> Html {
    let tasks = use_reducer(TaskState::default);
    let task_controller = Rc::new(TaskController::new(tasks.clone()));

    {
        let task_controller = task_controller.clone();
        use_effect_with_deps(
            move |_| {
                task_controller.init_tasks();
                || ()
            },
            (),
        );
    }

    let on_create_task = {
        let task_controller = task_controller.clone();
        Callback::from(move |title: String| {
            task_controller.create_task(title);
        })
    };

    let on_delete_task = {
        let task_controller = task_controller.clone();
        Callback::from(move |id: String| {
            task_controller.delete_task(id);
        })
    };

    let on_toggle_task = {
        let task_controller = task_controller;
        Callback::from(move |id: String| {
            task_controller.toggle_task(id);
        })
    };

    html! {
        <div class="container">
        <TaskForm on_create_task={on_create_task} />
        <h3>{"Todo"}</h3>
        <div>
            <TaskList
                tasks={tasks.tasks.clone()}
                on_delete_task={on_delete_task}
                on_toggle_task={on_toggle_task}
            />
        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
