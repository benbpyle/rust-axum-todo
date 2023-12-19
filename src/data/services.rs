pub mod services {
    use aws_sdk_dynamodb::{
        types::{AttributeValue, ReturnValue},
        Client, Error,
    };
    use serde_dynamo::from_item;
    use uuid::Uuid;

    use crate::data::models::{DbError, Todo};

    #[derive(Clone)]
    pub struct TodoService {
        client: Client,
        table_name: String,
    }

    impl TodoService {
        // creates a new instance of the TodoService
        pub fn new(client: Client, table_name: String) -> TodoService {
            TodoService { client, table_name }
        }

        pub async fn delete_to_by_id(&self, id: &String) -> Result<String, DbError> {
            let n = std::format!("ID#{}", id);
            let r = self
                .client
                .delete_item()
                .condition_expression("attribute_exists(id)")
                .key("id".to_string(), AttributeValue::S(n.to_string()))
                .table_name(&self.table_name)
                .send()
                .await;

            match r {
                Ok(_) => Ok(n),
                Err(_) => Err(DbError::NotFound),
            }
        }

        pub async fn find_todo_by_id(&self, id: &String) -> Result<Todo, DbError> {
            let n = std::format!("ID#{}", id);
            let result = self
                .client
                .get_item()
                .key("id".to_string(), AttributeValue::S(n.to_string()))
                .table_name(&self.table_name)
                .send()
                .await?;

            match result.item {
                None => Err(DbError::NotFound),
                Some(item) => {
                    let i: Todo = from_item(item)?;
                    Ok(i)
                }
            }
        }

        pub async fn update_todo_by_id(
            &self,
            id: &String,
            description: &String,
        ) -> Result<Todo, DbError> {
            let n = std::format!("ID#{}", id);
            let result = self
                .client
                .update_item()
                .key("id".to_string(), AttributeValue::S(n.to_string()))
                .condition_expression("attribute_exists(id)")
                .update_expression("set description = :d")
                .expression_attribute_values(":d", AttributeValue::S(description.to_string()))
                .return_values(ReturnValue::AllNew)
                .table_name(&self.table_name)
                .send()
                .await;

            match result {
                Ok(r) => match r.attributes {
                    None => Err(DbError::NotFound),
                    Some(attributes) => {
                        let i: Todo = from_item(attributes)?;
                        Ok(i)
                    }
                },
                Err(_) => Err(DbError::NotFound),
            }
        }

        pub async fn create_todo(&self, description: &String) -> Result<Todo, Error> {
            let id = Uuid::new_v4();
            let todo_id: String = std::format!("ID#{}", id);

            let todo = Todo {
                id: todo_id.to_owned(),
                todo_id: id.to_string(),
                description: description.to_owned(),
            };

            let _ = self
                .client
                .put_item()
                .item("id", AttributeValue::S(String::from(todo_id)))
                .item("todo_id", AttributeValue::S(String::from(id)))
                .item("description", AttributeValue::S(String::from(description)))
                .table_name(self.table_name.to_owned())
                .send()
                .await?;

            Ok(todo)
        }
    }
}
