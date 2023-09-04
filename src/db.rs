use sqlx::{Pool, Postgres};

pub struct DBClient {
  pool: Pool<Postgres>,
}

impl DBClient {
  pub fn new(pool: Pool<Postgres>) -> Self {
    DBClient { pool }
  }
}
