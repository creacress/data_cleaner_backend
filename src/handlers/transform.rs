use actix_web::{HttpResponse, Error};
use polars::prelude::*;
use smartcore::decomposition::pca::*;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linalg::basic::arrays::Array2;
use serde::Serialize;
use std::fs::File;
use std::path::Path;
use std::io::Write;

#[derive(Serialize)]
struct PCA3D {
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    labels: Vec<String>,
}

pub async fn transform_csv() -> Result<HttpResponse, Error> {
    let path = Path::new("./cleaned.csv");
    if !path.exists() {
        return Ok(HttpResponse::BadRequest().body("❌ Fichier cleaned.csv introuvable"));
    }

    let df = CsvReader::from_path(path)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let numeric: Vec<Series> = df.get_columns()
        .iter()
        .filter(|s| matches!(s.dtype(), DataType::Float64))
        .cloned()
        .collect();

    let df_num = DataFrame::new(numeric)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let data: Vec<Vec<f64>> = df_num.get_columns()
        .iter()
        .map(|s| s.f64().unwrap().into_no_null_iter().collect::<Vec<f64>>())
        .collect();

    let data_matrix = DenseMatrix::from_2d_vec(
        &data[0]
            .iter()
            .enumerate()
            .map(|(i, _)| data.iter().map(|col| col[i]).collect::<Vec<f64>>())
            .collect::<Vec<Vec<f64>>>()
    );

    let pca = PCA::fit(&data_matrix, PCAParameters::default().with_n_components(3))
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let transformed = pca.transform(&data_matrix)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];

    for row in transformed.row_iter() {
        // Convertir le "row" en slice pour accéder à ses éléments
        let row_vec: Vec<f64> = row.iterator(0).map(|&v| v).collect(); // Utiliser iterator pour accéder aux éléments
        x.push(row_vec[0]);
        y.push(row_vec[1]);
        z.push(row_vec[2]);
    }
    
    
    
    
    let labels = (0..x.len()).map(|i| format!("Point {}", i)).collect();
    let result = PCA3D { x, y, z, labels };

    let json = serde_json::to_string(&result)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    let mut file = File::create("./pca_result.json")
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    file.write_all(json.as_bytes())
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().body("✅ PCA 3D effectué et fichier JSON généré."))
}
