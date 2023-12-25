use std::sync::{Arc, Mutex};

type AM<T> = Arc<Mutex<T>>;