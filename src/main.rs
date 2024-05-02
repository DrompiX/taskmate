use std::rc::Rc;

use slint::{Model, ModelRc, VecModel};

slint::include_modules!();

fn print_task_list(task_list: &VecModel<Task>) {
    println!("Tasks: [");
    for task in task_list.iter() {
        println!("\tTask info: {:?}", task)
    }
    println!("]")
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let tasks : Rc<VecModel<Task>> = Rc::new(VecModel::from(vec![
        Task {id: "DAILY-1".into(), completed: false, description: "Task 1".into()},
        Task {id: "DAILY-2".into(), completed: true, description: "Task 2".into()},
        Task {id: "DAILY-3".into(), completed: false, description: "Task 3".into()},
        Task {id: "DAILY-4".into(), completed: false, description: "Task 4".into()},
    ]));
    let tasks_model_rc = ModelRc::from(tasks.clone());
    print_task_list(&tasks);
    ui.set_task_list(tasks_model_rc);

    ui.on_request_mark_task(
        move |task_id| {
            for mut task in tasks.iter() {
                if task.id == task_id {
                    println!("Task before change {:?}", task);
                    task.completed = !task.completed;
                    println!("Changed state for {:?}", task);
                    // tasks.push(Task {id: "1".into(), completed: false, description: "heh".into()});
                }
            }
            print_task_list(&tasks);
        }
    );

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let new_counter = ui.get_counter() + 1;
    //         ui.set_counter(new_counter);
    //         ui.set_name("Dmitry".into());
    //     }
    // });

    ui.run()
}
