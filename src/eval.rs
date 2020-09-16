use crate::parser::Expression;
use std::collections::HashMap;

/// This enum represents all the types of values in my Scheme dialect. Currently that means floats,
/// integers, booleans, functions, pairs, and nil.
#[derive(Clone, Debug)]
pub enum Value {
    // TODO: add more types. String? Vector? Char? Symbol?
    Number(f64),
    Function(Vec<String>, Expression),
    Bool(bool),
    Integer(isize),
    Pair(Box<Value>, Box<Value>),
    Nil,
}

/// Prints values the obvious way. I couldn't quite figure out how to tell if something might be a
/// list or not, and how to print it nicely, so it does just print all pairs as `(car . cdr)`
/// unfortunately.
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", *n),
            Value::Function(_, _) => write!(f, "function"),
            Value::Bool(true) => write!(f, "#t"),
            Value::Bool(false) => write!(f, "#f"),
            Value::Integer(n) => write!(f, "{}", *n),
            Value::Pair(car, cdr) => write!(f, "({} . {})", car, cdr),
            Value::Nil => write!(f, "()"),
        }
    }
}

/// Function for converting a value back into an expression. Mostly useless, except for checking
/// environment for pairs.
fn value_to_expression(val: Value) -> Expression {
    match val {
        Value::Bool(b) => Expression::Bool(b),
        Value::Number(n) => Expression::Number(n),
        Value::Integer(n) => Expression::Integer(n),
        Value::Function(p, b) => Expression::Lambda(p, Box::new(b)),
        Value::Nil => Expression::Nil,
        Value::Pair(car, cdr) => Expression::Pair(
            Box::new(value_to_expression(*car)),
            Box::new(value_to_expression(*cdr)),
        ),
    }
}

/// This function takes an expression and replaces most of the known identifiers with their values.
/// It does respect shadowing.
fn check_environment(expr: Expression, env: &HashMap<String, Value>) -> Option<Expression> {
    match expr {
        Expression::Identifier(s) => match env.get(&s) {
            Some(Value::Integer(n)) => Some(Expression::Integer(*n)),
            Some(Value::Number(n)) => Some(Expression::Number(*n)),
            Some(Value::Function(params, body)) => {
                Some(Expression::Lambda(params.clone(), Box::new(body.clone())))
            }
            Some(Value::Nil) => Some(Expression::Nil),
            Some(Value::Bool(b)) => Some(Expression::Bool(*b)),
            Some(Value::Pair(car, cdr)) => Some(Expression::Pair(
                Box::new(value_to_expression(*car.clone())),
                Box::new(value_to_expression(*cdr.clone())),
            )),
            None => Some(Expression::Identifier(s.clone())),
        },
        Expression::Lambda(params, body) => Some(Expression::Lambda(
            params.iter().cloned().collect(),
            Box::new(check_environment(*body, env)?),
        )),
        Expression::SExpression(head, tail) => Some(Expression::SExpression(
            Box::new(check_environment(*head, env)?),
            tail.iter()
                .map(|e| check_environment(e.clone(), env).unwrap()) // TODO: better error handling here
                .collect(),
        )),
        e => Some(e),
    }
}

// TODO: optimizations? blowing up the stack is way too common: add loops or tail-call optimization; somehow
/// This function is the heart of the "interpreter". It takes an expression and returns the value
/// it evaluated to. It's a gigantic, messy, recursive pile of pattern matching that somehow gets
/// the job done. The bulk of the function is actually implementing built-in functions, so I might
/// separate those into a different file.
pub fn eval_expression(
    expr: &Expression,
    env: &mut HashMap<String, Value>,
) -> Result<Value, String> {
    match expr {
        Expression::Number(n) => Ok(Value::Number(*n)),
        Expression::Integer(n) => Ok(Value::Integer(*n)),
        Expression::Bool(b) => Ok(Value::Bool(*b)),
        Expression::Identifier(s) => env
            .get(s)
            .cloned()
            .ok_or(format!("Variable {} not in environment!", s)),
        Expression::Nil => Ok(Value::Nil),
        Expression::Lambda(params, body) => {
            let mut env2 = env.clone();
            for param in params {
                env2.remove(param);
            }
            Ok(Value::Function(
                params.clone(),
                check_environment(*body.clone(), &env2).ok_or("Check environment failed!")?,
            ))
        }
        Expression::Pair(a, b) => Ok(Value::Pair(
            Box::new(eval_expression(a, env)?),
            Box::new(eval_expression(b, env)?),
        )),
        Expression::Define(s, expr) => {
            let res = eval_expression(expr, env)?;
            env.insert(s.clone(), res);
            Ok(Value::Nil)
        }
        Expression::If(cond, if_branch, else_branch) => {
            let cond = eval_expression(&**cond, env)?; // TODO: is there any way to clean the &**? That seems like a code smell
            match cond {
                Value::Bool(b) => {
                    if b {
                        eval_expression(&**if_branch, env)
                    } else {
                        eval_expression(&**else_branch, env)
                    }
                }
                _ => Err(format!("Expected number in condition, got {:#?}", cond)),
            }
        }
        Expression::SExpression(head, tail) => {
            let args = tail
                .iter()
                .map(|v| eval_expression(v, env))
                .collect::<Result<Vec<Value>, String>>()?;
            match &**head {
                Expression::Identifier(s) => match s.as_str() {
                    "+" => args
                        .iter()
                        .fold(Ok(Value::Integer(0)), |acc, x| match (x, acc) {
                            (Value::Integer(n), Ok(Value::Integer(n2))) => {
                                Ok(Value::Integer(*n + n2))
                            }
                            (Value::Integer(n), Ok(Value::Number(n2))) => {
                                Ok(Value::Number(*n as f64 + n2))
                            }
                            (Value::Number(n), Ok(Value::Integer(n2))) => {
                                Ok(Value::Number(*n + n2 as f64))
                            }
                            (Value::Number(n), Ok(Value::Number(n2))) => Ok(Value::Number(*n + n2)),
                            (v, _) => Err(format!("{:#?} is not a number!", v)),
                        }),
                    "*" => args
                        .iter()
                        .fold(Ok(Value::Integer(1)), |acc, x| match (x, acc) {
                            (Value::Integer(n), Ok(Value::Integer(n2))) => {
                                Ok(Value::Integer(*n * n2))
                            }
                            (Value::Integer(n), Ok(Value::Number(n2))) => {
                                Ok(Value::Number(*n as f64 * n2))
                            }
                            (Value::Number(n), Ok(Value::Integer(n2))) => {
                                Ok(Value::Number(*n * n2 as f64))
                            }
                            (Value::Number(n), Ok(Value::Number(n2))) => Ok(Value::Number(*n * n2)),
                            (v, _) => Err(format!("{:#?} is not a number!", v)),
                        }),
                    "int" => Ok(Value::Bool(args.iter().all(|x| match x {
                        Value::Number(n) => *n == n.floor(),
                        Value::Integer(_) => true,
                        _ => false,
                    }))),
                    "-" => {
                        if args.len() == 1 {
                            match args[0] {
                                Value::Number(n) => Ok(Value::Number(-n)),
                                Value::Integer(n) => Ok(Value::Integer(-n)),
                                _ => Err(format!("{:#?} is not a number!", args[0])),
                            }
                        } else {
                            assert!(matches!(args[0], Value::Number(_) | Value::Integer(_)));
                            args.iter()
                                .skip(1)
                                .fold(Ok(args[0].clone()), |acc, x| match (x, acc) {
                                    (Value::Integer(n), Ok(Value::Integer(n2))) => {
                                        Ok(Value::Integer(n2 - *n))
                                    }
                                    (Value::Integer(n), Ok(Value::Number(n2))) => {
                                        Ok(Value::Number(n2 - *n as f64))
                                    }
                                    (Value::Number(n), Ok(Value::Integer(n2))) => {
                                        Ok(Value::Number(n2 as f64 - *n))
                                    }
                                    (Value::Number(n), Ok(Value::Number(n2))) => {
                                        Ok(Value::Number(n2 - *n))
                                    }
                                    (v, _) => Err(format!("{:#?} is not a number!", v)),
                                })
                        }
                    }
                    "/" => {
                        if args.len() == 1 {
                            match args[0] {
                                Value::Number(n) => Ok(Value::Number(1.0 / n)),
                                Value::Integer(n) => Ok(Value::Number(1.0 / n as f64)),
                                _ => Err(format!("{:#?} is not a number!", args[0])),
                            }
                        } else {
                            assert!(matches!(args[0], Value::Number(_) | Value::Integer(_)));
                            args.iter()
                                .skip(1)
                                .fold(Ok(args[0].clone()), |acc, x| match (x, acc) {
                                    (Value::Integer(n), Ok(Value::Integer(n2))) => {
                                        Ok(Value::Number(n2 as f64 / *n as f64))
                                    }
                                    (Value::Integer(n), Ok(Value::Number(n2))) => {
                                        Ok(Value::Number(n2 / *n as f64))
                                    }
                                    (Value::Number(n), Ok(Value::Integer(n2))) => {
                                        Ok(Value::Number(n2 as f64 / *n))
                                    }
                                    (Value::Number(n), Ok(Value::Number(n2))) => {
                                        Ok(Value::Number(n2 / *n))
                                    }
                                    (v, _) => Err(format!("{:#?} is not a number!", v)),
                                })
                        }
                    }
                    "<" => args
                        .iter()
                        .skip(1)
                        .fold(Ok(args[0].clone()), |acc, x| match (acc, x) {
                            (Ok(Value::Number(n2)), Value::Number(n)) => {
                                if n2 == 0.0 {
                                    Ok(Value::Number(0.0))
                                } else if n2 < *n {
                                    Ok(Value::Number(*n))
                                } else {
                                    Ok(Value::Number(0.0))
                                }
                            }
                            (Ok(Value::Integer(n2)), Value::Number(n)) => {
                                if n2 == 0 {
                                    Ok(Value::Number(0.0))
                                } else if (n2 as f64) < *n {
                                    Ok(Value::Number(*n))
                                } else {
                                    Ok(Value::Number(0.0))
                                }
                            }
                            (Ok(Value::Number(n2)), Value::Integer(n)) => {
                                if n2 == 0.0 {
                                    Ok(Value::Number(0.0))
                                } else if n2 < *n as f64 {
                                    Ok(Value::Number(*n as f64))
                                } else {
                                    Ok(Value::Number(0.0))
                                }
                            }
                            (Ok(Value::Integer(n2)), Value::Integer(n)) => {
                                if n2 == 0 {
                                    Ok(Value::Integer(0))
                                } else if n2 < *n {
                                    Ok(Value::Integer(*n))
                                } else {
                                    Ok(Value::Integer(0))
                                }
                            }
                            _ => Err(format!("{:#?} not a number!", x)),
                        })
                        .map(|val| match val {
                            Value::Number(n) => {
                                if n == 0.0 {
                                    Value::Bool(false)
                                } else {
                                    Value::Bool(true)
                                }
                            }
                            Value::Integer(0) => Value::Bool(false),
                            Value::Integer(_) => Value::Bool(true),
                            _ => unreachable!(),
                        }),
                    "=" => Ok(Value::Bool(args.iter().skip(1).all(|v| {
                        match (v, args[0].clone()) {
                            (Value::Number(n), Value::Number(n2)) => *n == n2,
                            (Value::Number(n), Value::Integer(n2)) => *n == n2 as f64,
                            (Value::Integer(n), Value::Number(n2)) => *n as f64 == n2,
                            (Value::Integer(n), Value::Integer(n2)) => *n == n2,
                            _ => false,
                        }
                    }))),
                    "not" => {
                        if args.len() != 1 {
                            Err(format!("Expected 1 arg, got {:#?}", args))
                        } else {
                            Ok(Value::Bool(match args[0] {
                                Value::Bool(b) => !b,
                                _ => false,
                            }))
                        }
                    }
                    "display" => {
                        if args.len() == 1 {
                            println!("{}", args[0]);
                            Ok(Value::Nil)
                        } else {
                            Err(format!(
                                "Expected one argument to `display`, got {:#?}",
                                args
                            ))
                        }
                    }
                    "cons" => {
                        if args.len() != 2 {
                            Err(format!("Expected two arguments to `cons`, got {:#?}", args))
                        } else {
                            Ok(Value::Pair(
                                Box::new(args[0].clone()),
                                Box::new(args[1].clone()),
                            ))
                        }
                    }
                    "car" => {
                        if args.len() != 1 {
                            Err(format!("Expected one argument to `car`, got {:#?}", args))
                        } else {
                            match args[0].clone() {
                                Value::Pair(a, _) => Ok(*a),
                                _ => Err(format!("{:#?} not a pair!", args[0])),
                            }
                        }
                    }
                    "cdr" => {
                        if args.len() != 1 {
                            Err(format!("Expected one argument to `cdr`, got {:#?}", args))
                        } else {
                            match &args[0] {
                                Value::Pair(_, b) => Ok(*b.clone()),
                                _ => Err(format!("{:#?} not a pair!", args[0])),
                            }
                        }
                    }
                    "list" => Ok(args.iter().rfold(Value::Nil, |acc, x| {
                        Value::Pair(Box::new(x.clone()), Box::new(acc))
                    })),
                    "null?" => {
                        if args.len() != 1 {
                            Err(format!("Expected one argument to `null?`, got {:#?}", args))
                        } else {
                            Ok(match &args[0] {
                                Value::Nil => Value::Bool(true),
                                _ => Value::Bool(false),
                            })
                        }
                    }
                    s => {
                        let f = env.get(s).ok_or(format!("Symbol {} not found!", s))?;
                        if let Value::Function(params, body) = f {
                            let mut map = env.clone();
                            assert!(args.len() <= params.len());
                            for (i, arg) in args.iter().enumerate() {
                                map.insert(params[i].clone(), arg.clone());
                            }
                            eval_expression(&body, &mut map)
                        } else {
                            Err(format!("{} is not a known function!", s))
                        }
                    }
                },
                _ => {
                    let res = eval_expression(&**head, env)?;
                    match res {
                        Value::Function(params, body) => {
                            let mut map = env.clone();
                            assert!(args.len() <= params.len());
                            for (i, arg) in args.iter().enumerate() {
                                map.insert(params[i].clone(), arg.clone());
                            }
                            eval_expression(&body, &mut map)
                        }
                        Value::Number(n) => Err(format!("{} is a number, not a function!", n)),
                        Value::Nil => Err(format!("Nil is not callable!")),
                        Value::Bool(b) => Err(format!("{} is a boolean, not a function!", b)),
                        Value::Integer(n) => Err(format!("{} is an integer, not a function!", n)),
                        Value::Pair(_, _) => Err(format!("Expected a function, got a pair!")),
                    }
                }
            }
        }
    }
}
