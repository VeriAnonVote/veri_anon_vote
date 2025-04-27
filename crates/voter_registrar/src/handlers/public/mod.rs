use crate::prelude::*;
use voter_registration_shared::{
    schema::verifier,
    schema::verifier::dsl,
    models::voter::Voter,
    models::verifier::PublicReqVerifier,
};

#[get("/registration_closed_status")]
pub async fn get_registration_closed_status(
    registration_closed: Data<RegistrationCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let res = web_block(move || -> AResult<bool> {
        Ok(registration_closed.load(Ordering::Relaxed))
    }).await?;

    let res = HttpResponse::Ok().json(res);
    Ok(res)
}

#[get("/all_voters")]
pub async fn get_all_voters(
    pool: web::Data<DbPool>,
    registration_closed: Data<RegistrationCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let all_voters = web_block(move || -> AResult<Vec<Voter>> {
        if !registration_closed.load(Ordering::Relaxed) {
            return Err(msg("registration has not closed"));
        }

        Voter::get_all(pool.get_ref())
    }).await?;


    let res = HttpResponse::Ok().json(all_voters);
    Ok(res)
}

#[get("/voter/{id}")]
pub async fn get_one_voter(
    voter_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<HttpResponse> {
    let voter_details = web_block(move || -> AResult<Voter> {
        Voter::get_one(*voter_id, pool.get_ref())
    }).await?;

    Ok(HttpResponse::Ok().json(voter_details))
}


#[get("/verifiers")]
pub async fn get_all_verifiers(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let verifiers = web_block(move || -> AResult<Vec<PublicReqVerifier>> {
        let conn = &mut pool.get()?;
        let verifiers = dsl::verifier.filter(verifier::id.gt(0))
            .select(PublicReqVerifier::as_select())
            .load(conn)?;
        Ok(verifiers)
    }).await?;

    let res = HttpResponse::Ok().json(verifiers);
    Ok(res)
}



#[get("/voter_requirements")]
pub async fn get_voter_requirements(
    config: web::Data<RegistrarConfig>,
) -> actix_web::Result<impl Responder> {
    let res = HttpResponse::Ok().json(config.voter_requirements.clone());
    Ok(res)
}

