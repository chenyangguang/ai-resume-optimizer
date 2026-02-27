use serde::{Deserialize, Serialize};

// ========== 优化简历 ==========

#[derive(Debug, Deserialize)]
pub struct OptimizeResumeRequest {
    pub resume: String,
    pub job_description: String,
}

#[derive(Debug, Serialize)]
pub struct OptimizeResumeResponse {
    pub optimized_resume: String,
    pub changes: Vec<String>,
    pub match_score: u8,
}

// ========== 简历评分 ==========

#[derive(Debug, Deserialize)]
pub struct ScoreResumeRequest {
    pub resume: String,
    pub job_description: String,
}

#[derive(Debug, Serialize)]
pub struct ScoreResumeResponse {
    pub overall_score: u8,
    pub categories: Vec<ScoreCategory>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ScoreCategory {
    pub name: String,
    pub score: u8,
    pub feedback: String,
}

// ========== 生成求职信 ==========

#[derive(Debug, Deserialize)]
pub struct CoverLetterRequest {
    pub resume: String,
    pub job_description: String,
}

#[derive(Debug, Serialize)]
pub struct CoverLetterResponse {
    pub cover_letter: String,
}

// ========== 提取关键词 ==========

#[derive(Debug, Deserialize)]
pub struct ExtractKeywordsRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ExtractKeywordsResponse {
    pub keywords: Vec<Keyword>,
}

#[derive(Debug, Serialize)]
pub struct Keyword {
    pub word: String,
    pub importance: u8,
    pub category: String,
}
