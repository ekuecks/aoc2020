use std::collections::HashMap;

struct Node {
    val: u64,
    next: Option<*mut Node>,
}

fn main() {
    let mut nodes: HashMap<u64, *mut Node> = HashMap::new();
    let cur = Box::new(Node { val: 1, next: None });
    let mut ptr: *mut Node = Box::into_raw(cur);
    nodes.insert(1, ptr);
    for val in &[6, 7, 2, 4, 8, 3, 5, 9] {
        let next = Box::new(Node {
            val: *val,
            next: None,
        });
        let next_ptr: *mut Node = Box::into_raw(next);
        unsafe { (*ptr).next = Some(next_ptr) };
        ptr = next_ptr;
        nodes.insert(*val, ptr);
    }
    for val in 10..=1000000 {
        let next = Box::new(Node { val, next: None });
        let next_ptr: *mut Node = Box::into_raw(next);
        unsafe { (*ptr).next = Some(next_ptr) };
        ptr = next_ptr;
        nodes.insert(val, ptr);
    }
    let head_ptr: *mut Node = *nodes.get(&1).unwrap();
    unsafe { (*ptr).next = Some(head_ptr) };

    let mut cur_node = *nodes.get(&1).unwrap();
    for j in 0..10_000_000 {
        if j % 100_000 == 0 {
            println!("{}", j);
        }
        let value = unsafe { (*cur_node).val };
        let mut to_remove = Vec::new();
        for _ in 0..3 {
            let to_insert = unsafe { (*cur_node).next.unwrap() };
            unsafe { (*cur_node).next = (*to_insert).next };
            to_remove.push(to_insert);
        }
        let mut target_value = value - 1;
        loop {
            if target_value == 0 {
                target_value = 1_000_000;
            }
            let mut found = false;
            for removed in to_remove.iter() {
                if unsafe { (**removed).val == target_value } {
                    target_value -= 1;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        let mut insert_node = *nodes.get(&target_value).unwrap();
        let stored = unsafe { (*insert_node).next.unwrap() };
        for removed in to_remove {
            unsafe { (*insert_node).next = Some(removed) };
            insert_node = removed;
        }
        unsafe { (*insert_node).next = Some(stored) };
        cur_node = unsafe { (*cur_node).next.unwrap() };
    }
    let cur_node = *nodes.get(&1).unwrap();
    let n_v = unsafe { (*(*cur_node).next.unwrap()).val };
    let nn_v = unsafe { (*(*(*cur_node).next.unwrap()).next.unwrap()).val };
    println!("{}, {}", n_v, nn_v);
    println!("{}", n_v * nn_v);
}
