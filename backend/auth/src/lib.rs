#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

pub mod db;

pub mod middleware {
    use actix_web::dev::ServiceRequest;
    use actix_web::Error;
    use actix_web_httpauth::extractors::bearer::BearerAuth;

    pub async fn jwt_validator(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, Error> {
        Ok(req)
    }
}
