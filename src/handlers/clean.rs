use actix_web::{HttpResponse, Error};
use polars::prelude::*;
use serde::Serialize;
use std::fs::File;
use std::path::Path;

#[derive(Serialize)]
pub struct CleanSummary {
    original_shape: (usize, usize),
    final_shape: (usize, usize),
    removed_constant_columns: Vec<String>,
    filled_nulls: usize,
    normalized_columns: Vec<String>,
}

pub async fn clean_csv() -> Result<HttpResponse, Error> {
    let path = Path::new("./last_uploaded.csv");

    if !path.exists() {
        return Ok(HttpResponse::BadRequest().body("❌ Fichier last_uploaded.csv introuvable"));
    }

    println!("🚀 Nettoyage lancé...");

    let mut df = CsvReader::from_path(path)
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Erreur lecture CSV: {}", e)))?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()
        .map_err(|e| actix_web::error::ErrorBadRequest(format!("Erreur parsing: {}", e)))?;

    let original_shape = (df.height(), df.width());

    // 🔹 Suppression colonnes constantes
    let mut removed = vec![];
    let col_names: Vec<String> = df.get_column_names().iter().map(|s| s.to_string()).collect();
    for col in col_names.iter() {
        let s = df.column(col).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
        if s.n_unique().unwrap_or(2) <= 1 {
            let _ = df.drop_in_place(col).unwrap();

            removed.push(col.clone());
        }
    }

    // 🔹 Remplissage des nulls
    let mut filled_nulls = 0;
    for name in col_names.iter() {
        let col = df.column(name).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
        if col.null_count() > 0 {
            if let Ok(values) = col.f64() {
                if let Some(mean_val) = values.mean() {
                    let filled = values
                        .into_iter()
                        .map(|v| Some(v.unwrap_or(mean_val)))
                        .collect::<Float64Chunked>();
                    df.replace(name, Series::new(name, filled)).unwrap();
                    filled_nulls += 1;
                }
            }
        }
    }

    // 🔹 Normalisation
    let mut normalized_columns = vec![];
    for name in col_names.iter() {
        let col = df.column(name).map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
        if let Ok(arr) = col.f64() {
            if let (Some(min), Some(max)) = (arr.min(), arr.max()) {
                if (max - min).abs() > 1e-9 {
                    let norm = arr
                        .into_iter()
                        .map(|v| Some((v.unwrap_or(min) - min) / (max - min)))
                        .collect::<Float64Chunked>();
                    df.replace(name, Series::new(name, norm)).unwrap();
                    normalized_columns.push(name.clone());
                }
            }
        }
    }

    // 🔹 Export CSV nettoyé
    let mut file = File::create("cleaned.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    let summary = CleanSummary {
        original_shape,
        final_shape: (df.height(), df.width()),
        removed_constant_columns: removed,
        filled_nulls,
        normalized_columns,
    };

    println!("✅ Nettoyage terminé");
    Ok(HttpResponse::Ok().json(summary))
}
