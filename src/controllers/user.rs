use bson;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::models;
use crate::meta;
use crate::utils;


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
        "error": ""
      })
    },
    Err (_e) => {
      return utils::validations::generateErrorJson(_e.to_string(), 400);
    }
  }
}