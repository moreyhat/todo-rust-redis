use actix_web::{error, get, post, web, App, HttpRequest, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use todo_rust_redis::todo::{ToDo, ToDoClient};

const REDIS_ENDPOINT: &str = "redis://127.0.0.1/";

#[derive(Serialize, Deserialize)]
struct PostRequest {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct PostResponse {
    id: f64,
}

#[get("/")]
async fn list_todo(_req: HttpRequest) -> Result<impl Responder> {
    let to_do_client = ToDoClient {
        endpoint: REDIS_ENDPOINT.to_string(),
    };

    let to_do_list = match to_do_client.list() {
        Ok(list) => list,
        Err(error) => {
            println!("Failed to get the ToDo list: Error {}", error);
            vec![]
        }
    };

    Ok(web::Json(to_do_list))
}

#[post("/")]
async fn post_todo(todo: web::Json<PostRequest>) -> Result<impl Responder> {
    let to_do_client = ToDoClient {
        endpoint: REDIS_ENDPOINT.to_string(),
    };

    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let description = todo.description.clone();
    let res = match to_do_client.put(&ToDo { id, description }) {
        Ok(id) => id,
        Err(_) => return Err(error::ErrorBadRequest("Failed to put a ToDo")),
    };

    Ok(web::Json(PostResponse { id: res }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(list_todo).service(post_todo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, App};
    use todo_rust_redis::todo::{ToDo, ToDoClient};
    const REDIS_ENDPOINT: &str = "redis://127.0.0.1/";

    #[actix_web::test]
    async fn test_list_todo() {
        let mut test_data: Vec<ToDo> = vec![];
        for i in 100..104 {
            test_data.push(ToDo {
                id: i as f64,
                description: format!("To Do # {}", i),
            })
        }

        let to_do_client = ToDoClient {
            endpoint: REDIS_ENDPOINT.to_string(),
        };
        for test_todo in &test_data {
            let _ = to_do_client.put(test_todo);
        }

        let app = test::init_service(App::new().service(list_todo)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::json())
            .to_request();
        let resp: Vec<ToDo> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.len(), test_data.len());
        for test_todo in resp {
            assert!(test_data.contains(&test_todo));
            _ = to_do_client.delete(test_todo.id);
        }
    }

    #[actix_web::test]
    async fn test_post_todo() {
        let test_description = "Post ToDo test";

        let app = test::init_service(App::new().service(post_todo)).await;
        let req = test::TestRequest::post()
            .set_json(PostRequest {
                description: test_description.to_string(),
            })
            .insert_header(ContentType::json())
            .to_request();
        let resp: PostResponse = test::call_and_read_body_json(&app, req).await;
        let to_do_client = ToDoClient {
            endpoint: REDIS_ENDPOINT.to_string(),
        };

        let posted_data = to_do_client.get(resp.id).unwrap();
        assert_eq!(posted_data.id, resp.id);

        assert_eq!(posted_data.description, test_description.to_string());
        _ = to_do_client.delete(resp.id);
    }
}
