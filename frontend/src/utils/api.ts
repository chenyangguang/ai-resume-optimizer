// 带认证的 API 请求工具

export async function authFetch(
  url: string,
  options: RequestInit = {}
): Promise<Response> {
  const token = localStorage.getItem('token')
  
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...options.headers,
  }
  
  if (token) {
    headers['Authorization'] = `Bearer ${token}`
  }
  
  const response = await fetch(url, {
    ...options,
    headers,
  })
  
  // 如果 401，清除登录状态
  if (response.status === 401) {
    localStorage.removeItem('token')
    localStorage.removeItem('user')
    window.location.href = '/'
  }
  
  return response
}

// 使用次数检查
export async function checkUsage(): Promise<{ canUse: boolean; remaining: number }> {
  const response = await authFetch('/api/resume/usage/check')
  const data = await response.json()
  return data
}

// 记录使用
export async function recordUsage(action: string): Promise<void> {
  await authFetch('/api/resume/usage/record', {
    method: 'POST',
    body: JSON.stringify({ action }),
  })
}
