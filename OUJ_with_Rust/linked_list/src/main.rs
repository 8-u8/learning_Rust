
const NOT_FOUND: i32 = -1;
const DATA_SIZE: i32 = 6;

struct Node {
    data: i32,
    next: *mut Node,
}

fn linked_list_delete_head(head: &mut *mut Node) -> i32 {
    let data: i32;
    let temp: *mut Node;

    unsafe{
        if (*head).is_null() {
            return NOT_FOUND;
    }
        data = (**head).data;
        temp = (*head);
        *head = (**head).next;
        Box::from_raw(temp);
    }
    return data;
}


fn main() {
    println!("Hello, world!");
}
