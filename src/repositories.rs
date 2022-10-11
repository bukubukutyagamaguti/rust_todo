use std::{
    collections::HashMap,
    sync::{
        Arc,
        RwLock,
        RwLockReadGuard,
        RwLockWriteGuard
    }
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

// error系の内容を定義
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

// traitを設定
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

// 各構造体を設定
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,   
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDates = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDates>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory { store: Arc::default(), }
    }
    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDates> {
        self.store.write().unwrap()
    }
    
    fn read_store_ref(&self) -> RwLockReadGuard<TodoDates> {
        self.store.read().unwrap()
    }
}


impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        todo
    }
    fn find(&self, id: i32) -> Option<Todo> {
        todo!();
    }
    fn all(&self) -> Vec<Todo>{
        todo!();
    }
    
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>{
        todo!();
    }
    
    fn delete(&self, id: i32) -> anyhow::Result<()>{
        todo!();
    }
    
}

