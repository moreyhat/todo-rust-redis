use redis::{Commands, Connection, RedisError, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: f64,
    pub description: String,
}

pub struct ToDoClient {
    pub endpoint: String,
}

impl ToDoClient {
    fn get_connection(&self) -> RedisResult<Connection> {
        let client = redis::Client::open(self.endpoint.as_str())?;
        client.get_connection()
    }
    pub fn get(&self, id: f64) -> Option<ToDo> {
        let mut con = match self.get_connection() {
            Ok(con) => con,
            Err(_) => return None,
        };

        match con.get(id) {
            Ok(description) => return Some(ToDo { id, description }),
            Err(_) => return None,
        };
    }
    pub fn put(&self, todo: &ToDo) -> Result<f64, RedisError> {
        let mut con = self.get_connection()?;
        let _: () = con.set(todo.id, &todo.description)?;
        Ok(todo.id)
    }
    pub fn delete(&self, id: f64) -> Result<(), RedisError> {
        let mut con = self.get_connection()?;
        con.del(id)?;
        Ok(())
    }
    pub fn list(&self) -> Result<Vec<ToDo>, RedisError> {
        let mut response: Vec<ToDo> = vec![];
        let mut con = self.get_connection()?;
        let keys: Vec<f64> = con.keys("*")?;
        for key in keys {
            let value = con.get(key)?;
            response.push(ToDo {
                id: key,
                description: value,
            });
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    const REDIS_ENDPOINT: &str = "redis://127.0.0.1/";

    #[test]
    fn put_get_del_success() {
        let to_do_client = ToDoClient {
            endpoint: String::from(REDIS_ENDPOINT),
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let test_data = ToDo {
            id: now,
            description: "This is test todo".to_string(),
        };

        let put_result = match to_do_client.put(&test_data) {
            Ok(id) => id,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        let stored_todo = to_do_client.get(now).unwrap();
        assert_eq!(test_data.id, stored_todo.id);
        assert_eq!(test_data.description, stored_todo.description);

        match to_do_client.delete(put_result) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };
    }
    #[test]
    fn get_all_keys_success() {
        let to_do_client = ToDoClient {
            endpoint: String::from(REDIS_ENDPOINT),
        };

        let mut test_data: Vec<ToDo> = vec![];

        for i in 0..4 {
            test_data.push(ToDo {
                id: i as f64,
                description: format!("To Do # {}", i),
            })
        }

        for test_todo in &test_data {
            let _ = to_do_client.put(test_todo);
        }

        let todo_list = to_do_client.list().unwrap();
        assert_eq!(todo_list.len(), test_data.len());
        for test_todo in todo_list {
            assert!(test_data.contains(&test_todo));
            let _ = to_do_client.delete(test_todo.id);
        }
    }
}
