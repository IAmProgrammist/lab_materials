use core::f64;
use std::{collections::VecDeque, error::Error};


pub trait OperationTrait {
    fn get_priority(&self) -> usize;
    
    fn compute(&self, arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>>;

    fn is_left_bracket(&self) -> bool {
        false
    }

    fn is_right_bracket(&self) -> bool {
        false
    }
}


pub struct OperationPlus {
}

static OPERATION_PLUS: OperationPlus = OperationPlus {};

impl OperationPlus {
    pub fn matches(token_raw: &String) -> bool {
        token_raw == "+"
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_PLUS
    }
}

impl OperationTrait for OperationPlus  {
    fn get_priority(&self) -> usize {
        0
    }

    fn compute(&self, arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        let error = "A plus operation can contain only two operands";

        Ok(arguments.pop_back().ok_or(error)? + 
            arguments.pop_back().ok_or(error)?)
    }
}


pub struct OperationMinus {
}

static OPERATION_MINUS: OperationMinus = OperationMinus {};

impl OperationMinus {
    pub fn matches(token_raw: &String) -> bool {
        token_raw == "-"
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_MINUS
    }
}

impl OperationTrait for OperationMinus  {
    fn get_priority(&self) -> usize {
        0
    }

    fn compute(&self, arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        let error = "A minus operation can contain only two operands";

        let a = arguments.pop_back().ok_or(error)?;
        let b = arguments.pop_back().ok_or(error)?;

        Ok(b - a)
    }
}


pub struct OperationMultiply {
}

static OPERATION_MULTIPLY: OperationMultiply = OperationMultiply {};

impl OperationMultiply {
    pub fn matches(token_raw: &String) -> bool {
        token_raw == "*"
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_MULTIPLY
    }
}

impl OperationTrait for OperationMultiply  {
    fn get_priority(&self) -> usize {
        1
    }

    fn compute(&self, arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        let error = "A multiply operation can contain only two operands";

        Ok(arguments.pop_back().ok_or(error)? * 
            arguments.pop_back().ok_or(error)?)
    }
}


pub struct OperationDivide {
}

static OPERATION_DIVIDE: OperationDivide = OperationDivide {};

impl OperationDivide {
    pub fn matches(token_raw: &String) -> bool {
        token_raw == "/"
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_DIVIDE
    }
}

impl OperationTrait for OperationDivide  {
    fn get_priority(&self) -> usize {
        1
    }

    fn compute(&self, arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        let error = "A divide operation can contain only two operands";

        let a = arguments.pop_back().ok_or(error)?;
        let b = arguments.pop_back().ok_or(error)?;

        Ok(b / a)
    }
}



pub struct OperationLeftBracket {
}

static OPERATION_LEFT_BRACKET: OperationLeftBracket = OperationLeftBracket {};

impl OperationLeftBracket {
    pub fn matches(token_raw: &String) -> bool {
        token_raw == "(" || token_raw == "{" || token_raw == "["
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_LEFT_BRACKET
    }
}

impl OperationTrait for OperationLeftBracket  {
    fn get_priority(&self) -> usize {
        2
    }

    fn is_left_bracket(&self) -> bool {
        true
    }

    fn compute(&self, _arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        Err("Brackets cannot be computed".into())
    }
}



pub struct OperationRightBracket {
}

static OPERATION_RIGHT_BRACKET: OperationRightBracket = OperationRightBracket {};

impl OperationRightBracket {
    pub fn matches(token_raw: &String) -> bool {
        token_raw == ")" || token_raw == "}" || token_raw == "]"
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_RIGHT_BRACKET
    }
}

impl OperationTrait for OperationRightBracket  {
    fn get_priority(&self) -> usize {
        2
    }

    fn is_right_bracket(&self) -> bool {
        true
    }

    fn compute(&self, _arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        Err("Brackets cannot be computed".into())
    }
}



pub struct OperationNone {
}

static OPERATION_NONE: OperationNone = OperationNone {};

impl OperationNone {
    pub fn matches(_token_raw: &str) -> bool {
        false
    }

    pub fn get() -> &'static dyn OperationTrait {
        &OPERATION_NONE
    }
}

impl OperationTrait for OperationNone  {
    fn get_priority(&self) -> usize {
        0
    }

    fn compute(&self, _arguments: &mut VecDeque<f64>) -> Result<f64, Box<dyn Error>> {
        Err("Empty operation is not computable".into())
    }
}


#[derive(PartialEq, Debug)]
pub enum TokenType {
    NUMBER,
    OPERATION
}

pub struct Token {
    number: f64,
    operation: &'static dyn OperationTrait,
    pub token_type: TokenType
}

impl Token {
    pub fn get_number(&self) -> Result<f64, Box<dyn Error>> {
        if self.token_type == TokenType::NUMBER {
            return Ok(self.number)
        }

        Err("A token is not a number".into())
    }

    pub fn get_operation(&self) -> Result<&'static dyn OperationTrait, Box<dyn Error>> {
        if self.token_type == TokenType::OPERATION {
            return Ok(self.operation)
        }

        Err("A token is not an operation".into())
    }
}

pub struct Tokenizer {}

impl Tokenizer {
    fn match_operation(operation: &String) -> Option<&'static dyn OperationTrait> {
        if OperationDivide::matches(operation) {
            return Some(OperationDivide::get());
        } else if OperationMultiply::matches(operation) {
            return Some(OperationMultiply::get());
        } else if OperationPlus::matches(operation) {
            return Some(OperationPlus::get());
        } else if OperationMinus::matches(operation) {
            return Some(OperationMinus::get());
        } else if OperationLeftBracket::matches(operation) {
            return Some(OperationLeftBracket::get())
        } else if OperationRightBracket::matches(operation) {
            return Some(OperationRightBracket::get())
        }

        None
    }

    fn parse_number(number: &str) -> Result<Option<f64>, Box<dyn Error>> {
        if number.is_empty() {
            return Ok(None)
        }

        Ok(Some(number.parse::<f64>()?))
    }

    pub fn tokenize(expression: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokenized: Vec<Token> = vec![];
        let mut number_buffer: String = "".to_string();
        for character in expression.chars() {
            // Пропускаем лишние символы
            if character.is_whitespace() {
                continue;
            }

            // Если мы нашли операцию
            if let Some(operation) = Tokenizer::match_operation(&character.to_string()) {
                // То парсим и сохраняем число, если оно у нас накопилось
                if let Some(parsed_number_buffer) = Tokenizer::parse_number(&number_buffer)? {
                    tokenized.push(Token { number: parsed_number_buffer, operation: OperationNone::get(), token_type: TokenType::NUMBER });
                    number_buffer = "".to_string();
                }

                // И сохраняем саму операцию
                tokenized.push(Token { number: 0., operation, token_type: TokenType::OPERATION });
            } else {
                // Иначе - считаем, что мы попали на составляющую числа. Продолжаем подсчёт.
                number_buffer.push(character);
            }
            
        }

        if let Some(parsed_number_buffer) = Tokenizer::parse_number(&number_buffer)? {
            tokenized.push(Token { number: parsed_number_buffer, operation: OperationNone::get(), token_type: TokenType::NUMBER });
        }

        Ok(tokenized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_plus() {
        let plus = OperationPlus::get();
        assert_eq!(plus.get_priority(), 0);
        
        let mut args: VecDeque<f64> = VecDeque::new();
        args.push_back(1.);
        args.push_back(2.);
        match plus.compute(&mut args) {
            Ok(comp_result) => {
                assert_eq!(comp_result, 3.);
                assert_eq!(args.len(), 0);
            },
            Err(err) => panic!("{err}")
        }

        match plus.compute(&mut args) {
            Ok(_) => panic!("Cannot compute for zero args"),
            Err(_) => {}
        }
    }

    #[test]
    fn run_minus() {
        let minus = OperationMinus::get();
        assert_eq!(minus.get_priority(), 0);
        
        let mut args: VecDeque<f64> = VecDeque::new();
        args.push_back(1.);
        args.push_back(2.);
        match minus.compute(&mut args) {
            Ok(comp_result) => {
                assert_eq!(comp_result, -1.);
                assert_eq!(args.len(), 0);
            },
            Err(err) => panic!("{err}")
        }

        match minus.compute(&mut args) {
            Ok(_) => panic!("Cannot compute for zero args"),
            Err(_) => {}
        }
    }

    #[test]
    fn run_multiply() {
        let multiply = OperationMultiply::get();
        assert_eq!(multiply.get_priority(), 1);
        
        let mut args: VecDeque<f64> = VecDeque::new();
        args.push_back(3.);
        args.push_back(2.);
        match multiply.compute(&mut args) {
            Ok(comp_result) => {
                assert_eq!(comp_result, 6.);
                assert_eq!(args.len(), 0);
            },
            Err(err) => panic!("{err}")
        }

        match multiply.compute(&mut args) {
            Ok(_) => panic!("Cannot compute for zero args"),
            Err(_) => {}
        }
    }

    #[test]
    fn run_divide() {
        let divide: &'static dyn OperationTrait = OperationDivide::get();
        assert_eq!(divide.get_priority(), 1);
        
        let mut args: VecDeque<f64> = VecDeque::new();
        args.push_back(1.);
        args.push_back(2.);
        match divide.compute(&mut args) {
            Ok(comp_result) => {
                assert_eq!(comp_result, 0.5);
                assert_eq!(args.len(), 0);
            },
            Err(err) => panic!("{err}")
        }

        match divide.compute(&mut args) {
            Ok(_) => panic!("Cannot compute for zero args"),
            Err(_) => {}
        }
    }

    #[test]
    fn run_tokenizer() {
        match Tokenizer::tokenize( "0 ( 1.75 -2    ) * /\r\n- +") {
            Ok(tokens_parsed) => {
                assert_eq!(tokens_parsed.len(), 10);
                
                assert_eq!(tokens_parsed[0].token_type, TokenType::NUMBER);
                assert_eq!(tokens_parsed[0].get_number().unwrap(), 0.);
                
                assert_eq!(tokens_parsed[1].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[1].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationLeftBracket::get() as *const dyn OperationTrait);
                
                assert_eq!(tokens_parsed[2].token_type, TokenType::NUMBER);
                assert_eq!(tokens_parsed[2].get_number().unwrap(), 1.75);
                
                assert_eq!(tokens_parsed[3].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[3].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationMinus::get() as *const dyn OperationTrait);
                
                assert_eq!(tokens_parsed[4].token_type, TokenType::NUMBER);
                assert_eq!(tokens_parsed[4].get_number().unwrap(), 2.0);
                
                assert_eq!(tokens_parsed[5].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[5].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationRightBracket::get() as *const dyn OperationTrait);
                
                assert_eq!(tokens_parsed[6].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[6].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationMultiply::get() as *const dyn OperationTrait);
                
                assert_eq!(tokens_parsed[7].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[7].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationDivide::get() as *const dyn OperationTrait);
                
                assert_eq!(tokens_parsed[8].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[8].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationMinus::get() as *const dyn OperationTrait);
                
                assert_eq!(tokens_parsed[9].token_type, TokenType::OPERATION);
                assert_eq!(tokens_parsed[9].get_operation().unwrap() as *const dyn OperationTrait, 
                OperationPlus::get() as *const dyn OperationTrait);
            },
            Err(_) => panic!("Legal tokens are considered illegal")
        }

        match Tokenizer::tokenize("32918 ~ jfie") {
            Ok(_) => panic!("Expression should be unparseable"),
            Err(_) => {}
        }
    }
}