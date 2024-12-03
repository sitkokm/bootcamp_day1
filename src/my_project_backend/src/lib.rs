use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk::update]
fn set_msg(new_msg: String) {
    MSG.with(|msg| *msg.borrow_mut()= new_msg)
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hi, {}!", name)
}
