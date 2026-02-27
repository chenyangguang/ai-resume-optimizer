import { useState } from 'react'
import ResumeGenerator from './components/ResumeGenerator'
import KeywordRecommendations from './components/KeywordRecommendations'
import ResumeOptimizer from './components/ResumeOptimizer'
import ResumeScorer from './components/ResumeScorer'
import CoverLetterGenerator from './components/CoverLetterGenerator'

type Tab = 'generate' | 'keywords' | 'optimize' | 'score' | 'cover-letter'

function App() {
  const [activeTab, setActiveTab] = useState<Tab>('optimize')

  const tabs = [
    { id: 'generate' as Tab, label: '简历生成器', icon: '📄' },
    { id: 'keywords' as Tab, label: '关键词推荐', icon: '🎯' },
    { id: 'optimize' as Tab, label: '简历优化', icon: '✨' },
    { id: 'score' as Tab, label: '简历评分', icon: '📊' },
    { id: 'cover-letter' as Tab, label: '求职信生成', icon: '✉️' },
  ]

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">
                AI 简历优化器
              </h1>
              <p className="text-gray-600 mt-1">
                让你的简历脱颖而出，轻松获得面试机会
              </p>
            </div>
            <div className="flex items-center space-x-2">
              <span className="text-sm text-gray-500">Powered by</span>
              <span className="font-semibold text-primary-600">Gitvim</span>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 py-8">
        {/* Tabs */}
        <div className="flex space-x-2 mb-6">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`px-6 py-3 rounded-lg font-medium transition-all duration-200 ${
                activeTab === tab.id
                  ? 'bg-primary-600 text-white shadow-lg'
                  : 'bg-white text-gray-700 hover:bg-gray-50'
              }`}
            >
              <span className="mr-2">{tab.icon}</span>
              {tab.label}
            </button>
          ))}
        </div>

        {/* Tab Content */}
        <div className="card">
          {activeTab === 'generate' && <ResumeGenerator />}
          {activeTab === 'keywords' && <KeywordRecommendations />}
          {activeTab === 'optimize' && <ResumeOptimizer />}
          {activeTab === 'score' && <ResumeScorer />}
          {activeTab === 'cover-letter' && <CoverLetterGenerator />}
        </div>

        {/* Features */}
        <div className="mt-12 grid grid-cols-1 md:grid-cols-5 gap-6">
          <div className="card">
            <div className="text-4xl mb-4">📄</div>
            <h3 className="text-xl font-semibold mb-2">AI 简历生成</h3>
            <p className="text-gray-600">
              输入信息，一键生成专业简历
            </p>
          </div>
          <div className="card">
            <div className="text-4xl mb-4">🎯</div>
            <h3 className="text-xl font-semibold mb-2">智能关键词</h3>
            <p className="text-gray-600">
              根据职位推荐相关技能关键词
            </p>
          </div>
          <div className="card">
            <div className="text-4xl mb-4">🎨</div>
            <h3 className="text-xl font-semibold mb-2">多模板选择</h3>
            <p className="text-gray-600">
              现代简约、专业经典、创意设计、科技极客
            </p>
          </div>
          <div className="card">
            <div className="text-4xl mb-4">📊</div>
            <h3 className="text-xl font-semibold mb-2">智能评分</h3>
            <p className="text-gray-600">
              多维度评估，精准优化建议
            </p>
          </div>
          <div className="card">
            <div className="text-4xl mb-4">✉️</div>
            <h3 className="text-xl font-semibold mb-2">求职信生成</h3>
            <p className="text-gray-600">
              根据简历和职位自动生成
            </p>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="mt-12 py-8 bg-white">
        <div className="max-w-7xl mx-auto px-4 text-center text-gray-600">
          <p>© 2026 Gitvim. 让 AI 赋能每一个创作者</p>
        </div>
      </footer>
    </div>
  )
}

export default App
