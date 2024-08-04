use std::env;

use actix_web::{web, FromRequest, HttpRequest};
use diesel::{
    r2d2::{self, ConnectionManager, PooledConnection},
    PgConnection,
};
use std::future::{ready, Ready};


pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;
type PoolConn = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> PoolConn {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub struct Pool {
    pub conn: Conn,
}

impl FromRequest for Pool {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(pool) = req.app_data::<web::Data<PoolConn>>() {
            match pool.get() {
                Ok(conn) => {
                    return ready(Ok(Pool { conn }));
                }

                Err(_) => {
                    return ready(Err(actix_web::error::ErrorInternalServerError(
                        "couldn't get db connection from pool",
                    )));
                }
            }
        } else {
            return ready(Err(actix_web::error::ErrorInternalServerError(
                "couldn't get db connection from pool",
            )));
        }
    }
}
