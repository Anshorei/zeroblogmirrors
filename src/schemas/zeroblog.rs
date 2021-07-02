use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Content {
  pub address: String,
  pub modified: usize,
  pub title: String,
  pub description: String,
  pub favicon: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ZeroBlog {
  pub title: String,
  pub description: String,
  pub links: String,
  pub next_post_id: usize,
  pub demo: bool,
  pub modified: usize,
  pub post: Vec<ZeroBlogPost>,
  #[serde(default)]
  pub tag: Vec<ZeroBlogPostTag>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ZeroBlogPost {
  pub post_id: usize,
  pub title: String,
  pub date_published: f64,
  pub body: String,
}

impl ZeroBlogPost {
  pub fn short_body(&self) -> String {
    self.body.split("* * *").next()
      .map(|s| s.to_string())
      .unwrap_or(self.body.clone())
  }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ZeroBlogPostTag {
  pub post_id: usize,
  pub value: String,
}
