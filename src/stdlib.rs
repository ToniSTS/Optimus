use crate::analyzer::RuntimeValue;

pub fn call_stdlib_function(name: &str, args: Vec<RuntimeValue>) -> Result<RuntimeValue, String> {
    match name {
        "max" => {
            if args.len() != 2 {
                return Err("max expects 2 arguments".to_string());
            }

            match (&args[0], &args[1]) {
                (RuntimeValue::Int(a), RuntimeValue::Int(b)) => {
                    Ok(RuntimeValue::Int(std::cmp::max(*a, *b)))
                }
                _ => Err("max only supports int values".to_string()),
            }
        }

        "min" => {
            if args.len() != 2 {
                return Err("min expects 2 arguments".to_string());
            }

            match (&args[0], &args[1]) {
                (RuntimeValue::Int(a), RuntimeValue::Int(b)) => {
                    Ok(RuntimeValue::Int(std::cmp::min(*a, *b)))
                }
                _ => Err("min only supports int values".to_string()),
            }
        }

        "abs" => {
            if args.len() != 1 {
                return Err("abs expects 1 argument".to_string());
            }

            match &args[0] {
                RuntimeValue::Int(n) => Ok(RuntimeValue::Int(n.abs())),
                _ => Err("abs only supports int values".to_string()),
            }
        }

        "typeOf" => {
            if args.len() != 1 {
                return Err("typeOf expects 1 argument".to_string());
            }

            Ok(RuntimeValue::Str(args[0].type_name()))
        }

        _ => Err(format!("unknown stdlib function '{}'", name)),
    }
}
