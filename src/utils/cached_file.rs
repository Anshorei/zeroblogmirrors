use rocket::{Request, response::{Response, Responder, NamedFile, Result}};

pub struct CachedFile(NamedFile);

impl CachedFile {
  pub fn new(named_file: NamedFile) -> CachedFile {
    CachedFile(named_file)
  }
}

impl<'r> Responder<'r> for CachedFile {
  fn respond_to(self, req: &Request) -> Result<'r> {
    Response::build_from(self.0.respond_to(req)?)
      .raw_header("Cache-control", "max-age=86400") //  24h (24*60*60)
      .ok()
  }
}
