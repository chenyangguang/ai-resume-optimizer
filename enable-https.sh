#!/bin/bash

# 配置 HTTPS（使用 Let's Encrypt 证书）

SSL_CERT="/etc/letsencrypt/live/gitvim.com/fullchain.pem"
SSL_KEY="/etc/letsencrypt/live/gitvim.com/privkey.pem"

echo "🔧 配置 HTTPS..."

# 检查证书是否存在
if [ ! -f "$SSL_CERT" ] || [ ! -f "$SSL_KEY" ]; then
    echo "❌ Let's Encrypt 证书不存在"
    echo "请先运行: ~/ai-resume-optimizer/letsencrypt-setup.sh"
    exit 1
fi

# 更新 Nginx 配置
echo "📝 更新 Nginx 配置..."

cat > ~/ai-resume-optimizer/nginx-letsencrypt.conf <<EOF
server {
    listen 80;
    server_name resume.gitvim.com;
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name resume.gitvim.com;

    # Let's Encrypt 证书
    ssl_certificate $SSL_CERT;
    ssl_certificate_key $SSL_KEY;

    # SSL 优化配置
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;

    # HSTS
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # 前端静态文件
    location / {
        root /root/ai-resume-optimizer/frontend/dist;
        try_files \$uri \$uri/ /index.html;

        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }
    }

    # 后端 API 代理
    location /api {
        proxy_pass http://localhost:3002;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }

    # 健康检查
    location /health {
        proxy_pass http://localhost:3002/api/health;
    }

    # Let's Encrypt 验证
    location /.well-known/acme-challenge/ {
        root /var/www/letsencrypt;
    }
}
EOF

# 创建 Let's Encrypt 验证目录
sudo mkdir -p /var/www/letsencrypt

# 部署配置
echo "🔧 部署 Nginx 配置..."
sudo cp ~/ai-resume-optimizer/nginx-letsencrypt.conf /etc/nginx/sites-available/ai-resume-optimizer-https
sudo ln -sf /etc/nginx/sites-available/ai-resume-optimizer-https /etc/nginx/sites-enabled/

# 测试并重载
if sudo nginx -t; then
    echo "✅ Nginx 配置正确"
    sudo systemctl reload nginx
    echo "✅ Nginx 已重新加载"
else
    echo "❌ Nginx 配置错误"
    exit 1
fi

# 测试 HTTPS
echo ""
echo "⏳ 测试 HTTPS..."
sleep 3

if curl -s https://resume.gitvim.com/api/health > /dev/null 2>&1; then
    echo "✅ HTTPS 配置成功！"
else
    echo "⚠️  HTTPS 测试失败，请检查 DNS 解析"
fi

echo ""
echo "🎉 配置完成！"
echo ""
echo "📍 访问地址："
echo "   https://resume.gitvim.com"
echo ""
echo "🔄 自动续期："
echo "   每天凌晨 3 点自动检查并续期"
echo "   查看续期任务: sudo crontab -l"
