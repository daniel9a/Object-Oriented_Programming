//8th submission 19 points

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;
        let mut stat = 0;
        let mut switch = 0; //RightParen
        let mut length = 0; //LeftParen
        let mut number = 0;
        let mut sign = 0;
        let mut error = 0;

//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){

            match *token {
                InfixToken::Operand(i) => if stat == 0{
                        number = number + 1;
                        output.push(PostfixToken::Operand(i));
                    },
                InfixToken::LeftParen => if stat % 2 == 0 {

                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                        length = length + 1;
                    } else {
                        return None;
                    },
                InfixToken::RightParen => /*while stack.len() !=0 {
                //    switch = switch + 1;
                    let top = stack.pop().unwrap();
                    match top {
                        InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                        InfixToken::LeftParen => break,  //break //{}
                        _ => {},
                    }
                },*/if length != switch { //lenght >= swithc ?
                    switch = switch + 1;
                    while stack.len() !=0 {
                        let top = stack.pop().unwrap();
                        match top {
                            InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                            InfixToken::LeftParen => break,  //break //{}
                            _ => {},
                        }
                    }
                } else if length =< switch {
                    return None;
                },
                /*if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        while stack.len() !=0 {
                            switch = switch + 1;
                            let top = stack.pop().unwrap();
                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                InfixToken::LeftParen => {},  //break
                                _ => {},
                            }
                        }
                    },*/
                InfixToken::Operator(ref op) => if stack.len() == 0 {//|| length == 1 { //&& switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        sign = sign + 1;
                        stack.push(InfixToken::Operator(*op));

                    } else if output.len() < 1 { //if stack.len() == 1 && switch % 2 == 0 {
                        return None;
                    } else {

                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }
                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }
                            sign = sign + 1;
                            stack.push(InfixToken::Operator(*op));

                        } else {
                            match *op {
                                Operator::Add | Operator::Sub => weight1 = 1,
                                Operator::Mul | Operator::Div => weight1 = 2,
                            }
                            sign = sign + 1;
                            stack.push(InfixToken::Operator(*op));
                        }
/*
                    } else {
                        return None;
                    }
*/
                    }

            }
        //    write!(f, )
        }
        if length != switch{ //checks for matching parentheses
            return None;
        }
        if sign >= number {
            return None;
        }

        if stack.len() != 0 {  //if any remaining values on stack
    //        for tokens in 0..stack.len() {
            while stack.len() != 0 {
                let top = stack.pop().unwrap();

                match top {
                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
            //        InfixToken::LeftParen => return None, //{}
            //        InfixToken::RightParen => return None,
                    _ => {},
                }

            }
        }
        if output.len() <= 2 {
            return None;
        }


        return Some(output);
    }
}

Test 0 failed.
Test 1 failed.
Test 10 failed.
Test 12 succeeded.
Test 15 failed.
Test 11 failed.
Test 13 succeeded.
Test 17 succeeded.
Test 16 succeeded.
Test 18 succeeded.
Test 23 succeeded.
Test 24 succeeded.
Test 19 succeeded.
Test 14 succeeded.
Test 22 succeeded.
Test 21 succeeded.
Test 2 succeeded.
Test 20 succeeded.
Test 6 succeeded.
Test 7 succeeded.
Test 4 succeeded.
Test 8 failed.
Test 9 succeeded.
Test 3 succeeded.
Test 5 succeeded.




//6th SUBMISSION    18 points

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;
        let mut stat = 0;
        let mut switch = 0; //RightParen
        let mut length = 0; //LeftParen
        let mut number = 0;
        let mut sign = 0;

//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){

            match *token {
                InfixToken::Operand(i) => if stat == 0{
                        number = number + 1;
                        output.push(PostfixToken::Operand(i));
                    },
                InfixToken::LeftParen => if switch % 2 == 0{
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                        length = length + 1;
                    } else {
                        return None;
                    },
                InfixToken::RightParen => /*while stack.len() !=0 {
                //    switch = switch + 1;
                    let top = stack.pop().unwrap();
                    match top {
                        InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                        InfixToken::LeftParen => break,  //break //{}
                        _ => {},
                    }
                },*/if length != 0 {
                    switch = switch + 1;
                    while stack.len() !=0 {
                        let top = stack.pop().unwrap();
                        match top {
                            InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                            InfixToken::LeftParen => break,  //break //{}
                            _ => {},
                        }
                    }
                } else if length == 0 {
                    return None;
                },
                /*if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        while stack.len() !=0 {
                            switch = switch + 1;
                            let top = stack.pop().unwrap();
                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                InfixToken::LeftParen => {},  //break
                                _ => {},
                            }
                        }
                    },*/
                InfixToken::Operator(ref op) => if stack.len() == 0 {//|| length == 1 { //&& switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        sign = sign + 1;
                        stack.push(InfixToken::Operator(*op));

                    } else if output.len()  < 1 { //if stack.len() == 1 && switch % 2 == 0 {
                        return None;
                    } else {

                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }
                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }
                            sign = sign + 1;
                            stack.push(InfixToken::Operator(*op));

                        } else {
                            match *op {
                                Operator::Add | Operator::Sub => weight1 = 1,
                                Operator::Mul | Operator::Div => weight1 = 2,
                            }
                            sign = sign + 1;
                            stack.push(InfixToken::Operator(*op));
                        }
/*
                    } else {
                        return None;
                    }
*/
                    }

            }
        //    write!(f, )
        }
        if length != switch{ //checks for matching parentheses
            return None;
        }
        if sign >= number {
            return None;
        }
        if stack.len() != 0 {  //if any remaining values on stack
    //        for tokens in 0..stack.len() {
            while stack.len() != 0 {
                let top = stack.pop().unwrap();

                match top {
                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                //    InfixToken::LeftParen => {},//return None, //{}
                //    InfixToken::RightParen => return None,
                    _ => {},
                }

            }
        }
        if output.len() <= 2 {
            return None;
        }


        return Some(output);
    }
}










//5th SUBMISSION
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;
        let mut switch = 0;
        let mut length = 0;
//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){

            match *token {
                InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
                InfixToken::LeftParen => if switch % 2 == 0{
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                        length = length + 1;
                    },/* else {
                        return None;
                    },*/
                InfixToken::RightParen => while stack.len() !=0 {
                //    switch = switch + 1;
                    let top = stack.pop().unwrap();
                    match top {
                        InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                        InfixToken::LeftParen => break,  //break //{}
                        _ => {},
                    }
                }, /*if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        while stack.len() !=0 {
                            switch = switch + 1;
                            let top = stack.pop().unwrap();
                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                InfixToken::LeftParen => {},  //break
                                _ => {},
                            }
                        }
                    },*/
                InfixToken::Operator(ref op) => if stack.len() == 0 {//|| length == 1 { //&& switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        stack.push(InfixToken::Operator(*op));

                    } else{ //if stack.len() == 1 && switch % 2 == 0 {

                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }
                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }
                            stack.push(InfixToken::Operator(*op));

                        } else {
                            match *op {
                                Operator::Add | Operator::Sub => weight1 = 1,
                                Operator::Mul | Operator::Div => weight1 = 2,
                            }
                            stack.push(InfixToken::Operator(*op));
                        }
/*
                    } else {
                        return None;
                    }
*/
                    }

            }
        //    write!(f, )
        }
        if stack.len() != 0 {  //if any remaining values on stack
            for tokens in 0..stack.len() {
        //    while stack.len() != 0 {
                let top = stack.pop().unwrap();

                match top {
                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                    InfixToken::LeftParen => {},//return None, //{}
                //    InfixToken::RightParen => return None,
                    _ => {},
                }

            }
        }
        return Some(output);
    }
}


Test 10 failed.
Test 1 succeeded.
Test 0 succeeded.
Test 11 failed.
Test 12 failed.
Test 16 failed.
Test 18 failed.
Test 15 failed.
Test 19 failed.
Test 13 failed.
Test 17 failed.
Test 2 succeeded.
Test 20 failed.
Test 14 failed.
Test 23 failed.
Test 24 failed.
Test 22 failed.
Test 6 succeeded.
Test 4 succeeded.
Test 9 succeeded.
Test 5 succeeded.
Test 21 failed.
Test 3 succeeded.
Test 8 failed.
Test 7 succeeded.







//fourth SUBMISSION
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;
        let mut switch = 0;
        let mut length = 0;

        for token in tokens.into_iter(){

            match *token {
                InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
                InfixToken::LeftParen => if switch % 2 == 0{
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                        length = length + 1;
                    } else {
                        return None;
                    },
                InfixToken::RightParen => if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        while stack.len() !=0 {
                            let top = stack.pop().unwrap();
                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                InfixToken::LeftParen => {},  //break
                                _ => {},
                            }
                        }
                    },
                InfixToken::Operator(ref op) => if stack.len() == 0 || length == 1 { //&& switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        stack.push(InfixToken::Operator(*op));

                    } else{

                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }
                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }
                            stack.push(InfixToken::Operator(*op));

                        } else {
                            stack.push(InfixToken::Operator(*op));
                        }

                    }

            }
        //    write!(f, )
        }
        if stack.len() != 0 {  //if any remaining values on stack
            for tokens in 0..stack.len() {
        //    while stack.len() != 0 {
                let top = stack.pop().unwrap();

                match top {
                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                    InfixToken::LeftParen => {},//return None, //{}
                    _ => {},
                }

            }
        }
        return Some(output);
    }
}


Test 0 succeeded.
Test 1 succeeded.
Test 14 succeeded.
Test 13 failed.
Test 10 failed.
Test 11 failed.
Test 15 failed.
Test 12 succeeded.
Test 18 failed.
Test 16 succeeded.
Test 17 failed.
Test 19 failed.
Test 2 succeeded.
Test 3 failed.
Test 24 failed.
Test 23 failed.
Test 7 succeeded.
Test 6 succeeded.
Test 9 succeeded.
Test 20 failed.
Test 21 failed.
Test 22 failed.
Test 5 failed.
Test 8 failed.
Test 4 failed.









/*#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;
        let mut switch = 0;
//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){
            match *token {
                InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
                InfixToken::LeftParen => if switch % 2 == 0{
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                    } else {
                        return None;
                    },
                InfixToken::RightParen => if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        let top = stack.pop().unwrap(); //top value on stack

                        if top == InfixToken::LeftParen { //if top of stack is a LeftParen then none
                            return None;
                        } else {
                            switch = switch + 1;
                            while stack.len() != 0 {  //loop continues popping options and outputing them until the stack is empty
                                let top = stack.pop().unwrap();

                                match top {
                                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)), //if top of stack is Operator then output it
                                    InfixToken::LeftParen => {}, //if top of stack is LeftParen then discard it
                                    _ => {},
                                }
                            }
                        }
                    },
                InfixToken::Operator(ref op) => if stack.len() == 0 && switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        stack.push(InfixToken::Operator(*op));

                    } else if stack.len() == 1 && switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }
                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }
                            stack.push(InfixToken::Operator(*op));

                        } else {
                            stack.push(InfixToken::Operator(*op));
                        }

                    } else {
                        return None;
                    }


            }
        }
        if stack.len() != 0 {  //if any remaining values on stack
            for tokens in 0..stack.len() {
        //while stack.len() != 0 {
                let top = stack.pop().unwrap();

                match top {
                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                    InfixToken::LeftParen => {},//return None, //{}
                    InfixToken::RightParen => return None,
                    _ => {},
                }

            }
        }

        return Some(output);
    }
}
*/


















//SECOND SUBMISSION

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;
        let mut switch = 0;
//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){
            match *token {
                InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
                InfixToken::LeftParen => if switch % 2 == 0{
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                    } else {
                        return None;
                    },
                InfixToken::RightParen => if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        let top = stack.pop().unwrap(); //top value on stack

                        if top == InfixToken::LeftParen { //if top of stack is a LeftParen then none
                            return None;
                        } else {
                            switch = switch + 1;
                            while stack.len() != 0 {  //loop continues popping options and outputing them until the stack is empty
                                let top = stack.pop().unwrap();

                                match top {
                                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)), //if top of stack is Operator then output it
                                    InfixToken::LeftParen => {}, //if top of stack is LeftParen then discard it
                                    _ => {},
                                }
                            }
                        }
                    },
                InfixToken::Operator(ref op) => if stack.len() == 0 && switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }

                        stack.push(InfixToken::Operator(*op));

                    } else if stack.len() == 1 && switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }

                    } else if switch % 2 == 0 {

                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }

                        } else {
                            stack.push(InfixToken::Operator(*op));
                        }
                    } else {
                        return None;
                    }


            }
        }
        return Some(output);
    }
}

Test 0 succeeded.
Test 1 failed.
Test 10 failed.
Test 12 succeeded.
Test 11 failed.
Test 16 succeeded.
Test 14 succeeded.
Test 17 failed.
Test 13 succeeded.
Test 15 failed.
Test 2 failed.
Test 19 succeeded.
Test 22 failed.
Test 21 failed.
Test 18 succeeded.
Test 23 failed.
Test 24 failed.
Test 20 succeeded.
Test 5 failed.
Test 4 failed.
Test 7 failed.
Test 6 failed.
Test 3 failed.
Test 9 failed.
Test 8 failed.










/*
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}
/*
impl Operator {
    fn precedence(&self) -> i32 {
        match self {
            Add => 1,
            Sub => 1,
            Mul => 2,
            Div => 2,
        }
    }
}
*/
#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = 0;
        let mut weight2 = 0;
        let mut switch = 0;
        let mut switch2 = 0;
//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){
            match *token {
                InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
                InfixToken::LeftParen => if switch % 2 == 0{
                        switch2 = switch2 + 1;
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                    } else {
                        return None;
                    },
                InfixToken::RightParen => if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        let top = stack.pop().unwrap(); //top value on stack

                        if top == InfixToken::LeftParen { //if top of stack is a LeftParen then none
                            return None;
                        } else {
                            switch = switch + 1;
                            while stack.len() != 0 {  //loop continues popping options and outputing them until the stack is empty
                                let top = stack.pop().unwrap();

                                match top {
                                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)), //if top of stack is Operator then output it
                                //    InfixToken::LeftParen => {}, //if top of stack is LeftParen then discard it
                                    _ => {},
                                }
                            }
                        }
                    },
                InfixToken::Operator(ref op) => if stack.len() == 0 && switch % 2 == 0 || stack.pop().unwrap() == InfixToken::LeftParen{
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        stack.push(InfixToken::Operator(*op));

                    } else if stack.len() == 1 && switch % 2 == 0 {
                        match *op {
                            Operator::Add | Operator::Sub => weight2 = 1,
                            Operator::Mul | Operator::Div => weight2 = 2,
                        }
                        if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                            let top = stack.pop().unwrap();

                            match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                            }
                            stack.push(InfixToken::Operator(*op));
                        } else {
                            stack.push(InfixToken::Operator(*op));
                        }

                    } else {
                        return None;
                    }


            }
        }
        if stack.len() != 0{  //if any remaining values on stack
            for tokens in 0..stack.len() {
        //while stack.len() != 0 {
                let top = stack.pop().unwrap();

                match top {
                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                    InfixToken::LeftParen => {},//return None, //{}
                    InfixToken::RightParen => return None,
                    _ => {},
                }

            }
        }
        return Some(output);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
/*
        let result = vec![PostfixToken::Operand(2), PostfixToken::Operand(3), PostfixToken::Operator(Operator::Mul), PostfixToken::Operand(1), PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::Operand(2), InfixToken::Operator(Operator::Mul), InfixToken::Operand(3), InfixToken::Operator(Operator::Add), InfixToken::Operand(1)]));
*/
        let result = vec![PostfixToken::Operand(2), PostfixToken::Operand(3), PostfixToken::Operator(Operator::Mul)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen, InfixToken::Operand(2), InfixToken::Operator(Operator::Mul), InfixToken::Operand(3), InfixToken::RightParen]));
    }
}
/*



/*
//FIRST SUBMISSION

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    if tokens.len() == 0 {
        return None;
    } else {

        let mut output: Vec<PostfixToken> = Vec::new();
        let mut stack: Vec<InfixToken> = Vec::new();
        let mut weight1 = -1;
        let mut weight2 = -1;


        for token in tokens.into_iter(){
            match *token {
                InfixToken::Operand(n) => output.push(PostfixToken::Operand(n)),
                InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
                InfixToken::RightParen => if stack.len() == 0 {
                        return None;
                } else {
                    while !stack.len() == 0 {
                        let top = stack.pop().unwrap();

                        match top {
                            InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                            InfixToken::LeftParen => {},
                            _ => {},
                        }
                    }
                },
                InfixToken::Operator(ref op) => if stack.len() == 0 {
                    match *op {
                        Operator::Add | Operator::Sub => weight2 = 1,
                        Operator::Mul | Operator::Div => weight2 = 2,
                    }

                    stack.push(InfixToken::Operator(*op));

                } else if stack.len() == 1 {
                    match *op {
                        Operator::Add | Operator::Sub => weight2 = 1,
                        Operator::Mul | Operator::Div => weight2 = 2,
                    }

                } else{
                    if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

                        let top = stack.pop().unwrap();

                        match top {
                                InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
                                _ => {},
                        }

                    } else {
                        stack.push(InfixToken::Operator(*op));
                    }
                }


            }
        }
        return Some(output);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

*/
