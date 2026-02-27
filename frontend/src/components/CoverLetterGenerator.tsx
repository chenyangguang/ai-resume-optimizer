import { useState } from 'react'
import axios from 'axios'

interface CoverLetterResponse {
  cover_letter: string
}

function CoverLetterGenerator() {
  const [resume, setResume] = useState('')
  const [jobDescription, setJobDescription] = useState('')
  const [result, setResult] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  const handleGenerate = async () => {
    if (!resume || !jobDescription) {
      alert('请填写简历和职位描述')
      return
    }

    setLoading(true)
    try {
      const response = await axios.post<CoverLetterResponse>('/api/resume/cover-letter', {
        resume,
        job_description: jobDescription,
      })
      setResult(response.data.cover_letter)
    } catch (error) {
      console.error('生成失败:', error)
      alert('生成失败，请重试')
    } finally {
      setLoading(false)
    }
  }

  const handleCopy = () => {
    if (result) {
      navigator.clipboard.writeText(result)
      alert('已复制到剪贴板')
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
          onClick={handleGenerate}
          disabled={loading}
          className="btn-primary w-full"
        >
          {loading ? '生成中...' : '✉️ 生成求职信'}
        </button>
      </div>

      {/* Output */}
      <div>
        {result ? (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="font-semibold text-gray-900">生成的求职信</h3>
              <button
                onClick={handleCopy}
                className="px-4 py-2 text-sm bg-gray-100 hover:bg-gray-200 rounded transition-colors"
              >
                📋 复制
              </button>
            </div>
            <div className="bg-gray-50 rounded-lg p-6 whitespace-pre-wrap text-gray-800 max-h-[600px] overflow-y-auto border">
              {result}
            </div>
            <div className="bg-blue-50 rounded-lg p-4 text-sm text-blue-800">
              💡 <strong>提示：</strong>请根据实际情况修改求职信中的个人信息、联系方式等细节。
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-400">
            <div className="text-center">
              <div className="text-6xl mb-4">✉️</div>
              <p>填写简历和职位描述后<br />点击生成按钮获取求职信</p>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default CoverLetterGenerator
