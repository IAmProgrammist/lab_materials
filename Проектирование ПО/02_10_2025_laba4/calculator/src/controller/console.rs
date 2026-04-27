use crate::service::ISolver;

pub struct ConsoleController<'a> {
    solver: &'a dyn ISolver
}

impl ConsoleController<'_> {
    pub fn new(solver: &'_ dyn ISolver) -> ConsoleController<'_> {
        ConsoleController {solver}
    }

    pub fn mainloop(self) {
        loop {
            println!("Введите выражение: ");
            let mut  input_expression = String::new();
            match std::io::stdin().read_line(&mut input_expression) {
                Ok(_) => {
                    match input_expression.as_str().trim() {
                        "exit" => {
                            break;
                        },
                        "help" => {
                            println!("Введите выражение для решения\nВведите exit для выхода из калькулятора\nВведите help для вывода помощи")
                        },
                        _ => {
                            match self.solver.solve(&input_expression) {
                                Ok(result) => {
                                    println!("Ответ: {result}");
                                },
                                Err(err) => {
                                    println!("Выражение содержит ошибки: {err}\nВы можете ввести help для получения помощи");
                                }
                            }       
                        }
                    }
                },
                Err(e) => {
                    println!("Не удалось считать выражение из-за ошибки: {e}");
                }
            }
        }
    }
}