#[macro_use]
extern crate unborrow;
use if_chain::if_chain;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let stack: Vec<Option<i32>> = inputs.iter().fold(vec![], |mut acc, input| {
        match input {
            CalculatorInput::Value(v) => acc.push(Some(*v)),
            CalculatorInput::Add => unborrow!(acc.push(combine_args(&mut acc, Add::add))),
            CalculatorInput::Subtract => unborrow!(acc.push(combine_args(&mut acc, Sub::sub))),
            CalculatorInput::Multiply => unborrow!(acc.push(combine_args(&mut acc, Mul::mul))),
            CalculatorInput::Divide => unborrow!(acc.push(combine_args(&mut acc, Div::div))),
        };
        acc
    });
    if_chain! {
        if stack.len() == 1;
        if let Some(res) = stack.get(0);
        then {
           *res
        }
        else {
            None
        }
    }
}

fn combine_args<F>(stack: &mut Vec<Option<i32>>, func: F) -> Option<i32>
where
    F: Fn(i32, i32) -> i32,
{
    let args = get_args(stack);
    match args {
        Some((a, b)) => Some(func(b, a)),
        None => None,
    }
}

fn get_args(stack: &mut Vec<Option<i32>>) -> Option<(i32, i32)> {
    let first = stack.pop();
    let second = stack.pop();
    match (first, second) {
        (Some(Some(a)), Some(Some(b))) => Some((a, b)),
        _ => None,
    }
}
