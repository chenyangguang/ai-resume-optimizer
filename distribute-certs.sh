#!/bin/bash
# Let's Encrypt 续期后自动分发证书到所有服务器

# ========== 配置区 ==========
DOMAIN="gitvim.com"
CERT_DIR="/etc/letsencrypt/live/$DOMAIN"

# 所有需要证书的服务器列表
SERVERS=(
    "root@8.130.134.196"      # 简历优化器
    # "root@服务器2的IP"       # 其他服务
    # "root@服务器3的IP"
    # 添加更多...
)

# ========== 主程序 ==========

# 只在证书目录存在时执行
if [ ! -d "$CERT_DIR" ]; then
    echo "❌ 证书目录不存在: $CERT_DIR"
    exit 1
fi

echo "🔄 证书续期成功，开始分发到所有服务器..."
echo ""

# 分发到所有服务器
for SERVER in ${SERVERS[@]}; do
    echo "📤 分发到: $SERVER"

    # 1. 创建远程目录
    ssh $SERVER "mkdir -p /root/ssl/$DOMAIN"

    # 2. 复制证书文件
    scp $CERT_DIR/fullchain.pem $SERVER:/root/ssl/$DOMAIN/
    scp $CERT_DIR/privkey.pem $SERVER:/root/ssl/$DOMAIN/

    # 3. 重载 Nginx
    ssh $SERVER "systemctl reload nginx 2>/dev/null || systemctl reload httpd 2>/dev/null || true"

    echo "✅ $SERVER 完成"
    echo ""
done

echo "🎉 所有服务器证书更新完成！"
echo ""
echo "📋 已更新的服务器："
for SERVER in ${SERVERS[@]}; do
    echo "   - $SERVER"
done
