use redis::{Commands, Connection, RedisError, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: String,
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
    pub fn get(&self, id: String) -> Option<ToDo> {
        let mut con = match self.get_connection() {
            Ok(con) => con,
            Err(_) => return None,
        };

        match con.get(id.clone()) {
            Ok(description) => return Some(ToDo { id, description }),
            Err(_) => return None,
        };
    }
    pub fn put(&self, todo: &ToDo) -> Result<String, RedisError> {
        let mut con = self.get_connection()?;
        let _: () = con.set(todo.id.clone(), &todo.description)?;
        Ok(todo.id.clone())
    }
    pub fn delete(&self, id: String) -> Result<(), RedisError> {
        let mut con = self.get_connection()?;
        con.del(id)?;
        Ok(())
    }
    pub fn list(&self) -> Result<Vec<ToDo>, RedisError> {
        let mut response: Vec<ToDo> = vec![];
        let mut con = self.get_connection()?;
        let keys: Vec<String> = con.keys("*")?;
        for key in keys {
            println!("{}", key);
            let value = con.get(key.clone())?;
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
    use rand::{distributions::Alphanumeric, Rng};
    const REDIS_ENDPOINT: &str = "redis://127.0.0.1/";

    #[test]
    fn put_get_del_success() {
        let to_do_client = ToDoClient {
            endpoint: String::from(REDIS_ENDPOINT),
        };

        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let test_data = ToDo {
            id: id.clone(),
            description: "This is test todo".to_string(),
        };

        let put_result = match to_do_client.put(&test_data) {
            Ok(id) => id,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        let stored_todo = to_do_client.get(id).unwrap();
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
            let id: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();

            test_data.push(ToDo {
                id,
                description: format!("To Do # {}", i),
            });
        }

        for test_todo in &test_data {
            let _ = to_do_client.put(test_todo);
        }

        let todo_list = to_do_client.list().unwrap();
        for test_todo in todo_list {
            assert!(test_data.contains(&test_todo));
            let _ = to_do_client.delete(test_todo.id);
        }
    }
}
