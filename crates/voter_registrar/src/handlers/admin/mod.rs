use crate::prelude::*;
use voter_registration_shared::{
    schema::verifier::{
        self,
        dsl,
        wallet_address,
    },
    models::verifier::{
        Verifier,
        NewVerifier,
    },
};



// #[utoipa::path(
//     responses(
//         (status = 200, description = "Upsert successfully")
//     ),
//     params(
//         ("id", description = "Pet id"),
//     )
// )]
#[utoipa::path(
    context_path = "/admin",
    tag = "Upsert Verifier",
    request_body(
        content = NewVerifier,
        content_type = "application/json",
    ),
    responses(
        (
            status = 200,
            description = "upsert verifier",
            body = NewVerifier,
        )
    )
)]
#[post("/upsert_verifier")]
pub async fn upsert_verifier(
    verifier_map: Data<VerifierMap>,
    mut verifier_info: web::Json<NewVerifier>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    if verifier_info.api_key.is_none() {
        let api_key = passwords::PasswordGenerator::new()
            .length(128)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(false)
            .spaces(false)
            .exclude_similar_characters(true)
            .strict(true)
            .generate_one()
            .unwrap();

        verifier_info.api_key = Some(api_key);
    }

    let verifier_info = web_block(move || -> AResult<NewVerifier> {
        diesel::insert_into(verifier::table)
            .values(&*verifier_info)
            .on_conflict(wallet_address)
            .do_update()
            .set(&*verifier_info)
            .execute(&mut pool.conn()?)?;

        verifier_map.refresh(pool.as_ref())?;
        Ok(verifier_info.clone())
    }).await?;


    let res = HttpResponse::Ok().json(verifier_info);
    Ok(res)
}



#[utoipa::path(
    context_path = "/admin",
    tag = "Get all Verifier's all info",
    responses(
        (
            status = 200,
            description = "Verifiers from sqlite and dashmap",
            body = (Verifier, Verifier),
        )
    )
)]
#[get("/verifiers/details")]
pub async fn get_all_verifiers_details(
    verifier_map: Data<VerifierMap>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let verifiers = web_block(move || -> AResult<Vec<Verifier>> {
        verifier_map.refresh(pool.as_ref())?;
        Verifier::get_all(pool.as_ref())
    }).await?;

    // let res = HttpResponse::Ok().json((verifiers, verifier_map));
    let res = HttpResponse::Ok().json(verifiers);
    Ok(res)
}



#[utoipa::path(
    context_path = "/admin",
    tag = "Delete Verifier by Verifier's id",
    params(
        ("id", description = "Verifier id"),
    ),
    responses(
        (
            status = 200,
            description = "Delete successfully",
            body = String,
        )
    )
)]
#[delete("/verifier/{id}")]
pub async fn delete_verifier(
    verifier_map: Data<VerifierMap>,
    verifier_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    web_block(move || -> AResult<()> {
        diesel::delete(dsl::verifier.filter(dsl::id.eq(*verifier_id)))
            .execute(&mut pool.conn()?)?;

        verifier_map.refresh(pool.as_ref())
    }).await?;


    let res = "Delete successfully".to_string();
    Ok(res)
}



#[utoipa::path(
    tag = "Stop or begain Registation",
    context_path = "/admin",
    responses(
        (
            status = 200,
            description = "Current registration_closed value",
            body = bool,
        )
    )
)]
#[post("/toggle_registration_status")]
pub async fn toggle_registration_status(
    registration_closed: Data<RegistrationCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let status_to_be_set = !registration_closed.load(Ordering::Relaxed);
    web_block(move || -> AResult<()> {
        registration_closed.store(status_to_be_set, Ordering::Relaxed);
        Ok(())
    }).await?;

    // let res = format!("registration_closed: {status_to_be_set}");
    let res = HttpResponse::Ok().json(status_to_be_set);
    Ok(res)
}




// #[derive(Debug, Serialize)]
// struct BearerKey;

// impl Modify for BearerKey {
//     fn modify(&self, openapi: &mut openapi::OpenApi) {
//         if let Some(schema) = openapi.components.as_mut() {
//             schema.security_schemes.clear();
//             schema.add_security_scheme(
//                 "Bearer",
//                 SecurityScheme::Http(
//                     security::HttpBuilder::new()
//                         .scheme(HttpAuthScheme::Bearer)
//                         .bearer_format("Bearer")
//                         .build(),
//                 ),
//             );
//         }
//     }
// }
#[derive(OpenApi)]
#[openapi(
    paths(
        upsert_verifier,
        get_all_verifiers_details,
        toggle_registration_status,
        delete_verifier,
    ),
    // modifiers(&BearerKey),
)]
pub struct AdminDoc;

