use bson;
use rocket_contrib::json::JsonValue;

pub fn generateErrorJson(errorMessage: String, status: usize) -> JsonValue {
    return json!({
        "code": status,
        "success": false,
        "data": {},
        "error": errorMessage
    });
}