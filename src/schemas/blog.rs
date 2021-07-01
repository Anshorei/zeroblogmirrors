use chrono::prelude::*;

pub struct Blog {
  headline: String,
  description: String,
  date_modified: DateTime<Utc>,
  date_published: DateTime<Utc>,
  keywords: Vec<String>,
}

pub struct BlogPosting {
  headline: String,
  datePublished: DateTime<Utc>,
  article_body: String,
  image: Option<String>,
  word_count: usize,
}
