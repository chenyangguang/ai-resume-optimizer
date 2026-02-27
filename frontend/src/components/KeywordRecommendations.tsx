import { useState } from 'react'

interface KeywordCategory {
  category: string
  keywords: string[]
}

const jobKeywords: Record<string, KeywordCategory[]> = {
  'Python工程师': [
    {
      category: '后端框架',
      keywords: ['Django', 'Flask', 'FastAPI', 'Tornado', 'Sanic']
    },
    {
      category: '数据库',
      keywords: ['PostgreSQL', 'MySQL', 'Redis', 'MongoDB', 'Elasticsearch']
    },
    {
      category: '工具',
      keywords: ['Docker', 'Kubernetes', 'Celery', 'Gunicorn', 'Nginx']
    },
    {
      category: '其他',
      keywords: ['REST API', '微服务', '单元测试', 'CI/CD', 'Git']
    }
  ],
  '前端工程师': [
    {
      category: '框架',
      keywords: ['React', 'Vue.js', 'Angular', 'Next.js', 'Nuxt.js']
    },
    {
      category: '语言',
      keywords: ['TypeScript', 'JavaScript', 'HTML5', 'CSS3', 'Sass']
    },
    {
      category: '工具',
      keywords: ['Webpack', 'Vite', 'ESLint', 'Prettier', 'Git']
    },
    {
      category: '其他',
      keywords: ['响应式设计', '性能优化', '跨浏览器兼容', 'SEO', 'PWA']
    }
  ],
  '全栈工程师': [
    {
      category: '前端',
      keywords: ['React', 'Vue.js', 'TypeScript', 'Tailwind CSS', 'Next.js']
    },
    {
      category: '后端',
      keywords: ['Node.js', 'Python', 'Django', 'FastAPI', 'Express']
    },
    {
      category: '数据库',
      keywords: ['PostgreSQL', 'MongoDB', 'Redis', 'GraphQL', 'Prisma']
    },
    {
      category: '其他',
      keywords: ['Docker', 'Kubernetes', 'AWS', 'CI/CD', '敏捷开发']
    }
  ],
  '产品经理': [
    {
      category: '产品设计',
      keywords: ['用户研究', '原型设计', 'Figma', 'Sketch', 'Axure']
    },
    {
      category: '数据分析',
      keywords: ['SQL', 'Excel', 'Tableau', '数据分析', 'A/B测试']
    },
    {
      category: '项目管理',
      keywords: ['Scrum', 'Jira', '需求文档', '竞品分析', '用户故事']
    },
    {
      category: '其他',
      keywords: ['跨部门协作', '演讲能力', 'PPT制作', '市场调研', '商业化']
    }
  ],
  'UI/UX设计师': [
    {
      category: '设计工具',
      keywords: ['Figma', 'Sketch', 'Adobe XD', 'Photoshop', 'Illustrator']
    },
    {
      category: '设计能力',
      keywords: ['用户研究', '交互设计', '视觉设计', '设计系统', '原型制作']
    },
    {
      category: '前端',
      keywords: ['HTML', 'CSS', '响应式设计', '设计交付', 'Design Tokens']
    },
    {
      category: '其他',
      keywords: ['用户测试', '可用性分析', '信息架构', '动效设计', '品牌设计']
    }
  ]
}

function KeywordRecommendations() {
  const [jobTitle, setJobTitle] = useState('')
  const [selectedKeywords, setSelectedKeywords] = useState<string[]>([])
  const [recommendedKeywords, setRecommendedKeywords] = useState<KeywordCategory[]>([])

  const handleJobTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const title = e.target.value
    setJobTitle(title)

    // 查找匹配的职位关键词
    let matched = false
    for (const [key, keywords] of Object.entries(jobKeywords)) {
      if (title.includes(key) || key.includes(title)) {
        setRecommendedKeywords(keywords)
        matched = true
        break
      }
    }

    if (!matched) {
      setRecommendedKeywords([])
    }
  }

  const toggleKeyword = (keyword: string) => {
    if (selectedKeywords.includes(keyword)) {
      setSelectedKeywords(selectedKeywords.filter(k => k !== keyword))
    } else {
      setSelectedKeywords([...selectedKeywords, keyword])
    }
  }

  const copySelectedKeywords = () => {
    const text = selectedKeywords.join(', ')
    navigator.clipboard.writeText(text)
    alert('已复制到剪贴板：\n' + text)
  }

  const clearSelected = () => {
    setSelectedKeywords([])
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-gray-900 mb-2">🎯 智能关键词推荐</h2>
        <p className="text-gray-600">输入职位名称，获取相关技能关键词建议</p>
      </div>

      {/* 职位输入 */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          职位名称
        </label>
        <input
          type="text"
          value={jobTitle}
          onChange={handleJobTitleChange}
          className="input-field"
          placeholder="例如：Python工程师、前端工程师、产品经理..."
        />
        <p className="text-xs text-gray-500 mt-1">
          支持的职位：Python工程师、前端工程师、全栈工程师、产品经理、UI/UX设计师
        </p>
      </div>

      {/* 推荐的关键词 */}
      {recommendedKeywords.length > 0 && (
        <div className="space-y-4">
          <h3 className="font-semibold text-gray-900">推荐关键词</h3>
          {recommendedKeywords.map((category, idx) => (
            <div key={idx}>
              <h4 className="text-sm font-medium text-gray-700 mb-2">{category.category}</h4>
              <div className="flex flex-wrap gap-2">
                {category.keywords.map((keyword) => (
                  <button
                    key={keyword}
                    onClick={() => toggleKeyword(keyword)}
                    className={`px-3 py-1 rounded-full text-sm transition-colors ${
                      selectedKeywords.includes(keyword)
                        ? 'bg-primary-500 text-white'
                        : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
                    }`}
                  >
                    {keyword}
                  </button>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* 已选择的关键词 */}
      {selectedKeywords.length > 0 && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold text-gray-900">
              已选择 ({selectedKeywords.length})
            </h3>
            <div className="flex gap-2">
              <button
                onClick={copySelectedKeywords}
                className="px-4 py-2 text-sm bg-primary-100 hover:bg-primary-200 text-primary-700 rounded transition-colors"
              >
                📋 复制选中
              </button>
              <button
                onClick={clearSelected}
                className="px-4 py-2 text-sm bg-gray-100 hover:bg-gray-200 rounded transition-colors"
              >
                清空
              </button>
            </div>
          </div>
          <div className="bg-primary-50 rounded-lg p-4">
            <div className="flex flex-wrap gap-2">
              {selectedKeywords.map((keyword) => (
                <span
                  key={keyword}
                  className="px-3 py-1 bg-primary-500 text-white rounded-full text-sm cursor-pointer"
                  onClick={() => toggleKeyword(keyword)}
                >
                  {keyword} ×
                </span>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* 使用提示 */}
      <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
        <h4 className="font-semibold text-blue-900 mb-2">💡 使用建议</h4>
        <ul className="text-sm text-blue-800 space-y-1">
          <li>• 点击关键词可以选中/取消选中</li>
          <li>• 选中的关键词会显示为蓝色标签</li>
          <li>• 点击"复制选中"可以复制到剪贴板</li>
          <li>• 将关键词粘贴到简历生成器的技能栏</li>
        </ul>
      </div>
    </div>
  )
}

export default KeywordRecommendations
