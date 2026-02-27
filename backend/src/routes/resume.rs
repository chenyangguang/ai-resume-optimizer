use axum::{Json, extract::State};
use serde_json::{json, Value};
use crate::{models::*, utils::AppState, services::ResumeOptimizer};

pub async fn optimize(
    State(_state): State<AppState>,
    Json(req): Json<OptimizeResumeRequest>,
) -> Json<OptimizeResumeResponse> {
    let result = ResumeOptimizer::optimize(&req.resume, &req.job_description).await;
    
    Json(OptimizeResumeResponse {
        optimized_resume: result.optimized_resume,
        changes: result.changes,
        match_score: result.match_score,
    })
}

pub async fn score(
    State(_state): State<AppState>,
    Json(req): Json<ScoreResumeRequest>,
) -> Json<ScoreResumeResponse> {
    let result = ResumeOptimizer::score(&req.resume, &req.job_description).await;
    
    Json(ScoreResumeResponse {
        overall_score: result.overall_score,
        categories: result.categories,
        suggestions: result.suggestions,
    })
}

pub async fn generate_cover_letter(
    State(_state): State<AppState>,
    Json(req): Json<CoverLetterRequest>,
) -> Json<CoverLetterResponse> {
    let letter = ResumeOptimizer::generate_cover_letter(&req.resume, &req.job_description).await;
    
    Json(CoverLetterResponse {
        cover_letter: letter,
    })
}

pub async fn extract_keywords(
    State(_state): State<AppState>,
    Json(req): Json<ExtractKeywordsRequest>,
) -> Json<ExtractKeywordsResponse> {
    let keywords = ResumeOptimizer::extract_keywords(&req.text).await;
    
    Json(ExtractKeywordsResponse {
        keywords,
    })
}
