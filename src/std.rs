use std::collections::HashMap;
use std::fmt;

pub struct TaskInput<T: fmt::Display + Sized> {
    value: T,
    description: Option<String>,
}

impl<T: fmt::Display + Sized> TaskInput<T> {
    pub fn new(value: T, description: Option<String>) -> Self {
        TaskInput { value, description }
    }
}

impl<T: fmt::Display + Sized> fmt::Display for TaskInput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

pub enum Task {
    Shell {
        name: String,
        command: String,
        args: Option<Vec<TaskInput>>,
        cwd: Option<String>,
        env: Option<HashMap<String, String>>,
    },
}
