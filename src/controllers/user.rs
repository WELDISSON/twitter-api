use bson;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;
use crate::utils;

#[get("/user/<id>")]
pub fn get(id: String) -> JsonValue {
  match utils::validations::validateObjectId(id.clone()) {
    false => {
      return utils::validations::generateErrorJson("Id incorreto".to_string(), 400);
    },
    true => {
      let document = models::User::find_one(id.to_owned()).unwrap();
      let result = bson::from_bson::<meta::user::GetResponse>(bson::Bson::Document(document.unwrap()));
    
      match result {
        Ok(user) => {
          json!({
            "code": 200,
            "success": true,
            "data": user,
            "error": "null"
          })
        },
        Err (_e) => {
          return utils::validations::generateErrorJson(_e.to_string(), 404);
        }
      }
    }
  }
}

#[get("/users")]
pub fn getAll() -> JsonValue {
  match models::User::find() {
    Ok(users) => {
      json!({
        "code": 200,
        "success": true,
        "data": users,
        "error": "null"
      })
    },
    Err (_e) => {
      return utils::validations::generateErrorJson(_e.to_string(), 404);
    }
  }
}

#[post("/user", format = "application/json", data = "<user>")]
pub fn insert(user: Json<meta::user::Post>) -> JsonValue {

  let model = models::User::Model {
    email: user.email.to_owned(),
    name: user.name.to_owned()
  };

  let document = model.insert().unwrap();
  let result = bson::from_bson::<meta::user::PostResponse>(bson::Bson::Document(document.unwrap()));

  match result {
    Ok(user) => {
      json!({
        "code": 201,
        "success": true,
        "data": user,
        "error": "null"
      })
    },
    Err (_e) => {
      return utils::validations::generateErrorJson(_e.to_string(), 400);
    }
  }
}