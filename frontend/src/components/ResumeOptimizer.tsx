import { useState } from 'react'
import axios from 'axios'

interface OptimizeResponse {
  optimized_resume: string
  changes: string[]
  match_score: number
}

function ResumeOptimizer() {
  const [resume, setResume] = useState('')
  const [jobDescription, setJobDescription] = useState('')
  const [result, setResult] = useState<OptimizeResponse | null>(null)
  const [loading, setLoading] = useState(false)

  const handleOptimize = async () => {
    if (!resume || !jobDescription) {
      alert('请填写简历和职位描述')
      return
    }

    setLoading(true)
    try {
      const response = await axios.post<OptimizeResponse>('/api/resume/optimize', {
        resume,
        job_description: jobDescription,
      })
      setResult(response.data)
    } catch (error) {
      console.error('优化失败:', error)
      alert('优化失败，请重试')
    } finally {
      setLoading(false)
    }
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
          onClick={handleOptimize}
          disabled={loading}
          className="btn-primary w-full"
        >
          {loading ? '优化中...' : '✨ 开始优化'}
        </button>
      </div>

      {/* Output */}
      <div>
        {result ? (
          <div className="space-y-4">
            {/* Match Score */}
            <div className="bg-gradient-to-r from-primary-50 to-blue-50 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium text-gray-700">匹配度</span>
                <span className="text-2xl font-bold text-primary-600">
                  {result.match_score}%
                </span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-2">
                <div
                  className="bg-primary-600 h-2 rounded-full transition-all duration-500"
                  style={{ width: `${result.match_score}%` }}
                />
              </div>
            </div>

            {/* Changes */}
            <div>
              <h3 className="font-semibold text-gray-900 mb-2">优化建议</h3>
              <ul className="space-y-2">
                {result.changes.map((change, index) => (
                  <li key={index} className="flex items-start">
                    <span className="text-primary-600 mr-2">•</span>
                    <span className="text-gray-700">{change}</span>
                  </li>
                ))}
              </ul>
            </div>

            {/* Optimized Resume */}
            <div>
              <h3 className="font-semibold text-gray-900 mb-2">优化后的简历</h3>
              <div className="bg-gray-50 rounded-lg p-4 whitespace-pre-wrap text-sm text-gray-800 max-h-96 overflow-y-auto">
                {result.optimized_resume}
              </div>
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-400">
            <div className="text-center">
              <div className="text-6xl mb-4">📄</div>
              <p>填写简历和职位描述后<br />点击优化按钮查看结果</p>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default ResumeOptimizer
