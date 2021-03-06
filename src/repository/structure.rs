use serde::Serialize;
use rocket_contrib::databases::mongodb;
use crate::product::Product;
use crate::context::{Tag, Attribute};

#[derive(Serialize)]
pub struct Repository {
  pub ln_p: String,
  pub name: String,
  pub load: u64,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
  pub has: Vec<Product>,
}

impl Default for Repository {
  fn default() -> Self {
    Repository {
      ln_p: "Unknown".to_string(),
      name: "Unknown".to_string(),
      load: 0,
      tags: vec![],
      attributes: vec![],
      has: vec![],
    }
  }
}

impl From<mongodb::Document> for Repository {
  fn from(doc: mongodb::Document) -> Self {
    doc.iter().fold(Self::default(), |mut r, f| {
      // r: Repository, f: (&String, &Bson)
      match f.0.as_str() {
        "ln_p" => {
          r.ln_p = f.1.as_str().unwrap_or("Unknown").to_string();
        }
        "name" => {
          r.name = f.1.as_str().unwrap_or("Unknown").to_string();
        }
        // Note: Repository::load is not supposed to be filled here.
        "load" => {}
        // Note: Repository::has is not supposed to be filled here, but we also
        // need to ensure that if it is accidently added into the database it's
        // not treated as an attribute.
        "has" => {}
        _ => {
          // Skip MongoDB ID; this should be hidden from the user.
          if f.0 != "_id" {
            let kstr = f.0.to_string();
            let vstr = f.1.to_string();

            let key = if kstr.starts_with('"') && kstr.ends_with('"') {
              kstr.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_owned()
            } else {
              kstr
            };

            let value = if vstr.starts_with('"') && vstr.ends_with('"') {
              vstr.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_owned()
            } else {
              vstr
            };

            r.attributes.push(
              Attribute {
                key: key,
                value: value,
              }
            );
          };
        }
      };

      r
    })
  }
}
