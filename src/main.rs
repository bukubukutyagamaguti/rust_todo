mod repositories;
mod handlers;

use crate::repositories::{
    TodoRepository,
    TodoRepositoryForMemory,
    // CreateTodo,
    // UpdateTodo,
    // Todo
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
    // use axum::body;
    use repositories::{CreateTodo, Todo};
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode}
    };
    use tower::ServiceExt;
    use crate::repositories::{
        TodoRepository,
        TodoRepositoryForMemory,
        UpdateTodo,
    };
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

    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
                .uri(path)
                .method(method)
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json_body))
                .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
                .uri(path)
                .method(method)
                .body(Body::empty())
                .unwrap()
    }

    async fn res_to_todo(res: Response) -> Todo {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo   = serde_json::from_str(&body).expect(&format!("cannot convert Todo instance. body:{}", body));
        todo
    }

    #[tokio::test]
    async fn should_created_todo() {
        let expected = Todo::new(1, "should_created_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json(
            "/todos",
            Method::POST,
            r#"{"text" : "should_created_todo"}"#.to_string(),
        );

        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let expected = Todo::new(1, "should_created_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();

        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_get_all_todos() {
        let expected = Todo::new(1, "should_created_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_created_todo".to_string()));
        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Vec<Todo> = serde_json::from_str(&body).expect(&format!("connot convert Todo instance. body: {}", body));
        assert_eq!(vec![expected], todo);
    }

    
    #[tokio::test]
    async fn should_update_todos() {
        let expected = Todo::new(1, "should_update_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("before_update_todo".to_string()));

        let req = build_todo_req_with_json(
                                    "/todos/1",
                                    Method::PATCH,
                                    r#"{
                                        "id": 1,
                                        "text": "should_update_todo",
                                        "completed": false,
                                    }"#.to_string(),
                                );
        
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_delete_todos() {
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_delete_todo".to_string()));

        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}