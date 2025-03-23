use actix_web::{HttpResponse, Error};

pub async fn transform_csv() -> Result<HttpResponse, Error> {
    // À venir : PCA, t-SNE, feature selection, binning, etc.
    Ok(HttpResponse::Ok().body("⚙️ Pré-traitement ML à venir (PCA, binning, etc.)"))
}
