#!/bin/bash

# 在 gitvim 服务器上配置：证书续期后自动同步到简历优化器服务器

RESUME_SERVER="root@8.130.134.196"
LOCAL_CERT="/etc/letsencrypt/live/gitvim.com/fullchain.pem"
LOCAL_KEY="/etc/letsencrypt/live/gitvim.com/privkey.pem"
REMOTE_PATH="/root/ssl/gitvim.com"

echo "🔧 配置证书自动同步..."

# 测试 SSH 连接
echo "🔐 测试 SSH 连接..."
ssh $RESUME_SERVER "echo '✅ SSH 连接成功'" || {
    echo "❌ SSH 连接失败，请先配置免密登录："
    echo "   ssh-copy-id $RESUME_SERVER"
    exit 1
}

# 创建 deploy hook 脚本
cat > /etc/letsencrypt/renewal-hooks/deploy/sync-to-resume.sh <<'EOF'
#!/bin/bash
# Let's Encrypt 续期后自动同步证书

RESUME_SERVER="root@8.130.134.196"
CERT_DIR="/etc/letsencrypt/live/gitvim.com"

if [ -d "$CERT_DIR" ]; then
    echo "🔄 同步证书到简历优化器服务器..."
    ssh $RESUME_SERVER "mkdir -p /root/ssl/gitvim.com"
    scp $CERT_DIR/fullchain.pem $RESUME_SERVER:/root/ssl/gitvim.com/
    scp $CERT_DIR/privkey.pem $RESUME_SERVER:/root/ssl/gitvim.com/
    ssh $RESUME_SERVER "systemctl reload nginx"
    echo "✅ 证书同步完成"
fi
EOF

chmod +x /etc/letsencrypt/renewal-hooks/deploy/sync-to-resume.sh

echo "✅ 自动同步配置完成"
echo ""
echo "📋 说明："
echo "   - 每次 Let's Encrypt 续期后，会自动同步证书到简历优化器服务器"
echo "   - 同步后会自动重载 Nginx"
echo ""
echo "🧪 测试："
echo "   sudo certbot renew --dry-run"
