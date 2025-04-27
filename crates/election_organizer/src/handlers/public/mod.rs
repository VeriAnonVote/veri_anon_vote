use crate::prelude::*;
use election_shared::{
    schema::vote_record::{
        self,
        dsl,
    },
    models::vote_record::{
        NewVoteRecord,
        VoteRecord,
    },
};


// #[instrument]
#[post("/vote_record")]
pub async fn insert_vote_record(
    new_vote: web::Json<NewVoteRecord>,
    pub_ring: Data<PubRing>,
    pool: web::Data<DbPool>,
    election_closed: Data<ElectionCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let vote_record_id = web_block(move || -> AResult<i32> {
    // let res = web_block(move || -> AResult<i32> {
        if election_closed.load(Ordering::Relaxed) {
            return Err(msg("election closed"));
        }

        let conn = &mut pool.get()?;
        conn.transaction::<(), Error, _>(|conn| {
            let all_vote_records = dsl::vote_record.filter(vote_record::id.gt(0))
                .load(conn)?;

            if !new_vote.check_signature_uniqueness(&all_vote_records, &pub_ring) {
                bail!("The election has not yet started OR You have already voted.");
            }
            drop(all_vote_records);

            if !new_vote.verify_signature() {
                bail!("Bad Signature");
            }

            diesel::insert_into(vote_record::table)
                .values(&*new_vote)
                .execute(conn)?;

            Ok(())
        })?;

        let vote_record_id = dsl::vote_record.filter(vote_record::ring_sig.eq(&new_vote.ring_sig))
            .select(vote_record::id)
            .first(conn)
            .map_err(msg)?;

        Ok(vote_record_id)
    }).await?;
// info!("{:#?}", res);
    let res = HttpResponse::Ok().json(vote_record_id);
    Ok(res)
}



#[get("/all_vote_records")]
pub async fn get_all_vote_records(
    pool: web::Data<DbPool>,
    election_closed: Data<ElectionCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let all_vote_records = web_block(move || -> AResult<Vec<VoteRecord>> {
        if !election_closed.load(Ordering::Relaxed) {
            return Err(msg("election has not closed"));
        }

        VoteRecord::get_all(pool.get_ref())
    }).await?;

    let res = HttpResponse::Ok().json(all_vote_records);
    Ok(res)
}



#[get("/vote_record/{record_id}")]
pub async fn get_one_vote_record(
    record_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<HttpResponse> {
    let all_vote_records = web_block(move || -> AResult<VoteRecord> {
        VoteRecord::get_one(*record_id, pool.get_ref())
    }).await?;

    Ok(HttpResponse::Ok().json(all_vote_records))
}



#[get("/pub_ring")]
pub async fn get_pub_ring(
    pub_ring: Data<PubRing>,
) -> actix_web::Result<impl Responder> {
    let res = HttpResponse::Ok().json(pub_ring);
    Ok(res)
}



#[get("/vote_requirements")]
pub async fn get_vote_requirements(
    config: web::Data<OrganizerConfig>,
) -> actix_web::Result<impl Responder> {
    let res = HttpResponse::Ok().json(config.vote_requirements.clone());
    Ok(res)
}
