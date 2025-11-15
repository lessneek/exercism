#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn run_cmd<F: Fn(i32, i32) -> i32>(stack: &mut Vec<i32>, f: F) -> bool {
    if stack.len() >= 2 {
        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let x = f(a, b);
        stack.push(x);
        return true;
    }
    false
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        println!("{:?}", input);
        let good = match input {
            CalculatorInput::Add => run_cmd(&mut stack, |a, b| a + b),
            CalculatorInput::Subtract => run_cmd(&mut stack, |a, b| a - b),
            CalculatorInput::Multiply => run_cmd(&mut stack, |a, b| a * b),
            CalculatorInput::Divide => run_cmd(&mut stack, |a, b| a / b),
            CalculatorInput::Value(x) => {
                stack.push(*x);
                true
            }
        };
        if !good {
            return None;
        }
        println!("{:?}", stack);
    }
    if stack.len() == 1 {
        return stack.pop();
    }
    None
}
