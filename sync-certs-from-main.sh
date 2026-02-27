#!/bin/bash

# 从 gitvim.com 主服务器同步证书到当前服务器
# 使用方法：./sync-certs.sh

set -e

# ===== 配置部分 - 请修改 =====
GITVIM_SERVER_IP="主服务器IP"  # 改成 gitvim.com 服务器的 IP
GITVIM_SERVER_USER="root"       # SSH 用户名
GITVIM_DOMAIN="gitvim.com"      # 域名
# =============================

REMOTE_CERT_PATH="/etc/letsencrypt/live/$GITVIM_DOMAIN"
LOCAL_CERT_PATH="/etc/letsencrypt/live/$GITVIM_DOMAIN"

echo "🚀 从 $GITVIM_SERVER_IP 同步证书..."
echo ""

# 创建本地目录
sudo mkdir -p "$LOCAL_CERT_PATH"

# 使用 scp 同步证书
echo "📥 正在复制证书文件..."
scp "$GITVIM_SERVER_USER@$GITVIM_SERVER_IP:$REMOTE_CERT_PATH/fullchain.pem" /tmp/fullchain.pem
scp "$GITVIM_SERVER_USER@$GITVIM_SERVER_IP:$REMOTE_CERT_PATH/privkey.pem" /tmp/privkey.pem
scp "$GITVIM_SERVER_USER@$GITVIM_SERVER_IP:$REMOTE_CERT_PATH/chain.pem" /tmp/chain.pem 2>/dev/null || true
scp "$GITVIM_SERVER_USER@$GITVIM_SERVER_IP:$REMOTE_CERT_PATH/cert.pem" /tmp/cert.pem 2>/dev/null || true

# 移动到正确位置
sudo mv /tmp/fullchain.pem "$LOCAL_CERT_PATH/"
sudo mv /tmp/privkey.pem "$LOCAL_CERT_PATH/"
sudo mv /tmp/chain.pem "$LOCAL_CERT_PATH/" 2>/dev/null || true
sudo mv /tmp/cert.pem "$LOCAL_CERT_PATH/" 2>/dev/null || true

# 设置权限
sudo chmod 644 "$LOCAL_CERT_PATH/fullchain.pem"
sudo chmod 600 "$LOCAL_CERT_PATH/privkey.pem"
sudo chown -R root:root "$LOCAL_CERT_PATH"

echo "✅ 证书同步成功！"
echo ""

# 测试 Nginx 配置
echo "🔄 测试 Nginx 配置..."
if sudo nginx -t; then
    echo "✅ Nginx 配置正常"
    echo ""
    echo "📍 现在可以启用 HTTPS 配置了"
    echo "   运行: sudo nano /etc/nginx/sites-available/resume.gitvim.com"
    echo "   取消注释 HTTPS 部分"
else
    echo "❌ Nginx 配置有问题"
    exit 1
fi
