use actix_web::{HttpResponse, Error};
use polars::prelude::*;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct ColumnInfo {
    name: String,
    dtype: String,
    null_percentage: f64,
    examples: Vec<String>,
}

#[derive(Serialize)]
struct PreviewResponse {
    preview: Vec<Vec<String>>,
    columns: Vec<ColumnInfo>,
}

pub async fn preview_csv() -> Result<HttpResponse, Error> {
    let path = Path::new("./last_uploaded.csv");

    let df = CsvReader::from_path(path)
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;

    let mut preview = vec![];
    let head = df.head(Some(100));
    for i in 0..head.height() {
        let row = head.get_row(i).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
        preview.push(
            row.0.iter().map(|v| format!("{}", v)).collect::<Vec<String>>(),
        );
    }

    let total_rows = df.height() as f64;

    let columns_info = df.get_columns().iter().map(|s| {
        let nulls = s.null_count() as f64;
        let examples = s.head(Some(3))
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>();

        ColumnInfo {
            name: s.name().to_string(),
            dtype: s.dtype().to_string(),
            null_percentage: (nulls / total_rows * 100.0 * 10.0).round() / 10.0,
            examples,
        }
    }).collect::<Vec<_>>();

    let resp = PreviewResponse { preview, columns: columns_info };
    Ok(HttpResponse::Ok().json(resp))
}
