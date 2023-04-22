// make stack structure by OUJ class chp01.

// initialize constants.
const MAX: usize = 128;
const PUSH_SUCCESS: i32 = 1;
const PUSH_FAILURE: i32 = -1;
const POP_SUCCESS: i32 = 2;
const POP_FAILURE: i32 = -2;

// define functions 
fn stack_init(top: &mut i32) {
    *top = 0;
}

fn display(stack: &[i32], top: i32) {
    print!("stack ({}): ", top);
    for i in 0..top {
        print!("{} ", stack[i as usize]);
    }
    println!();
}

fn peek(stack: &[i32], top: &mut i32, data: &mut i32) -> i32 {
    if *top > 0 {
        *data = stack[(*top - 1) as usize];
        POP_SUCCESS
    } else {
        // if stack is empty
        POP_FAILURE
    }
}

fn push(stack: &mut [i32], top: &mut i32, data: i32) -> i32 {
    if *top >= MAX as i32 {
        // stack overflow
        return PUSH_FAILURE;
    } else {
        stack[*top as usize] = data;
        *top += 1;
        PUSH_SUCCESS
    }
}

fn pop(stack: &mut [i32], top: &mut i32, data: &mut i32) -> i32 {
    if *top > 0 {
        *data = stack[(*top - 1) as usize];
        *top -= 1;
        POP_SUCCESS
    } else {
        // if stack is empty
        POP_FAILURE
    }
}

fn main() {
    let mut stack: [i32; MAX] = [0; MAX];
    let mut top = 0;
    let mut data = 0;

    stack_init(&mut top);
    data = 300;
    println!("push: {}", data);
    push(&mut stack, &mut top, data);

    data = 400;
    println!("push: {}", data);
    push(&mut stack, &mut top, data);

    data = 500;
    println!("push: {}", data);
    push(&mut stack, &mut top, data);

    peek(&stack, &mut top, &mut data);
    println!("peek: {}", data);
    peek(&stack, &mut top, &mut data);

    display(&stack, top);

    pop(&mut stack, &mut top, &mut data);
    println!("pop: {}", data);

    peek(&stack, &mut top, &mut data);
    println!("peek: {}", data);

    pop(&mut stack, &mut top, &mut data);
    println!("pop: {}", data);

    pop(&mut stack, &mut top, &mut data);
    println!("pop: {}", data);
}
