#!/bin/bash

# 从 gitvim 服务器同步证书到当前服务器

GITVIM_SERVER="user@gitvim服务器IP"  # 改成你的 gitvim 服务器地址
REMOTE_CERT_PATH="/etc/letsencrypt/live/gitvim.com"  # gitvim 服务器上的证书路径

LOCAL_SSL_DIR="/root/ssl/gitvim.com"
LOCAL_CERT="$LOCAL_SSL_DIR/fullchain.pem"
LOCAL_KEY="$LOCAL_SSL_DIR/privkey.pem"

echo "🔄 从 gitvim 服务器同步证书..."

# 创建本地目录
mkdir -p $LOCAL_SSL_DIR

# 同步证书
echo "📥 正在复制证书..."
rsync -avz $GITVIM_SERVER:$REMOTE_CERT_PATH/fullchain.pem $LOCAL_CERT
rsync -avz $GITVIM_SERVER:$REMOTE_CERT_PATH/privkey.pem $LOCAL_KEY

if [ -f "$LOCAL_CERT" ] && [ -f "$LOCAL_KEY" ]; then
    echo "✅ 证书同步成功"

    # 更新 Nginx 配置
    cat > ~/ai-resume-optimizer/nginx-sync.conf <<EOF
server {
    listen 80;
    server_name resume.gitvim.com;
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    server_name resume.gitvim.com;

    ssl_certificate $LOCAL_CERT;
    ssl_certificate_key $LOCAL_KEY;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;

    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    location / {
        root /root/ai-resume-optimizer/frontend/dist;
        try_files \$uri \$uri/ /index.html;
    }

    location /api {
        proxy_pass http://localhost:3002;
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

    # 部署配置
    sudo cp ~/ai-resume-optimizer/nginx-sync.conf /etc/nginx/sites-available/ai-resume-https
    sudo ln -sf /etc/nginx/sites-available/ai-resume-https /etc/nginx/sites-enabled/
    sudo nginx -t && sudo systemctl reload nginx

    echo "✅ HTTPS 配置完成"
    echo ""
    echo "📍 访问: https://resume.gitvim.com"
else
    echo "❌ 证书同步失败"
    exit 1
fi
