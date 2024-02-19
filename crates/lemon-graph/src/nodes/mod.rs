use std::{collections::HashMap, future::Future};

use crate::Data;

pub mod delay;

pub trait Node {
    /// Process input from the graph.
    /// Called before running the node.
    fn process_input(&mut self, input: HashMap<String, Data>) {
        let _ = input;
    }
}

pub trait AsyncNode: Node {
    fn run(&self) -> Box<dyn Future<Output = ()> + Unpin>;
}

pub trait SyncNode: Node {
    fn run(&self);
}