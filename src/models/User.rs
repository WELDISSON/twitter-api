use bson;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::{doc, error::Error};

use crate::lib;

#[derive(Debug)]
pub struct Model {
  pub name: String,
  pub email: String
}

impl Model {
    pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
      doc! { 
        "email": self.email.to_owned(),
        "name": self.name.to_owned(),
      }
    }
    
    pub fn insert(&self) -> Result<Option<bson::ordered::OrderedDocument>, Error> {
        let client = lib::mongo::establish_connection();
        let collection = client.db("twitter").collection("user");
        collection.insert_one(self.to_bson().clone(), None).ok().expect("Failed to execute find.");
        let result = collection.find_one(Some(self.to_bson().clone()), None)
        .ok().expect("Failed to execute find.");

        Ok(result)
    }
}

pub fn find() -> Result<Vec<bson::ordered::OrderedDocument>, Error> {
  let client = lib::mongo::establish_connection();
  let collection = client.db("twitter").collection("user");
  let response_document = collection.find(None, None).unwrap();

  response_document
      .map(|result| match result {
          Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
              Ok(result_model) => Ok(result_model),
              Err(_) => Err(Error::DefaultError(String::from(""))),
          },
          Err(err) => Err(err),
      })
      .collect::<Result<Vec<bson::ordered::OrderedDocument>, Error>>()
}