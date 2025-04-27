use crate::prelude::*;
pub use voter_registration_shared::prelude::diesel_sqlite::*;



pub use actix_web_httpauth::{
    extractors::bearer::BearerAuth,
    middleware::HttpAuthentication,
};

pub use utoipa_actix_web::AppExt;
pub use utoipa;
pub use utoipa::ToSchema;
pub use utoipa::Modify;
pub use utoipa::OpenApi;
pub use utoipa::openapi;
pub use utoipa_swagger_ui::SwaggerUi;
pub use utoipa_swagger_ui::Url as SwagUrl;
pub use utoipa::openapi::security;
pub use utoipa::openapi::security::SecurityScheme;
pub use utoipa::openapi::security::Http;
pub use utoipa::openapi::security::HttpAuthScheme;

// pub use actix_web::error::ErrorInternalServerError as internal_err;
pub use actix_files;
pub use actix_web::error::ErrorBadRequest as internal_err;
pub use actix_web::{
    self,
    HttpServer,
    middleware::Logger,
    dev::ServiceRequest,
    web::{
        self,
        Data,
    },
    error,
    get,
    delete,
    post,
    HttpResponse,
    Responder,
    ResponseError,
};

pub use actix_cors::Cors;


pub trait IntoResponseError<E>: core::error::Error {
    fn ae(&'static self) -> actix_web::Error {
        error::ErrorInternalServerError(self)
    }
}

// impl error::ResponseError for AnyErr {}

pub async fn web_block<F, T>(
    closure: F,
) -> actix_web::Result<T>
where
    F: FnOnce() -> AResult<T> + Send + 'static,
    T: Send + 'static,
{
    let res = web::block(closure)
        .await
        .map_err(internal_err)?
    .map_err(internal_err)?;

    Ok(res)
}




// impl ConnectionProvider for web::Data<SqliteDbPool>
// {
//     fn conn(&self) -> AResult<DbConn> {
//         let conn = self.get()
//             .map_err(msg)?;

//         Ok(conn)
//     }
// }
