use actix_web::{
    delete, error, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::env;
use todo_rust_redis::todo::{ToDo, ToDoClient};

#[derive(Serialize, Deserialize)]
struct PostRequest {
    description: String,
}

#[derive(Serialize, Deserialize)]
struct PostResponse {
    id: String,
}

#[get("/")]
async fn list_todo(_req: HttpRequest) -> Result<impl Responder> {
    let redis_host = match env::var("REDIS_HOST") {
        Ok(host) => host,
        Err(_) => "127.0.0.1".to_string(),
    };
    let redis_endpoint = format!("redis://{}/", redis_host);
    let to_do_client = ToDoClient {
        endpoint: redis_endpoint,
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
    let redis_host = match env::var("REDIS_HOST") {
        Ok(host) => host,
        Err(_) => "127.0.0.1".to_string(),
    };
    let redis_endpoint = format!("redis://{}/", redis_host);
    let to_do_client = ToDoClient {
        endpoint: redis_endpoint,
    };

    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let description = todo.description.clone();
    let res = match to_do_client.put(&ToDo { id, description }) {
        Ok(id) => id,
        Err(_) => return Err(error::ErrorBadRequest("Failed to put a ToDo")),
    };

    Ok(web::Json(PostResponse { id: res }))
}

#[delete("/{id}")]
async fn delete_todo(path: web::Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();
    let redis_host = match env::var("REDIS_HOST") {
        Ok(host) => host,
        Err(_) => "127.0.0.1".to_string(),
    };
    let redis_endpoint = format!("redis://{}/", redis_host);
    let to_do_client = ToDoClient {
        endpoint: redis_endpoint,
    };
    match to_do_client.delete(id.clone()) {
        Ok(_) => return Ok(HttpResponse::Ok()),
        Err(_) => {
            return Err(error::ErrorInternalServerError(format!(
                "Failed to delete To Do: {}",
                id
            )));
        }
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(list_todo).service(post_todo))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test};
    const REDIS_ENDPOINT: &str = "redis://127.0.0.1/";

    #[actix_web::test]
    async fn test_list_todo() {
        let mut test_data: Vec<ToDo> = vec![];
        for i in 0..4 {
            let id: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();
            test_data.push(ToDo {
                id,
                description: format!("To Do # {}", i),
            })
        }

        let to_do_client = ToDoClient {
            endpoint: REDIS_ENDPOINT.to_string(),
        };
        for test_todo in &test_data {
            let _ = to_do_client.put(test_todo);
        }

        let before = to_do_client.list().unwrap();
        assert!(before.len() >= test_data.len());
        for test_todo in &test_data {
            assert!(before.contains(&test_todo));
        }

        let app = test::init_service(App::new().service(list_todo)).await;
        let req = test::TestRequest::get()
            .uri("/")
            .insert_header(ContentType::json())
            .to_request();
        let resp: Vec<ToDo> = test::call_and_read_body_json(&app, req).await;
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

        let posted_data = to_do_client.get(resp.id.clone()).unwrap();
        assert_eq!(posted_data.id, resp.id);

        assert_eq!(posted_data.description, test_description.to_string());
        _ = to_do_client.delete(resp.id);
    }

    #[actix_web::test]
    async fn test_delete_todo() {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let test_todo = ToDo {
            id,
            description: "ToDo for deleting test".to_string(),
        };

        let to_do_client = ToDoClient {
            endpoint: REDIS_ENDPOINT.to_string(),
        };

        let id = to_do_client.put(&test_todo).unwrap();
        let before_deleting = to_do_client.list().unwrap();
        assert!(before_deleting.contains(&test_todo));

        let app = test::init_service(App::new().service(delete_todo)).await;
        let req = test::TestRequest::delete()
            .uri(format!("/{}", id).as_str())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let after_deleting = to_do_client.list().unwrap();
        assert!(!after_deleting.contains(&test_todo));
    }
}
