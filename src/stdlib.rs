use crate::ast::Literal;

pub fn call(function: &str, args: &[Literal]) -> Result<Literal, String> {
    match function {
        "len" => match args {
            [Literal::Str(s)] => Ok(Literal::Int(s.chars().count() as i64)),
            [_] => Err("len() expects a string".to_string()),
            _ => Err("len() expects exactly 1 argument".to_string()),
        },
        "sqrt" => match args {
            [value] => Ok(Literal::Float(as_f64(value)?.sqrt())),
            _ => Err("sqrt() expects exactly 1 numeric argument".to_string()),
        },
        "abs" => match args {
            [Literal::Int(v)] => Ok(Literal::Int(v.abs())),
            [value] => Ok(Literal::Float(as_f64(value)?.abs())),
            _ => Err("abs() expects exactly 1 numeric argument".to_string()),
        },
        "max" => match args {
            [left, right] => Ok(number_result(left, right, as_f64(left)?.max(as_f64(right)?))),
            _ => Err("max() expects exactly 2 numeric arguments".to_string()),
        },
        "min" => match args {
            [left, right] => Ok(number_result(left, right, as_f64(left)?.min(as_f64(right)?))),
            _ => Err("min() expects exactly 2 numeric arguments".to_string()),
        },
        "pow" => match args {
            [left, right] => Ok(Literal::Float(as_f64(left)?.powf(as_f64(right)?))),
            _ => Err("pow() expects exactly 2 numeric arguments".to_string()),
        },
        "type_of" => match args {
            [Literal::Int(_)] => Ok(Literal::Str("int".to_string())),
            [Literal::Float(_)] => Ok(Literal::Str("float".to_string())),
            [Literal::Str(_)] => Ok(Literal::Str("string".to_string())),
            [Literal::Bool(_)] => Ok(Literal::Str("bool".to_string())),
            _ => Err("type_of() expects exactly 1 argument".to_string()),
        },
        _ => Err(format!("Unknown standard library function: {}", function)),
    }
}

fn as_f64(value: &Literal) -> Result<f64, String> {
    match value {
        Literal::Int(v) => Ok(*v as f64),
        Literal::Float(v) => Ok(*v),
        _ => Err("Expected numeric argument".to_string()),
    }
}

fn number_result(left: &Literal, right: &Literal, value: f64) -> Literal {
    if matches!(left, Literal::Int(_)) && matches!(right, Literal::Int(_)) && value.fract() == 0.0 {
        Literal::Int(value as i64)
    } else {
        Literal::Float(value)
    }
}
