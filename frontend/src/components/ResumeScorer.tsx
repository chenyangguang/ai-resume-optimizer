import { useState } from 'react'
import axios from 'axios'

interface ScoreCategory {
  name: string
  score: number
  feedback: string
}

interface ScoreResponse {
  overall_score: number
  categories: ScoreCategory[]
  suggestions: string[]
}

function ResumeScorer() {
  const [resume, setResume] = useState('')
  const [jobDescription, setJobDescription] = useState('')
  const [result, setResult] = useState<ScoreResponse | null>(null)
  const [loading, setLoading] = useState(false)

  const handleScore = async () => {
    if (!resume || !jobDescription) {
      alert('请填写简历和职位描述')
      return
    }

    setLoading(true)
    try {
      const response = await axios.post<ScoreResponse>('/api/resume/score', {
        resume,
        job_description: jobDescription,
      })
      setResult(response.data)
    } catch (error) {
      console.error('评分失败:', error)
      alert('评分失败，请重试')
    } finally {
      setLoading(false)
    }
  }

  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-600'
    if (score >= 60) return 'text-yellow-600'
    return 'text-red-600'
  }

  const getScoreBg = (score: number) => {
    if (score >= 80) return 'bg-green-500'
    if (score >= 60) return 'bg-yellow-500'
    return 'bg-red-500'
  }

  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
      {/* Input */}
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            你的简历
          </label>
          <textarea
            value={resume}
            onChange={(e) => setResume(e.target.value)}
            className="input-field h-48 resize-none"
            placeholder="粘贴你的简历内容..."
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            职位描述 (JD)
          </label>
          <textarea
            value={jobDescription}
            onChange={(e) => setJobDescription(e.target.value)}
            className="input-field h-48 resize-none"
            placeholder="粘贴目标职位的描述..."
          />
        </div>

        <button
          onClick={handleScore}
          disabled={loading}
          className="btn-primary w-full"
        >
          {loading ? '评分中...' : '📊 开始评分'}
        </button>
      </div>

      {/* Output */}
      <div>
        {result ? (
          <div className="space-y-4">
            {/* Overall Score */}
            <div className="text-center bg-gradient-to-r from-primary-50 to-blue-50 rounded-lg p-6">
              <div className="text-sm font-medium text-gray-600 mb-2">综合评分</div>
              <div className={`text-6xl font-bold ${getScoreColor(result.overall_score)}`}>
                {result.overall_score}
              </div>
              <div className="text-gray-500 mt-2">/ 100</div>
            </div>

            {/* Category Scores */}
            <div className="space-y-3">
              {result.categories.map((category, index) => (
                <div key={index} className="bg-gray-50 rounded-lg p-4">
                  <div className="flex items-center justify-between mb-2">
                    <span className="font-medium text-gray-900">{category.name}</span>
                    <span className={`font-bold ${getScoreColor(category.score)}`}>
                      {category.score}
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
                    <div
                      className={`${getScoreBg(category.score)} h-2 rounded-full transition-all duration-500`}
                      style={{ width: `${category.score}%` }}
                    />
                  </div>
                  <p className="text-sm text-gray-600">{category.feedback}</p>
                </div>
              ))}
            </div>

            {/* Suggestions */}
            <div>
              <h3 className="font-semibold text-gray-900 mb-3">改进建议</h3>
              <ul className="space-y-2">
                {result.suggestions.map((suggestion, index) => (
                  <li key={index} className="flex items-start bg-yellow-50 rounded p-3">
                    <span className="text-yellow-600 mr-2">💡</span>
                    <span className="text-gray-700 text-sm">{suggestion}</span>
                  </li>
                ))}
              </ul>
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-400">
            <div className="text-center">
              <div className="text-6xl mb-4">📊</div>
              <p>填写简历和职位描述后<br />点击评分按钮查看结果</p>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default ResumeScorer
