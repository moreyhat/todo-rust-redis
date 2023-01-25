use redis::{Commands, Connection, RedisError, RedisResult};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Todo {
    endpoint: String,
}

impl Todo {
    fn get_connection(&self) -> RedisResult<Connection> {
        let client = redis::Client::open(self.endpoint.as_str())?;
        client.get_connection()
    }
    pub fn get(&self, id: f64) -> Option<String> {
        let mut con = match self.get_connection() {
            Ok(con) => con,
            Err(_) => return None,
        };

        match con.get(id) {
            Ok(value) => return Some(value),
            Err(_) => return None,
        };
    }
    pub fn put(&self, todo: String) -> Result<f64, RedisError> {
        let mut con = self.get_connection()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let _: () = con.set(&now, todo)?;
        Ok(now)
    }
    pub fn delete(&self, id: f64) -> Result<(), RedisError> {
        let mut con = self.get_connection()?;
        con.del(id)?;
        Ok(())
    }
    pub fn list(&self) -> Result<Vec<String>, RedisError> {
        let mut response: Vec<String> = vec![];
        let mut con = self.get_connection()?;
        let keys: Vec<f64> = con.keys("\\*")?;
        for key in keys {
            let value = con.get(key)?;
            response.push(value);
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const REDIS_ENDPOINT: &str = "redis://127.0.0.1/";

    fn delete_all_keys() {
        let client = redis::Client::open(REDIS_ENDPOINT).unwrap();
        let mut con = client.get_connection().unwrap();
        let keys: Vec<f64> = con.keys("\\*").unwrap();

        for key in keys {
            con.del::<f64, f64>(key).unwrap();
        }
    }
    #[test]
    fn put_get_del_success() {
        let todo = Todo {
            endpoint: String::from(REDIS_ENDPOINT),
        };

        let test_data = "This is test todo".to_string();
        let test_data_clone = test_data.clone();

        let put_result = match todo.put(test_data) {
            Ok(id) => id,
            Err(_) => {
                assert!(false);
                return;
            }
        };

        let stored_todo = todo.get(put_result).unwrap();
        assert_eq!(test_data_clone, stored_todo);

        match todo.delete(put_result) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };

        delete_all_keys();
    }
    #[test]
    fn get_all_keys_success() {
        let todo = Todo {
            endpoint: String::from(REDIS_ENDPOINT),
        };

        let test_data = vec![
            "The first todo".to_string(),
            "The second todo".to_string(),
            "The third todo".to_string(),
            "The fourth todo".to_string(),
            "The fifth todo".to_string(),
        ];

        let verification_data = test_data.clone();

        for test_todo in test_data {
            let _ = todo.put(test_todo);
        }

        let todo_list = todo.list().unwrap();
        for test_todo in todo_list {
            assert!(verification_data.contains(&test_todo));
        }

        delete_all_keys();
    }
}
