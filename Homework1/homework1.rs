pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
// Div
}

pub enum Token {
    Operator(Operator), //is an enum in itself
    Operand(isize),
}
pub fn eval(tokens: &[Token] -> Option<isize>){
    if tokens.len() == 0 { //means vector is empty
        Some(0);
    } else{
        let mut s: Vec<isize> = Vec::new(); //stack
        let mut sum: isize = 0;
        let

        for token in tokens.into_iter(){
            match *tok {
                Token::Operand(i) => s.push(i),
                Token::Operator(ref op) => match Operator{
                    Operator::Add => //x + y
                    Operator::Sub => //x - y
                    Operator::Mul => //x * y
                }
            }
        }
        if () {
            return None;
        } else{
        Some(s[0]);
        }

    }
 }


#[cfg(test)]
mod tests{
    use super::*
    #[test]
    fn it_works(){

    }
}

fn valid_works(){
    let mut_stack = Vec::new();
    stack.push(Token::Operand(1));
    stack.push(Token::Operand(2));
    stack.push(Token::Operator(Add));
    let option_x = eval(&stack);
    let x: isize;
    match option_x {
        Some(a) => x = a,
        None =>,
    }
    assert_eq!(x, -100)
}




let mut counteroperand = 0;  //counter for number of operands
let mut counteroperator = 0;  //counter for number of operators
/*
for token in tokens.into_iter(){
    match *tok {
        Token::Operand(i) => counteroperand + 1,
        Token::Operator(ref op) => counteroperator + 1;
    }
}
*/
