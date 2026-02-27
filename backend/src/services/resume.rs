use crate::models::*;

pub struct ResumeOptimizer;

impl ResumeOptimizer {
    /// 优化简历
    pub async fn optimize(resume: &str, jd: &str) -> OptimizeResult {
        // 提取 JD 关键词
        let jd_keywords = Self::extract_keywords_from_text(jd);
        
        // 分析简历
        let mut changes = Vec::new();
        
        // 模拟优化建议
        if jd_keywords.iter().any(|k| k.contains("Python")) && !resume.contains("Python") {
            changes.push("建议添加 Python 相关经验".to_string());
        }
        
        if jd_keywords.iter().any(|k| k.contains("团队")) && !resume.contains("团队") {
            changes.push("建议强调团队协作经验".to_string());
        }
        
        if resume.len() < 500 {
            changes.push("简历内容过短，建议补充更多细节".to_string());
        }
        
        if !resume.contains("项目") && !resume.contains("Project") {
            changes.push("建议添加项目经验部分".to_string());
        }
        
        // 计算匹配度
        let match_score = Self::calculate_match_score(resume, jd);
        
        // 模拟优化后的简历
        let optimized = format!(
            r#"# 优化后的简历

{}

## 💡 优化建议

{}
"#,
            resume,
            changes.join("\n- ")
        );
        
        OptimizeResult {
            optimized_resume: optimized,
            changes,
            match_score,
        }
    }
    
    /// 简历评分
    pub async fn score(resume: &str, jd: &str) -> ScoreResult {
        let overall_score = Self::calculate_match_score(resume, jd);
        
        let categories = vec![
            ScoreCategory {
                name: "技能匹配".to_string(),
                score: if resume.contains("技能") || resume.contains("技术") { 85 } else { 60 },
                feedback: "技能描述清晰，建议突出与职位相关的核心技能".to_string(),
            },
            ScoreCategory {
                name: "经验相关度".to_string(),
                score: if resume.contains("年") || resume.contains("经验") { 80 } else { 50 },
                feedback: "工作经验与职位要求匹配度较高".to_string(),
            },
            ScoreCategory {
                name: "关键词覆盖".to_string(),
                score: Self::calculate_keyword_coverage(resume, jd),
                feedback: "已覆盖大部分职位关键词".to_string(),
            },
            ScoreCategory {
                name: "格式规范".to_string(),
                score: if resume.len() > 300 { 75 } else { 50 },
                feedback: "简历结构清晰，建议使用更专业的排版".to_string(),
            },
        ];
        
        let suggestions = vec![
            "建议在开头添加个人简介，突出核心竞争力".to_string(),
            "使用量化数据展示工作成果（如：提升效率 30%）".to_string(),
            "将最相关的经验放在前面".to_string(),
            "确保联系方式完整且专业".to_string(),
        ];
        
        ScoreResult {
            overall_score,
            categories,
            suggestions,
        }
    }
    
    /// 生成求职信
    pub async fn generate_cover_letter(resume: &str, jd: &str) -> String {
        let _company = Self::extract_company(jd).unwrap_or("贵公司");
        let position = Self::extract_position(jd).unwrap_or("该职位");
        
        format!(
            r#"尊敬的招聘经理：

您好！

我写信是希望申请贵公司的{}职位。通过仔细阅读职位描述，我发现自己具备该岗位所需的核心能力和经验。

## 核心优势

{}

## 为什么选择我

- 具备岗位要求的关键技能和经验
- 对该领域充满热情，持续学习进步
- 良好的团队协作和沟通能力
- 注重细节，追求卓越

## 期待机会

我非常期待能有机会与您面谈，进一步展示我的能力和对这份工作的热情。感谢您抽出宝贵时间阅读我的申请。

此致
敬礼！

[您的姓名]
[日期]"#,
            position,
            resume.lines().take(5).collect::<Vec<_>>().join("\n")
        )
    }
    
    /// 提取关键词
    pub async fn extract_keywords(text: &str) -> Vec<Keyword> {
        let common_keywords = vec![
            ("Python", "编程语言", 90),
            ("JavaScript", "编程语言", 85),
            ("Rust", "编程语言", 80),
            ("团队协作", "软技能", 75),
            ("项目管理", "管理能力", 70),
            ("数据分析", "技能", 85),
            ("沟通能力", "软技能", 65),
            ("问题解决", "软技能", 70),
            ("领导力", "管理能力", 75),
            ("创新", "软技能", 60),
        ];
        
        common_keywords
            .into_iter()
            .filter(|(word, _, _)| text.contains(word))
            .map(|(word, category, importance)| Keyword {
                word: word.to_string(),
                importance,
                category: category.to_string(),
            })
            .collect()
    }
    
    // ========== 辅助方法 ==========
    
    fn calculate_match_score(resume: &str, jd: &str) -> u8 {
        let resume_lower = resume.to_lowercase();
        let jd_lower = jd.to_lowercase();
        
        let jd_words: Vec<&str> = jd_lower.split_whitespace().collect();
        let matched = jd_words
            .iter()
            .filter(|word| resume_lower.contains(*word))
            .count();
        
        if jd_words.is_empty() {
            return 50;
        }
        
        let score = (matched as f64 / jd_words.len() as f64 * 100.0) as u8;
        score.min(95).max(30)
    }
    
    fn calculate_keyword_coverage(resume: &str, jd: &str) -> u8 {
        let keywords = Self::extract_keywords_from_text(jd);
        let matched = keywords
            .iter()
            .filter(|k| resume.to_lowercase().contains(&k.to_lowercase()))
            .count();
        
        if keywords.is_empty() {
            return 70;
        }
        
        ((matched as f64 / keywords.len() as f64 * 100.0) as u8).min(95)
    }
    
    fn extract_keywords_from_text(text: &str) -> Vec<String> {
        let common_tech = vec![
            "Python", "JavaScript", "Rust", "Go", "Java", "TypeScript",
            "React", "Vue", "Node", "Docker", "Kubernetes", "AWS",
            "Git", "Linux", "SQL", "MongoDB", "Redis", "PostgreSQL",
            "团队", "管理", "领导", "沟通", "分析", "设计",
        ];
        
        common_tech
            .into_iter()
            .filter(|keyword| text.contains(keyword))
            .map(|s| s.to_string())
            .collect()
    }
    
    fn extract_company(jd: &str) -> Option<&str> {
        if jd.contains("阿里巴巴") { Some("阿里巴巴") }
        else if jd.contains("腾讯") { Some("腾讯") }
        else if jd.contains("字节") { Some("字节跳动") }
        else { None }
    }
    
    fn extract_position(jd: &str) -> Option<&str> {
        if jd.contains("工程师") { Some("工程师") }
        else if jd.contains("开发") { Some("开发") }
        else if jd.contains("经理") { Some("经理") }
        else { None }
    }
}

// ========== 内部结果结构 ==========

pub struct OptimizeResult {
    pub optimized_resume: String,
    pub changes: Vec<String>,
    pub match_score: u8,
}

pub struct ScoreResult {
    pub overall_score: u8,
    pub categories: Vec<ScoreCategory>,
    pub suggestions: Vec<String>,
}
