// Smart pointers are data structures that act like pointers but have additional metadata and capabilities.
// The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.
// The Box<T> type also implements the Drop trait, which allows Box<T> values to be cleaned up when they go out of scope.
// The Rc<T> type is a reference-counted smart pointer that allows multiple ownership of a value.
// The RefCell<T> type is a smart pointer that allows mutable borrows checked at runtime.
// The Arc<T> type is an atomically reference-counted smart pointer that allows shared ownership across threads.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
struct MyData {
    value: RefCell<i32>,
}
#[allow(clippy::arc_with_non_send_sync)]
#[allow(unused_variables)]
fn main() {
    // Using Rc (Reference Counting) smart pointer
    println!("Using Rc (Reference Counting) smart pointer:");
    let data_rc = Rc::new(MyData {
        value: RefCell::new(42),
    });

    // Cloning an Rc pointer increases the reference count
    let cloned_rc1 = Rc::clone(&data_rc);
    let cloned_rc2 = Rc::clone(&data_rc);

    // Modify the value using RefCell and Rc
    *data_rc.value.borrow_mut() += 10;

    println!("Data value: {:?}", cloned_rc2.value.borrow());

    println!(
        "Reference count after cloning: {}",
        Rc::strong_count(&data_rc)
    );
    println!("Data value: {:?}", data_rc);

    // Using Arc (Atomically Reference Counted) smart pointer
    println!("\nUsing Arc (Atomically Reference Counted) smart pointer:");
    let data_arc = Arc::new(MyData {
        value: RefCell::new(84),
    });

    // Cloning an Arc pointer also increases the reference count
    let cloned_arc1 = Arc::clone(&data_arc);
    let cloned_arc2 = Arc::clone(&data_arc);

    // Modify the value using RefCell and Arc
    *data_arc.value.borrow_mut() += 20;

    println!("Data value: {:?}", cloned_arc2.value.borrow());

    println!(
        "Reference count after cloning: {}",
        Arc::strong_count(&data_arc)
    );
    println!("Data value: {:?}", data_arc);
}
