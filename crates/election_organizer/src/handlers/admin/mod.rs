use crate::prelude::*;
pub use election_shared::models::vote_record::VoteRecord;
// use crate::{
//     schema::verifier::{
//         self,
//         // id,
//         dsl,
//         // name,
//         wallet_address,
//         // max_upload_count,
//         // description,
//     },
//     models::verifier::{
//     //     NewVerifier,
//         Verifier,
//         // AdminReqVerifier,
//         NewVerifier,
//         // VerifierMap,
//     },
//     verifier::Verifier,
// };

#[post("/toggle_election_status")]
pub async fn toggle_election_status(
    election_closed: Data<ElectionCloseStatus>,
) -> actix_web::Result<impl Responder> {
    let status_to_be_set = !election_closed.load(Ordering::Relaxed);
    web_block(move || -> AResult<()> {
                election_closed.store(status_to_be_set, Ordering::Relaxed);
                Ok(())
    }).await?;

    // let res = format!("election_closed: {status_to_be_set}");
    let res = HttpResponse::Ok().json(status_to_be_set);
    Ok(res)
}


#[post("/generate_election_result")]
pub async fn generate_election_result(
    config: web::Data<OrganizerConfig>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    // let sqlite3_file_path = config.sqlite3_file_path.clone();
    let _ = web_block(move || -> AResult<()> {
    // let data_dir = Path::new(&sqlite3_file_path);
    let data_dir = Path::new(&config.sqlite3_file_path);
        let target_dir = data_dir.parent()
            // .ok_or_else()?
            .ok_or(msg("data_dir don't have parent"))?
            .join("pub")
            .join("files");

        fs::create_dir_all(&target_dir)?;

        let file_path = target_dir.join("vote_record.csv");

        let file = File::create(&file_path)?;

        // file.write_all(b"\xEF\xBB\xBF")?; // UTF-8 BOM


        let all_vote_records: Vec<VoteRecord> = VoteRecord::get_all(pool.get_ref())?;
        let mut wtr = WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::Necessary)
            .from_writer(file);

        for record in all_vote_records {
            wtr.serialize(record)?;
        }

        // 8. 刷新缓冲区，确保所有数据写入文件
        wtr.flush()?;
        Ok(())
    }).await?;

    // let res = HttpResponse::Ok().json(all_vote_records);
    // Ok(res)
    // let res = HttpResponse::Ok().json(config.vote_requirements.clone());
    Ok("finished")
}
// #[post("/upsert_verifier")]
// pub async fn upsert_verifier(
//     verifier_map: Data<VerifierMap>,
//     mut verifier_info: web::Json<NewVerifier>,
//     pool: web::Data<DbPool>,
// ) -> actix_web::Result<String> {
//     if verifier_info.api_key.is_none() {
//         let api_key = passwords::PasswordGenerator::new()
//             .length(128)
//             .numbers(true)
//             .lowercase_letters(true)
//             .uppercase_letters(true)
//             .symbols(false)
//             .spaces(false)
//             .exclude_similar_characters(true)
//             .strict(true)
//             .generate_one()
//             .unwrap();

//         verifier_info.api_key = Some(api_key);
//     }

//     // let verifier_info = Verifier::from(verifier_info.into_inner());
//     web_block(move || -> AResult<()> {
//         diesel::insert_into(verifier::table)
//             .values(&*verifier_info)
//             .on_conflict(wallet_address)
//             .do_update()
//             .set(&*verifier_info)
//             .execute(&mut pool.conn()?)?;

//         verifier_map.refresh(pool.as_ref())
//         // verifiers = Verifier::get_all(pool.as_ref());
//     }).await?;


//     // let res = verifier_map.get();
//     let res = "Upsert successfully".to_string();
//     Ok(res)
//     // Ok(HttpResponse::Ok().json(verifier_info))
// }


// #[get("/verifiers/details")]
// pub async fn get_all_verifiers_details(
//     verifier_map: Data<VerifierMap>,
//     pool: web::Data<DbPool>,
// ) -> actix_web::Result<impl Responder> {
//     let verifiers = web_block(move || -> AResult<Vec<Verifier>> {
//         Verifier::get_all(pool.as_ref())
//     }).await?;

//     // Ok(HttpResponse::Ok().json(verifier_info))

//     let res = HttpResponse::Ok().json((verifiers, verifier_map));
//     // let res = format!("Hello {}!", user);
//     // let res = format!("Insert successfully");
//     Ok(res)
// }



// #[delete("/verifier/{id}")]
// pub async fn delete_verifier(
//     verifier_map: Data<VerifierMap>,
//     verifier_id: web::Path<i32>,
//     pool: web::Data<DbPool>,
// ) -> actix_web::Result<String> {
//     web_block(move || -> AResult<()> {
//         diesel::delete(dsl::verifier.filter(dsl::id.eq(*verifier_id)))
//             .execute(&mut pool.conn()?)?;

//         verifier_map.refresh(pool.as_ref())
//     }).await?;


//     let res = "Delete successfully".to_string();
//     Ok(res)
// }


