use crate::parser::Expression;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    // TODO: add more types. Bool? Int? String? Pair? Vector? Char? Symbol?
    Number(f64),
    Function(Vec<String>, Expression),
    Nil,
}

fn check_environment(expr: Expression, env: &HashMap<String, Value>) -> Option<Expression> {
    match expr {
        Expression::Identifier(s) => match env.get(&s) {
            Some(Value::Number(n)) => Some(Expression::Number(*n)),
            Some(Value::Function(params, body)) => {
                Some(Expression::Lambda(params.clone(), Box::new(body.clone())))
            }
            Some(Value::Nil) => Some(Expression::Nil),
            None => Some(Expression::Identifier(s.clone())),
        },
        Expression::Lambda(params, body) => Some(Expression::Lambda(
            params
                .iter()
                .cloned()
                .filter(|p| !env.contains_key(p))
                .collect(),
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
pub fn eval_expression(
    expr: &Expression,
    env: &mut HashMap<String, Value>,
) -> Result<Value, String> {
    match expr {
        Expression::Number(n) => Ok(Value::Number(*n)),
        Expression::Identifier(s) => env
            .get(s)
            .cloned()
            .ok_or(format!("Variable {} not in environment!", s)),
        Expression::Nil => Ok(Value::Nil),
        Expression::Lambda(params, body) => Ok(Value::Function(
            params.clone(),
            check_environment(*body.clone(), env).ok_or("Check environment failed!")?,
        )),
        Expression::Define(s, expr) => {
            let res = eval_expression(expr, env)?;
            env.insert(s.clone(), res);
            Err(format!(""))
        }
        Expression::If(cond, if_branch, else_branch) => {
            let cond = eval_expression(&**cond, env)?;
            match cond {
                Value::Number(n) => {
                    if n != 0.0 {
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
                .map(|v| {
                    eval_expression(v, env)
                })
                .collect::<Result<Vec<Value>, String>>()?;
            match &**head {
                Expression::Identifier(s) => {
                    // TODO: more operators?
                    match s.as_str() {
                        "+" => args.iter().fold(Ok(Value::Number(0.0)), |acc, x| {
                            if let Value::Number(n) = x {
                                if let Ok(Value::Number(n2)) = acc {
                                    return Ok(Value::Number(n2 + *n));
                                }
                            }
                            Err(format!("{:#?} is not a number!", x))
                        }),
                        "*" => args.iter().fold(Ok(Value::Number(1.0)), |acc, x| {
                            if let Value::Number(n) = x {
                                if let Ok(Value::Number(n2)) = acc {
                                    return Ok(Value::Number(n2 * *n));
                                }
                            }
                            Err(format!("{:#?} is not a number!", x))
                        }),
                        "int" => Ok(Value::Number(
                            if args.iter().all(|x| match x {
                                Value::Number(n) => *n == n.floor(),
                                _ => false,
                            }) {
                                1.0
                            } else {
                                0.0
                            },
                        )),
                        "-" => {
                            if args.len() == 1 {
                                if let Value::Number(n) = args[0] {
                                    Ok(Value::Number(-n))
                                } else {
                                    Err(format!("{:#?} not a number!", args[0]))
                                }
                            } else {
                                assert!(matches!(args[0], Value::Number(_)));
                                args.iter().skip(1).fold(Ok(args[0].clone()), |acc, x| {
                                    if let Value::Number(n) = x {
                                        if let Ok(Value::Number(n2)) = acc {
                                            Ok(Value::Number(n2 - *n))
                                        } else {
                                            Err(format!("{:#?} is not a number!", x))
                                        }
                                    } else {
                                        Err(format!("{:#?} is not a number!", x))
                                    }
                                })
                            }
                        }
                        "/" => {
                            if args.len() == 1 {
                                if let Value::Number(n) = args[0] {
                                    Ok(Value::Number(1.0 / n))
                                } else {
                                    Err(format!("{:#?} not a number!", args[0]))
                                }
                            } else {
                                assert!(matches!(args[0], Value::Number(_)));
                                args.iter().skip(1).fold(Ok(args[0].clone()), |acc, x| {
                                    if let Value::Number(n) = x {
                                        if let Ok(Value::Number(n2)) = acc {
                                            Ok(Value::Number(n2 / *n))
                                        } else {
                                            Err(format!("{:#?} is not a number!", x))
                                        }
                                    } else {
                                        Err(format!("{:#?} is not a number!", x))
                                    }
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
                                _ => Err(format!("{:#?} not a number!", x)),
                            })
                            .map(|val| match val {
                                Value::Number(n) => {
                                    if n == 0.0 {
                                        val
                                    } else {
                                        Value::Number(1.0)
                                    }
                                }
                                _ => unreachable!(),
                            }),
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
                    }
                }
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
                    }
                }
            }
        }
    }
}
