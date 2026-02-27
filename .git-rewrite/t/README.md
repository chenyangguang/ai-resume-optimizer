# AI 简历优化器 🎯

**智能简历优化助手 - 让你的简历脱颖而出**

---

## 🚀 功能特性

### 1. 简历优化 ✨
- 根据 JD 自动优化简历
- 智能匹配关键词
- 生成优化建议
- 显示匹配度评分

### 2. 简历评分 📊
- 多维度评分系统
  - 技能匹配
  - 经验相关度
  - 关键词覆盖
  - 格式规范
- 详细的改进建议

### 3. 求职信生成 ✉️
- 自动生成专业求职信
- 基于简历和 JD
- 一键复制

---

## 💻 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React 18 + TypeScript + Tailwind CSS + Vite |
| 后端 | Rust (Axum) |
| 数据库 | PostgreSQL + Redis |
| AI | Mock AI（快速验证）|

---

## 🎯 商业模式

| 版本 | 价格 | 功能 |
|------|------|------|
| **免费版** | $0 | 5 次优化/月 |
| **专业版** | $19/月 | 100 次优化 + 求职信 |
| **企业版** | $99/月 | 无限次 + API + 团队 |

---

## 🚀 快速开始

### 后端

```bash
cd backend
cargo run --release
```

访问：http://localhost:3002

### 前端

```bash
cd frontend
npm install
npm run dev
```

访问：http://localhost:5173

---

## 📡 API 端点

```
POST /api/resume/optimize       - 优化简历
POST /api/resume/score          - 简历评分
POST /api/resume/cover-letter   - 生成求职信
POST /api/resume/keywords       - 提取关键词
GET  /api/health                - 健康检查
```

---

## 📊 收入预测

```
目标：200 付费用户/月

专业版：150 用户 × $19 = $2,850/月
企业版：50 用户 × $99 = $4,950/月

月收入：$7,800
年收入：$93,600

扩展目标：500 付费用户
年收入：$234,000
```

---

## 🎯 开发进度

- [x] 后端 API（4个端点）
- [x] 前端界面（React + TS）
- [ ] 用户认证系统
- [ ] 订阅支付集成
- [ ] 部署上线
- [ ] 市场推广

---

## 📝 项目结构

```
ai-resume-optimizer/
├── backend/           # Rust 后端
│   ├── src/
│   │   ├── routes/   # API 路由
│   │   ├── services/ # 业务逻辑
│   │   └── models/   # 数据模型
│   └── Cargo.toml
├── frontend/          # React 前端
│   ├── src/
│   │   ├── components/  # UI 组件
│   │   ├── App.tsx     # 主应用
│   │   └── main.tsx    # 入口
│   └── package.json
└── README.md
```

---

## 🔥 竞争优势

1. **快速** - Mock AI 零延迟
2. **简单** - 无需注册即可试用
3. **专业** - 多维度评分系统
4. **便宜** - 比 CareerBuilder、Indeed 便宜 90%

---

## 📅 上线计划

| 时间 | 里程碑 |
|------|--------|
| Week 1 | ✅ MVP 完成 |
| Week 2 | 部署上线 + 用户测试 |
| Week 3-4 | 收集反馈 + 优化 |
| Month 2 | 付费功能上线 |
| Month 3 | 市场推广 |

---

**目标：年入 $1,000,000** 💰

---

_创建时间：2026-02-27_
