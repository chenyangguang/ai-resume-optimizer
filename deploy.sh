#!/bin/bash

# AI 简历优化器 - 一键部署脚本

echo "🚀 开始部署 AI 简历优化器..."

# 1. 检查服务状态
echo "1️⃣ 检查服务状态..."
if curl -s http://localhost:3002/api/health > /dev/null; then
    echo "✅ 后端服务运行中"
else
    echo "⚠️  后端服务未运行，正在启动..."
    cd ~/ai-resume-optimizer/backend
    source ~/.cargo/env
    nohup ./target/release/ai_resume_optimizer > /tmp/resume-backend.log 2>&1 &
    sleep 2
    echo "✅ 后端服务启动完成"
fi

# 2. 配置 Nginx
echo "2️⃣ 配置 Nginx..."
sudo cp ~/ai-resume-optimizer/nginx.conf /etc/nginx/sites-available/ai-resume-optimizer
sudo ln -sf /etc/nginx/sites-available/ai-resume-optimizer /etc/nginx/sites-enabled/

# 测试 Nginx 配置
if sudo nginx -t; then
    echo "✅ Nginx 配置正确"
    sudo systemctl reload nginx
    echo "✅ Nginx 已重新加载"
else
    echo "❌ Nginx 配置错误，请检查"
    exit 1
fi

# 3. 检查防火墙
echo "3️⃣ 检查防火墙..."
if command -v ufw &> /dev/null; then
    sudo ufw allow 80/tcp
    sudo ufw allow 443/tcp
    echo "✅ 防火墙已配置"
fi

# 4. 测试访问
echo "4️⃣ 测试访问..."
sleep 2

if curl -s http://localhost/api/health > /dev/null; then
    echo "✅ API 访问正常"
else
    echo "⚠️  API 访问异常，请检查"
fi

# 5. 显示访问地址
echo ""
echo "✅ 部署完成！"
echo ""
echo "📍 访问地址："
echo "   HTTP:    http://$(curl -s ifconfig.me)"
echo ""
echo "📝 日志位置："
echo "   后端: /tmp/resume-backend.log"
echo "   Nginx: /var/log/nginx/access.log"
echo ""
echo "🔧 管理命令："
echo "   查看后端日志: tail -f /tmp/resume-backend.log"
echo "   重启后端: pkill -f ai_resume_optimizer && cd ~/ai-resume-optimizer/backend && ./target/release/ai_resume_optimizer &"
echo "   重启 Nginx: sudo systemctl restart nginx"
