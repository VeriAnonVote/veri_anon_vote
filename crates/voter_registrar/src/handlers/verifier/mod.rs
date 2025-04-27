use crate::prelude::*;
use voter_registration_shared::{
    schema::{
        verifier,
        voter::{
            self,
            dsl,
        },
    },
    verifier_sig::Message,
    models::voter::NewVoter,
};



#[post("/voter")]
pub async fn insert_voter(
    // req: ServiceRequest,
    voter_info: web::Json<NewVoter>,
    config: web::Data<RegistrarConfig>,
    verifier_map: Data<VerifierMap>,
    pool: web::Data<DbPool>,
    credentials: Option<BearerAuth>,
    registration_closed: Data<RegistrationCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let voter_id = web_block(move || -> AResult<i32> {
        if registration_closed.load(Ordering::Relaxed) {
            return Err(msg("registration closed"));
        }
        let verifier_info = verifier_map.get(credentials.unwrap().token())
            .unwrap()
            .clone();

        let message = Message::from_metadata(
            &voter_info,
            &config.voter_requirements.required_identity
        )?;

        if !message.verify(&voter_info.verifier_sig, &verifier_info.wallet_address)? {
                return Err(msg("message.verify failed"));
        }


        let conn = &mut pool.get()?;
        conn.transaction::<(), DslError, _>(|conn| {
            diesel::insert_into(voter::table)
                .values(&*voter_info)
                .execute(conn)?;

            diesel::update(verifier::table)
                .filter(verifier::dsl::id.eq(verifier_info.id))
                .set(verifier::dsl::max_upload_count.eq(verifier_info.max_upload_count - 1))
                .execute(conn)?;

            Ok(())
        })?;

        let voter_id = dsl::voter.filter(voter::voter_pubkey.eq(&voter_info.voter_pubkey))
            .select(voter::id)
            .first(conn)
            .map_err(msg)?;

        verifier_map.alter(&verifier_info.api_key, move |_, mut v| {
            v.max_upload_count -= 1;
            v
        });
        Ok(voter_id)
    }).await?;

    let res = HttpResponse::Ok().json(voter_id);
    Ok(res)
}


// #[get("/voter/{id}")]
// pub async fn get_verifier_id (
//     // voter_map: Data<VoterMap>,
//     voter_id: web::Path<i32>,
//     pool: web::Data<DbPool>,
// ) -> actix_web::Result<HttpResponse> {
//     let voter_details = web_block(move || -> AResult<Voter> {
//         Voter::get_one(*voter_id, &pool)
//     }).await?;

//     Ok(voter_id.to_string())
//         // Ok(HttpResponse::Ok().json(voter_info))
// }



// #[get("/voters/details")]
// pub async fn get_all_voters_details(
//     voter_map: Data<VoterMap>,
//     pool: web::Data<DbPool>,
// ) -> actix_web::Result<impl Responder> {
//     let voters = web_block(move || -> AResult<Vec<Voter>> {
//         Voter::get_all(pool.as_ref())
//     }).await?;

//     // Ok(HttpResponse::Ok().json(voter_info))

//     let res = HttpResponse::Ok().json((voters, voter_map));
//     // let res = format!("Hello {}!", user);
//     // let res = format!("Insert successfully");
//     Ok(res)
// }

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
