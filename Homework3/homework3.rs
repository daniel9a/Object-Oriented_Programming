#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
  //'+'
  Add,
  //'-'
  Sub,
  //'*'
  Mul,
  //'/'
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




pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {

if tokens.len() == 0 {
    return None;
} else {
    let mut output: Vec<PostfixToken> = Vec::new();
    let mut stack : Vec<InfixToken> = Vec::new();
    for (a,b) in tokens.iter().enumerate(){
        match b {
            &InfixToken::Operator(i) => operatorCheck(i, &mut stack, &mut output),
            &InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
            &InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
            &InfixToken::RightParen => parenCheck(&mut stack, &mut output),
        };

        let mut check = 0;
        match b {
            &InfixToken::Operator(i) => check = check + 1,
            &InfixToken::Operand(i) => check = check + 2,
            &InfixToken::LeftParen => check = check + 3,
            &InfixToken::RightParen => check = check + 4,
        };


        if a != tokens.len() - 1
        {
            if check == 1
            {
                match tokens[a+1] {
                    InfixToken::Operator(i) => return None,
                    InfixToken::Operand(i) => continue,
                    InfixToken::LeftParen => continue,
                    InfixToken::RightParen => return None,
                };
            }

            else if check == 2
            {
                match tokens[a+1] {
                    InfixToken::Operator(i) => continue,
                    InfixToken::Operand(i) => return None,
                    InfixToken::LeftParen => return None,
                    InfixToken::RightParen => continue,
                };
            }

            else if check == 3
            {
                match tokens[a+1] {
                    InfixToken::Operator(i) => return None,
                    InfixToken::Operand(i) => continue,
                    InfixToken::LeftParen => continue,
                    InfixToken::RightParen => return None,
                };
            }

            else if check == 4
            {
                match tokens[a+1] {
                    InfixToken::Operator(i) => continue,
                    InfixToken::Operand(i) => return None,
                    InfixToken::LeftParen => return None,
                    InfixToken::RightParen => continue,
                };
            }
        }
    }
    while stack.len() > 0
    {
        let top = stack.pop().unwrap();
        let out = match top {
            InfixToken::Operator(n)=> n,
            _ => return None,
        };

        output.push(PostfixToken::Operator(out));

    }
    let mut num = 0;
    let mut sign = 0;

    for b in output.iter() {
        match b {
            &PostfixToken::Operator(i)=> sign = sign + 1,
            &PostfixToken::Operand(i)=> num = num + 1,
        };
    }

    if num == sign + 1 {
        Some(output)
    } else {
        None
    }
}
}


pub fn operatorCheck(op : Operator, stack: &mut Vec<InfixToken>,  output : &mut Vec<PostfixToken>) {
    let first = match op {
      Operator::Add => 1,
      Operator::Sub => 1,
      Operator::Mul => 2,
      Operator::Div => 2,
    };
    while stack.len() > 0 {
      let stacktop = stack.pop();
      if stacktop == None {
          stack.push(InfixToken::Operator(op));
          break;
        }


      let top = stacktop.unwrap();
      if top == InfixToken::LeftParen {
          stack.push(top);
          break;
      }
      let checkop = match top {
          InfixToken::Operator(o)=> o,
          _ => break,
      };

      let second = match checkop {
        Operator::Add => 1,
        Operator::Sub => 1,
        Operator::Mul => 2,
        Operator::Div => 2,
      };

      if second >= first {
        output.push(PostfixToken::Operator(checkop));

      }
      else {
        stack.push(top);
        break;
      }
    }
    stack.push(InfixToken::Operator(op));


}

pub fn parenCheck(stack: &mut Vec<InfixToken>, output : &mut Vec<PostfixToken>) {
    while stack.len() != 0 {
        let top = stack.pop().unwrap();
        match top {
            InfixToken::Operator(i) => output.push(PostfixToken::Operator(i)),
            InfixToken::LeftParen => return,
            InfixToken::RightParen => return,
            _ => {}, 
        }
    }
    stack.push(InfixToken::RightParen);
}


#[test]
fn valid1() {
    assert_eq!(
        infix_to_postfix(&[
            InfixToken::Operand(-1),
        ]), Some(vec![
            PostfixToken::Operand(-1)
        ])
    )
}
#[test]
fn valid2(){
    let x = &[InfixToken::Operand(1),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(2),
          ];
    let y = Some(vec![
             PostfixToken::Operand(1),
             PostfixToken::Operand(2),
             PostfixToken::Operator(Operator::Add)]);
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn valid3() {
    assert_eq!(
        infix_to_postfix(&[
            InfixToken::LeftParen,
            InfixToken::Operand(1),
            InfixToken::Operator(Operator::Add),
            InfixToken::Operand(2),
            InfixToken::RightParen,
        ]), Some(vec![
            PostfixToken::Operand(1),
            PostfixToken::Operand(2),
            PostfixToken::Operator(Operator::Add),
        ])
    )
}
#[test]
fn valid4() {
    //(2+2)*(3/4)+18+7
    //2 2 + 3 4 / * 18 + 7 +

    assert_eq!(
        infix_to_postfix(&[
            InfixToken::LeftParen,
            InfixToken::Operand(2),
            InfixToken::Operator(Operator::Add),
            InfixToken::Operand(2),
            InfixToken::RightParen,
            InfixToken::Operator(Operator::Mul),
            InfixToken::LeftParen,
            InfixToken::Operand(3),
            InfixToken::Operator(Operator::Div),
            InfixToken::Operand(4),
            InfixToken::RightParen,
            InfixToken::Operator(Operator::Add),
            InfixToken::Operand(18),
            InfixToken::Operator(Operator::Add),
            InfixToken::Operand(7),
        ]), Some(vec![
            PostfixToken::Operand(2),
            PostfixToken::Operand(2),
            PostfixToken::Operator(Operator::Add),
            PostfixToken::Operand(3),
            PostfixToken::Operand(4),
            PostfixToken::Operator(Operator::Div),
            PostfixToken::Operator(Operator::Mul),
            PostfixToken::Operand(18),
            PostfixToken::Operator(Operator::Add),
            PostfixToken::Operand(7),
            PostfixToken::Operator(Operator::Add),
        ])
    )
}
#[test]
fn valid4b() {

    //(2/2)-(3*4)/18+7
    //2 2 / 3 4 * 18 / + 7 -
    assert_eq!(
        infix_to_postfix(&[
            InfixToken::LeftParen,
            InfixToken::Operand(2),
            InfixToken::Operator(Operator::Div),
            InfixToken::Operand(2),
            InfixToken::RightParen,
            InfixToken::Operator(Operator::Sub),
            InfixToken::LeftParen,
            InfixToken::Operand(3),
            InfixToken::Operator(Operator::Mul),
            InfixToken::Operand(4),
            InfixToken::RightParen,
            InfixToken::Operator(Operator::Div),
            InfixToken::Operand(18),
            InfixToken::Operator(Operator::Add),
            InfixToken::Operand(7),
        ]), Some(vec![
            PostfixToken::Operand(2),
            PostfixToken::Operand(2),
            PostfixToken::Operator(Operator::Div),
            PostfixToken::Operand(3),
            PostfixToken::Operand(4),
            PostfixToken::Operator(Operator::Mul),
            PostfixToken::Operand(18),
            PostfixToken::Operator(Operator::Div),
            PostfixToken::Operator(Operator::Sub),
            PostfixToken::Operand(7),
            PostfixToken::Operator(Operator::Add),
        ])
    )
}
#[test]
fn valid5 (){
//INFIX: ((5))
//POSTFIX: 5
      let x = &[
          InfixToken::LeftParen,
          InfixToken::LeftParen,
          InfixToken::LeftParen,
          InfixToken::Operand(5),
          InfixToken::RightParen,
          InfixToken::RightParen,
          InfixToken::RightParen,
      ];
      let y = Some(vec![
         PostfixToken::Operand(5)]);
      assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn valid6 (){
//INFIX:
//POSTFIX:
      let x = &[
          InfixToken::LeftParen,
          InfixToken::LeftParen,
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Mul),
          InfixToken::Operand(10),
          InfixToken::Operator(Operator::Mul),
          InfixToken::Operand(5),
          InfixToken::Operator(Operator::Mul),
          InfixToken::Operand(4),
          InfixToken::RightParen,
          InfixToken::RightParen,
      ];
      let y = Some(vec![
          PostfixToken::Operand(6),
          PostfixToken::Operand(4),
          PostfixToken::Operator(Operator::Add),
          PostfixToken::Operand(6),
          PostfixToken::Operand(10),
          PostfixToken::Operator(Operator::Mul),
          PostfixToken::Operand(5),
          PostfixToken::Operator(Operator::Mul),
          PostfixToken::Operand(4),
          PostfixToken::Operator(Operator::Mul),
          PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn valid7 (){
    //INFIX: 3+4*(2−1)
    //POSTFIX: 3 4 2 1 - * +
        let x = &[InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::LeftParen,
                  InfixToken::Operand(2),
                  InfixToken::Operator(Operator::Sub),
                  InfixToken::Operand(1),
                  InfixToken::RightParen,
              ];
        let y = Some(vec![
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(2),
                 PostfixToken::Operand(1),
                 PostfixToken::Operator(Operator::Sub),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operator(Operator::Add),
                 ]);
        assert_eq!(y, infix_to_postfix(x));

}
#[test]
fn valid8(){
//INFIX: 3 + (4) + (4)
//POSTFIX: 3 4 + 4 +
 let x = &[InfixToken::Operand(3),
      InfixToken::Operator(Operator::Add),
      InfixToken::LeftParen,
      InfixToken::Operand(4),
      InfixToken::RightParen,
      InfixToken::Operator(Operator::Add),
      InfixToken::LeftParen,
      InfixToken::Operand(4),
      InfixToken::RightParen,
  ];
  let y = Some(vec![
     PostfixToken::Operand(3),
     PostfixToken::Operand(4),
     PostfixToken::Operator(Operator::Add),
     PostfixToken::Operand(4),
     PostfixToken::Operator(Operator::Add)]);
  assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn valid9 () {
    //INFIX: ((3+5))
    //POSTFIX: 3 5 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(5),
    InfixToken::RightParen,
    InfixToken::RightParen,
    ];
    let y = Some(vec![
   PostfixToken::Operand(3),
   PostfixToken::Operand(5),
   PostfixToken::Operator(Operator::Add)]);
   assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn valid10 () {
    //INFIX: (3) + (8) + 9 +1
    //POSTFIX: 3 5 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::LeftParen,
    InfixToken::Operand(8),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(9),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(1),
    ];
    let y = Some(vec![
   PostfixToken::Operand(3),
   PostfixToken::Operand(8),
   PostfixToken::Operator(Operator::Add),
   PostfixToken::Operand(9),
   PostfixToken::Operator(Operator::Add),
   PostfixToken::Operand(1),
   PostfixToken::Operator(Operator::Add)]);
   assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase0 (){
    //INFIX: 3+4*(2−1)
    //POSTFIX: 3 4 2 1 - * +
        let x = &[InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::LeftParen,
                  InfixToken::Operand(2),
                  InfixToken::Operator(Operator::Sub),
                  InfixToken::Operand(1),
                  InfixToken::RightParen,
              ];
        let y = Some(vec![
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(2),
                 PostfixToken::Operand(1),
                 PostfixToken::Operator(Operator::Sub),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operator(Operator::Add),
                 ]);
        assert_eq!(y, infix_to_postfix(x));

}




/*
let mut weight1 = 0;
let mut weight2 = 0;

if stack.len() == 0{

    match op{
       Operator::Add => weight1 = weight1 + 1,
       Operator::Sub => weight1 = weight1 + 1, //2,
       Operator::Mul => weight1 = weight1 + 2, //3,
       Operator::Div => weight1 = weight1 + 2,   //4,
   };

   stack.push(InfixToken::Operator(op));
} else {

   match op {
       Operator::Add | Operator::Sub => weight2 = 1,
       Operator::Mul | Operator::Div => weight2 = 2,
   }
   if weight1 >= weight2 {  //I want to stack.pop().unwrap(), and the output.push()

       let top = stack.pop().unwrap();

       match top {
           InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)),
           _ => {},
       }
       stack.push(InfixToken::Operator(op));

   } else {
       match op {
           Operator::Add | Operator::Sub => weight1 = 1,
           Operator::Mul | Operator::Div => weight1 = 2,
       }
       stack.push(InfixToken::Operator(op));
   }
}





     let l = match op {
       Operator::Add => 1,
       Operator::Sub => 1, //2,
       Operator::Mul => 2, //3,
       Operator::Div => 2,   //4,
     };
     while stack.len() > 0 {
       let stackob = stack.pop();
       if stackob == None {
           stack.push(InfixToken::Operator(op));
           break;
         }


       let s = stackob.unwrap();
       if s == InfixToken::LeftParen {
           stack.push(s);
           break;
       }
       let m = match s {
           InfixToken::Operator(t)=> t,
           _ => break,
       };

       let n = match m {
         Operator::Add => 1,
         Operator::Sub => 1, //2,
         Operator::Mul => 2, //3,
         Operator::Div => 2, //4,
       };
       if n >= l
       {
         output.push(PostfixToken::Operator(m));

       }
       else
       {
         stack.push(s);
         break;
       }
     }
     stack.push(InfixToken::Operator(op));
}
*/




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

impl Operator {
    fn precedence(&self) -> i32 {
        match *self {
            Operator::Add | Operator::Sub => 1,
            Operator::Mul | Operator::Div => 2,
        }
    }
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
//        let mut switch2 = 0;
//        let sample stack:
//        let sample output:

        for token in tokens.into_iter(){
            match *token {
                InfixToken::Operand(i) => output.push(PostfixToken::Operand(i)),
                InfixToken::LeftParen => if switch % 2 == 0{
        //                switch2 = switch2 + 1;
                        stack.push(InfixToken::LeftParen);    //stack.push(*token),
                    } else {
                        return None;
                    },
                InfixToken::RightParen => if stack.len() == 0 { //if RightParen is the first value on stack then none
                        return None;
                    } else {
                        /*let check = stack.pop().unwrap(); //top value on stack

                        if check == InfixToken::LeftParen { //if top of stack is a LeftParen then none
                            return None;*/
                //        } else {
                            switch = switch + 1;
                            while stack.len() != 0 {  //loop continues popping options and outputing them until the stack is empty
                                let top = stack.pop().unwrap();

                                match top {
                                    InfixToken::Operator(op) => output.push(PostfixToken::Operator(op)), //if top of stack is Operator then output it
                                    InfixToken::LeftParen => {}, //if top of stack is LeftParen then discard it
                                    _ => {},
                                }
                            }
                //        }
                    },
                InfixToken::Operator(ref op) => if stack.len() == 0 && switch % 2 == 0 { //|| stack.pop().unwrap() == InfixToken::LeftParen{
                        match *op {
                            Operator::Add | Operator::Sub => weight1 = 1,
                            Operator::Mul | Operator::Div => weight1 = 2,
                        }

                        stack.push(InfixToken::Operator(*op));

                    } else if switch % 2 ==0 {
                        return None;
                    } else { // if stack.len() == 1 && switch % 2 == 0 {
                        let check = stack.pop().unwrap();
                        if check == InfixToken::LeftParen{
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
                    } /*else {
                        return None;
                    }*/



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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let result = vec![PostfixToken::Operand(2), PostfixToken::Operand(3), PostfixToken::Operator(Operator::Mul), PostfixToken::Operand(1), PostfixToken::Operator(Operator::Add)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::Operand(2), InfixToken::Operator(Operator::Mul), InfixToken::Operand(3), InfixToken::Operator(Operator::Add), InfixToken::Operand(1)]));

/*
        let result = vec![PostfixToken::Operand(2), PostfixToken::Operand(3), PostfixToken::Operator(Operator::Mul)];
        assert_eq!(Some(result), infix_to_postfix(&[InfixToken::LeftParen, InfixToken::Operand(2), InfixToken::Operator(Operator::Mul), InfixToken::Operand(3), InfixToken::RightParen]));
*/
    }
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
*/
