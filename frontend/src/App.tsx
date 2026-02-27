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
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 relative overflow-hidden">
      {/* 动态背景装饰 */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-40 -right-40 w-80 h-80 bg-purple-300 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob"></div>
        <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-300 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob animation-delay-2000"></div>
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-80 h-80 bg-pink-300 rounded-full mix-blend-multiply filter blur-xl opacity-70 animate-blob animation-delay-4000"></div>
      </div>

      {/* Header */}
      <header className="bg-white/80 backdrop-blur-md shadow-sm relative z-10">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <div className="animate-fade-in">
              <h1 className="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                AI 简历优化器
              </h1>
              <p className="text-gray-600 mt-1">
                让你的简历脱颖而出，轻松获得面试机会 ✨
              </p>
            </div>
            <div className="flex items-center space-x-2 animate-slide-in-right">
              <span className="text-sm text-gray-500">Powered by</span>
              <span className="font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">Gitvim</span>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 py-8 relative z-10">
        {/* Tabs */}
        <div className="flex flex-wrap gap-3 mb-6">
          {tabs.map((tab, index) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`px-6 py-3 rounded-xl font-medium transition-all duration-300 transform hover:scale-105 ${
                activeTab === tab.id
                  ? 'bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg scale-105'
                  : 'bg-white/80 backdrop-blur-sm text-gray-700 hover:bg-white hover:shadow-md'
              }`}
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <span className="mr-2 text-xl">{tab.icon}</span>
              {tab.label}
            </button>
          ))}
        </div>

        {/* Tab Content */}
        <div className="bg-white/80 backdrop-blur-md rounded-2xl shadow-xl p-8 animate-fade-in">
          {activeTab === 'generate' && <ResumeGenerator />}
          {activeTab === 'keywords' && <KeywordRecommendations />}
          {activeTab === 'optimize' && <ResumeOptimizer />}
          {activeTab === 'score' && <ResumeScorer />}
          {activeTab === 'cover-letter' && <CoverLetterGenerator />}
        </div>

        {/* Features */}
        <div className="mt-12 grid grid-cols-1 md:grid-cols-5 gap-6">
          {[
            { icon: '📄', title: 'AI 简历生成', desc: '输入信息，一键生成专业简历' },
            { icon: '🎯', title: '智能关键词', desc: '根据职位推荐相关技能关键词' },
            { icon: '🎨', title: '多模板选择', desc: '现代简约、专业经典、创意设计、科技极客' },
            { icon: '📊', title: '智能评分', desc: '多维度评估，精准优化建议' },
            { icon: '✉️', title: '求职信生成', desc: '根据简历和职位自动生成' },
          ].map((feature, idx) => (
            <div 
              key={idx}
              className="bg-white/80 backdrop-blur-sm rounded-xl p-6 shadow-lg hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2 cursor-pointer group"
            >
              <div className="text-5xl mb-4 group-hover:scale-110 transition-transform duration-300">
                {feature.icon}
              </div>
              <h3 className="text-xl font-semibold mb-2 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                {feature.title}
              </h3>
              <p className="text-gray-600 text-sm">
                {feature.desc}
              </p>
            </div>
          ))}
        </div>
      </main>

      {/* Footer */}
      <footer className="mt-12 py-8 bg-white/60 backdrop-blur-sm relative z-10">
        <div className="max-w-7xl mx-auto px-4 text-center">
          <p className="text-gray-600">
            © 2026 <span className="font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">Gitvim</span>. 让 AI 赋能每一个创作者 ✨
          </p>
        </div>
      </footer>
    </div>
  )
}

export default App
