use actix_web::{HttpResponse, Error};

pub async fn analyze_csv() -> Result<HttpResponse, Error> {
    // À venir : analyse des corrélations, ANOVA, types, etc.
    Ok(HttpResponse::Ok().body("📊 Analyse avancée à venir (corrélation, variance, etc.)"))
}
