use std::{collections::VecDeque, error::Error};

use crate::service::{ISolver, Token, TokenType, Tokenizer};

pub struct RPNSolver {

}

impl RPNSolver {
    fn infix_to_postfix(tokens: Vec<Token>) -> Result<VecDeque<Token>, Box<dyn Error>> {
        let mut postfix_expression: VecDeque<Token> = VecDeque::new();
        let mut operations_stack: VecDeque<Token> = VecDeque::new();

        // Для всех токенов
        for token in tokens {
            // Если у нас число, помещаем его в очередь
            if token.token_type == TokenType::NUMBER {
                postfix_expression.push_front(token);
                continue;
            }

            // Если нам попалась левая скобка, добавляем её в стек операций
            if token.get_operation()?.is_left_bracket() {
                operations_stack.push_back(token);
                continue;
            }

            // Если нам попалась правая скобка, будем добавлять в постфиксную запись все операции до тех пор, пока не увидим левую скобку.
            if token.get_operation()?.is_right_bracket() {
                while let Some(peeked_operation) = operations_stack.back() && !peeked_operation.get_operation()?.is_left_bracket() {
                    postfix_expression.push_front(operations_stack.pop_back().ok_or("Failed to pop")?);
                }

                if operations_stack.is_empty() {
                    return Err("A leveling of brackets is broken".into());
                }

                // Не забываем удалить левую скобку - она больше не нужна
                operations_stack.pop_back();
                continue;
            }

            // Иначе, мы попали на оператор. Исследуем верхний оператор
            match operations_stack.back() {
                Some(top_operation) => {
                    // Если верхний оператор - левая скобка, добавляем в массив операций токен
                    if top_operation.get_operation()?.is_left_bracket() {
                        operations_stack.push_back(token);
                        continue;
                    }

                    // Если приоритет текущей операции выше, чем у верхнего элемента, то добавляем его в стек
                    if token.get_operation()?.get_priority() > top_operation.get_operation()?.get_priority() {
                        operations_stack.push_back(token);
                    } else {
                        // Иначе добавляем все элементы из стека до тех пор, пока не достигнем левой скобки, 
                        // конца стека или операции с меньшим приоритетом
                        while let Some(peeked_operation) = operations_stack.back() && 
                        !(peeked_operation.get_operation()?.is_left_bracket() || 
                            peeked_operation.get_operation()?.get_priority() < token.get_operation()?.get_priority()) {
                            postfix_expression.push_front(operations_stack.pop_back().ok_or("Failed to pop")?);
                        }

                        // А потом добавляем токен в стек
                        operations_stack.push_back(token);
                    }
                },
                None => {
                    // Если стек операций пуст, добавляем в массив операций токен
                    operations_stack.push_back(token);
                    continue;
                }
            }
        }

        // Выгружаем оставшийся стек в очередь
        while let Some(popped_operation) = operations_stack.pop_back() {
            postfix_expression.push_front(popped_operation);
        }

        Ok(postfix_expression)
    }

    fn solve_postfix(mut tokens: VecDeque<Token>) -> Result<f64, Box<dyn Error>> {
        let mut args: VecDeque<f64> = VecDeque::new();
        
        while let Some(token) = tokens.pop_back() {
            if token.token_type == TokenType::NUMBER {
                args.push_back(token.get_number()?);
            } else {
                let computation_result = token.get_operation()?.compute(&mut args)?;
                args.push_back(computation_result);
            }
        }

        if args.len() != 1 {
            return Err("Expression is invalid".into());
        }
        
        Ok(args.pop_front().ok_or("Expression is invalid")?)
    }
}

impl ISolver for RPNSolver {
    fn solve(&self, expression: &str) -> Result<String, Box<dyn Error>>{
        // Шаг 1: токенизировать выражение
        let tokenized_expression = Tokenizer::tokenize(expression)?;

        // Шаг 2: преобразовать инфиксное выражение в постфиксное
        let postfix_expression = RPNSolver::infix_to_postfix(tokenized_expression)?;

        // Шаг 3: посчитать постфиксное выражение
        let postfix_result = RPNSolver::solve_postfix(postfix_expression)?;

        Ok(postfix_result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::service::{OperationMinus, OperationMultiply, OperationPlus, OperationTrait};

    use super::*;

    #[test]
    fn run_infix_to_postfix() {
        match RPNSolver::infix_to_postfix(Tokenizer::tokenize("5*6+(2-9)").unwrap()) {
            Ok(postfix) => {
                assert_eq!(postfix[0].token_type, TokenType::OPERATION);
                assert_eq!(postfix[0].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationPlus::get() as *const dyn OperationTrait);

                assert_eq!(postfix[1].token_type, TokenType::OPERATION);
                assert_eq!(postfix[1].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationMinus::get() as *const dyn OperationTrait);

                assert_eq!(postfix[2].token_type, TokenType::NUMBER);
                assert_eq!(postfix[2].get_number().unwrap(), 9.);

                assert_eq!(postfix[3].token_type, TokenType::NUMBER);
                assert_eq!(postfix[3].get_number().unwrap(), 2.);

                assert_eq!(postfix[4].token_type, TokenType::OPERATION);
                assert_eq!(postfix[4].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationMultiply::get() as *const dyn OperationTrait);

                assert_eq!(postfix[5].token_type, TokenType::NUMBER);
                assert_eq!(postfix[5].get_number().unwrap(), 6.);

                assert_eq!(postfix[6].token_type, TokenType::NUMBER);
                assert_eq!(postfix[6].get_number().unwrap(), 5.);
            },
            Err(err) => panic!("{err}")
        }
    }

    #[test]
    fn run_solve() {
        let solver = RPNSolver {};

        match solver.solve("5*6+(2-9)".into()) {
            Ok(result) => {
                assert_eq!(result, "23");
            },
            Err(err) => {
                panic!("{err}");
            }
        }
    }
}