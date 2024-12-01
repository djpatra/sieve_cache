use std::sync::{
    atomic::{
        AtomicBool, 
        Ordering
    },
    Arc
};


// pub struct ReferenceKeyNode<Key> {
//     key: Key,
//     read: AtomicBool
// }


pub struct ValueNode<Value> {
    value: Value,
    read: AtomicBool
}

// impl<Key> ReferenceKeyNode<Key> 
// where 
//     Key: Default
// {
//     pub fn new() -> Self {
//         ReferenceKeyNode { 
//             key: Default::default(), 
//             read: AtomicBool::new(true) }        
//     }

//     pub fn take_read_state(&self) -> bool {
//         self.read.swap(false, Ordering::Relaxed)
//     }

//     pub fn set(&self) {
//         self.read.store(true, Ordering::Relaxed);
//     }

//     pub fn key(&self) -> &Key {
//         &self.key
//     }
// }


impl<Value> ValueNode<Value> {
    pub fn new(value: Value) -> Self {
        ValueNode {
            value,
            read: AtomicBool::new(true)
        }        
    }

    pub fn set_read_state(&self) {
        self.read.store(true, Ordering::Relaxed);
    }

    pub fn take_read_state(&self) -> bool {
        self.read.swap(false, Ordering::Relaxed)
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
