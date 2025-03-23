use actix_multipart::Multipart;
use actix_web::{HttpResponse, Error};
use futures::StreamExt;
use polars::prelude::*;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

pub async fn upload_csv(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut filepath = String::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let filename = format!("tmp_{}.csv", Uuid::new_v4());
        filepath = format!("./{}", filename);
        let mut f = File::create(&filepath)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }
    }

    let df = CsvReader::from_path(&filepath)
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;


    let summary = format!(
        "✅ Upload OK | Lignes: {}, Colonnes: {}\nColonnes: {:?}",
        df.height(),
        df.width(),
        df.get_column_names()
    );

    Ok(HttpResponse::Ok().body(summary))
}
