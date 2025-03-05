use rand::prelude::*;

fn generate_math_problem() -> String {
    let op = ["+", "-", "*", "/"];
    let num1 = thread_rng().gen_range(0, 10);
    let num2 = thread_rng().gen_range(0, 10);
    let operation = op[thread_rng().gen_range(0, 4)];
    return format!("{} {} {}", num1, operation, num2);
}
