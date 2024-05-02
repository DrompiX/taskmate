use std::rc::Rc;

use slint::Model;

slint::include_modules!();

fn print_task_list(task_list: &slint::VecModel<Task>) {
    println!("Tasks: [");
    for task in task_list.iter() {
        println!("\tTask info: {:?}", task)
    }
    println!("]")
}

struct PlaceholderRepo {
    pub tasks: Rc<slint::VecModel<Task>>,
}

impl PlaceholderRepo {
    pub fn new() -> Self {
        let tasks = Rc::new(slint::VecModel::from(vec![
            Task { id: "DAILY-1".into(), completed: false, description: "Task 1".into() },
            Task { id: "DAILY-2".into(), completed: true, description: "Task 2".into() },
            Task { id: "DAILY-3".into(), completed: false, description: "Task 3".into() },
            Task { id: "DAILY-4".into(), completed: false, description: "Task 4".into() },
        ]));
        PlaceholderRepo { tasks }
    }

    pub fn get(&self, task_id: String) -> Option<(usize, Task)> {
        self.tasks
            .iter()
            .enumerate()
            .find(|(_, task)| task.id == task_id)
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let task_repo = PlaceholderRepo::new();
    let tasks_model_rc = slint::ModelRc::from(task_repo.tasks.clone());
    print_task_list(&task_repo.tasks);
    ui.set_task_list(tasks_model_rc);

    ui.on_request_mark_task(move |task_id| {
        task_repo.get(task_id.into()).map(|(row, task_val)| {
            println!("Task before change {:?}", task_val);
            let mut new_task_val = task_val.clone();
            new_task_val.completed = !new_task_val.completed;
            println!("Changed state for {:?}", new_task_val);
            task_repo.tasks.set_row_data(row, new_task_val);
        });
        print_task_list(&task_repo.tasks);
    });

    ui.run()
}
