import { useState } from 'react'

interface UsageStatsProps {
  tier: string
  usageCount: number
  usageLimit: number
}

function UsageStats({ tier, usageCount, usageLimit }: UsageStatsProps) {
  const percentage = (usageCount / usageLimit) * 100
  const remaining = usageLimit - usageCount
  
  const tierLabels: Record<string, string> = {
    free: '免费版',
    pro: '专业版',
    enterprise: '企业版',
  }
  
  const tierColors: Record<string, string> = {
    free: 'from-gray-400 to-gray-500',
    pro: 'from-blue-500 to-purple-500',
    enterprise: 'from-yellow-500 to-orange-500',
  }

  return (
    <div className="bg-white/80 backdrop-blur-md rounded-xl shadow-lg p-6">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">
            使用情况
          </h3>
          <p className="text-sm text-gray-600">
            {tierLabels[tier] || tier}
          </p>
        </div>
        <div className={`px-4 py-2 rounded-full bg-gradient-to-r ${tierColors[tier]} text-white text-sm font-medium`}>
          {tierLabels[tier]}
        </div>
      </div>
      
      {/* 进度条 */}
      <div className="mb-3">
        <div className="flex justify-between text-sm mb-1">
          <span className="text-gray-600">已使用 {usageCount} 次</span>
          <span className="text-gray-600">剩余 {remaining} 次</span>
        </div>
        <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
          <div
            className={`h-full bg-gradient-to-r ${
              percentage > 80 
                ? 'from-red-500 to-orange-500'
                : percentage > 50
                ? 'from-yellow-500 to-orange-500'
                : 'from-green-500 to-emerald-500'
            } transition-all duration-500`}
            style={{ width: `${Math.min(percentage, 100)}%` }}
          />
        </div>
      </div>
      
      {/* 升级提示 */}
      {tier === 'free' && percentage > 80 && (
        <div className="mt-4 p-3 bg-blue-50 rounded-lg text-sm">
          <p className="text-blue-700 font-medium">
            📊 使用次数即将用完
          </p>
          <p className="text-blue-600 mt-1">
            升级到专业版可获得 1,000 次/月
          </p>
          <button className="mt-2 px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg text-sm font-medium hover:shadow-lg transition-shadow">
            立即升级 →
          </button>
        </div>
      )}
    </div>
  )
}

export default UsageStats
