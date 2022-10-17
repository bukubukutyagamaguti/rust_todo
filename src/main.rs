mod repositories;
mod handlers;

use crate::repositories::{
    TodoRepository,
    TodoRepositoryForMemory,
    CreateTodo,
    UpdateTodo,
    Todo
};

use axum::{
    extract::Extension,
    routing::{get, post},
    Router
};

use handlers::{all_todo,create_todo, delete_todo, find_todo, update_todo};

use std::net::SocketAddr;
use std::{
    // Arc 共有所有権に関して
    // RwLock リーダ、ライター権限をロック
    env,
    sync::Arc
};

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", 
        post(create_todo::<T>).get(all_todo::<T>))
        .route("/todo/:id",
                get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
            )
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "heeeeeeee"
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn todo_crud_scenario(){
        let text = "todo text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        // create
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo{text});
        assert_eq!(expected, todo);

        // find
        let todo = repository.find(todo.id).unwrap();
        assert_eq!(expected, todo);

        // all
        let todo = repository.all();
        assert_eq!(vec![expected], todo);

        // update
        let text = "update todo text".to_string();
        let todo = repository.update(1, UpdateTodo {
            text : Some(text.clone()),
            completed: Some(true),
        },).expect("failed update todo");
        assert_eq!(
            Todo {
                id,
                text,
                completed: true,
            },
            todo
        );

        //delete
        let res = repository.delete(id);
        assert!(res.is_ok())
    }

    
}