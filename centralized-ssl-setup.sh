#!/bin/bash

# 集中式证书管理：一张通配符证书服务所有服务器

# ========== 配置区 ==========
DOMAIN="gitvim.com"
EMAIL="your-email@example.com"

# 所有需要证书的服务器列表
SERVERS=(
    "root@8.130.134.196"      # 简历优化器
    # "root@服务器2的IP"       # 其他服务
    # "root@服务器3的IP"
    # 添加更多...
)

# 本地证书路径
CERT_DIR="/etc/letsencrypt/live/$DOMAIN"
DEPLOY_HOOK="/etc/letsencrypt/renewal-hooks/deploy/distribute-certs.sh"

# ========== 主程序 ==========

echo "🔐 集中式证书管理配置"
echo ""

# 检查是否已安装 certbot
if ! command -v certbot &> /dev/null; then
    echo "📦 安装 Certbot..."
    sudo apt update && sudo apt install -y certbot
fi

# 检查是否已有证书
if [ -d "$CERT_DIR" ]; then
    echo "✅ 发现已存在的证书"
    read -p "是否重新申请？(y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "使用现有证书"
    else
        REQUEST_NEW=true
    fi
else
    REQUEST_NEW=true
fi

# 申请证书
if [ "$REQUEST_NEW" = true ]; then
    echo ""
    echo "📝 开始申请通配符证书..."
    echo ""
    echo "⚠️  需要添加 DNS TXT 记录验证："
    echo "   _acme-challenge.$DOMAIN TXT <验证码>"
    echo ""

    sudo certbot certonly --manual --preferred-challenges dns \
        -d "*.$DOMAIN" -d "$DOMAIN" \
        --email "$EMAIL" --agree-tos --no-eff-email

    if [ $? -ne 0 ]; then
        echo "❌ 证书申请失败"
        exit 1
    fi
fi

# 创建自动分发脚本
echo ""
echo "🔧 配置自动分发..."

cat > "$DEPLOY_HOOK" <<EOF
#!/bin/bash
# Let's Encrypt 续期后自动分发到所有服务器

CERT_DIR="/etc/letsencrypt/live/$DOMAIN"

echo "🔄 证书续期成功，开始分发到所有服务器..."
echo ""

# 分发到所有服务器
for SERVER in ${SERVERS[@]}; do
    echo "📤 分发到: \$SERVER"

    # 创建远程目录
    ssh \$SERVER "mkdir -p /root/ssl/$DOMAIN"

    # 复制证书
    scp \$CERT_DIR/fullchain.pem \$SERVER:/root/ssl/$DOMAIN/
    scp \$CERT_DIR/privkey.pem \$SERVER:/root/ssl/$DOMAIN/

    # 重载 Nginx
    ssh \$SERVER "systemctl reload nginx 2>/dev/null || systemctl reload httpd 2>/dev/null || true"

    echo "✅ \$SERVER 完成"
    echo ""
done

echo "🎉 所有服务器证书更新完成！"
EOF

chmod +x "$DEPLOY_HOOK"

echo "✅ 自动分发配置完成"
echo ""

# 配置 SSH 免密登录
echo "🔐 检查 SSH 连接..."
echo ""

ALL_CONNECTED=true
for SERVER in "${SERVERS[@]}"; do
    echo "测试: $SERVER"
    if ssh -o ConnectTimeout=5 -o BatchMode=yes $SERVER "echo '✅ 连接成功'" 2>/dev/null; then
        :
    else
        echo "❌ $SERVER 连接失败"
        echo "   请配置免密登录: ssh-copy-id $SERVER"
        ALL_CONNECTED=false
    fi
    echo ""
done

if [ "$ALL_CONNECTED" = false ]; then
    echo "⚠️  部分服务器连接失败，请先配置 SSH 免密登录"
    echo "   然后重新运行此脚本"
    exit 1
fi

# 手动分发一次
read -p "是否立即分发证书到所有服务器？(y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "📤 开始分发证书..."
    bash "$DEPLOY_HOOK"
fi

# 配置自动续期
echo ""
echo "⏰ 配置自动续期..."
(sudo crontab -l 2>/dev/null | grep -v "certbot renew"; echo "0 3 * * * certbot renew --quiet") | sudo crontab -

echo "✅ 自动续期已配置（每天凌晨 3 点）"
echo ""

echo "🎉 配置完成！"
echo ""
echo "📋 说明："
echo "   - 通配符证书: *.$DOMAIN"
echo "   - 管理服务器数: ${#SERVERS[@]}"
echo "   - 自动续期: 每天凌晨 3 点"
echo "   - 自动分发: 续期后自动同步到所有服务器"
echo ""
echo "🧪 测试："
echo "   sudo certbot renew --dry-run"
