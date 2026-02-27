#!/bin/bash

# Let's Encrypt 通配符证书申请和自动续期脚本

DOMAIN="gitvim.com"
EMAIL="your-email@example.com"  # 改成你的邮箱

echo "🔐 开始申请 Let's Encrypt 通配符证书..."
echo ""
echo "⚠️  注意：通配符证书需要 DNS 验证"
echo ""

# 检查是否已安装 certbot
if ! command -v certbot &> /dev/null; then
    echo "📦 安装 Certbot..."
    sudo apt update
    sudo apt install -y certbot python3-certbot-nginx
fi

echo ""
echo "📝 证书申请步骤："
echo ""
echo "1️⃣ 运行以下命令开始申请："
echo "   sudo certbot certonly --manual --preferred-challenges dns -d '*.gitvim.com' -d gitvim.com --email $EMAIL --agree-tos"
echo ""
echo "2️⃣ Certbot 会要求你添加一条 TXT 记录到 DNS："
echo "   _acme-challenge.gitvim.com TXT <验证码>"
echo ""
echo "3️⃣ 在你的 DNS 服务商添加该记录"
echo ""
echo "4️⃣ 等待 1-2 分钟后，按回车继续验证"
echo ""
echo "5️⃣ 验证成功后，证书会保存在："
echo "   /etc/letsencrypt/live/gitvim.com/fullchain.pem"
echo "   /etc/letsencrypt/live/gitvim.com/privkey.pem"
echo ""
echo "6️⃣ 配置自动续期："
echo "   sudo crontab -e"
echo "   添加：0 3 * * * certbot renew --quiet --post-hook 'systemctl reload nginx'"
echo ""

read -p "是否现在开始申请？(y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo certbot certonly --manual --preferred-challenges dns -d '*.gitvim.com' -d gitvim.com --email $EMAIL --agree-tos

    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ 证书申请成功！"
        echo ""
        echo "📋 证书位置："
        echo "   证书: /etc/letsencrypt/live/gitvim.com/fullchain.pem"
        echo "   私钥: /etc/letsencrypt/live/gitvim.com/privkey.pem"
        echo ""
        echo "🔄 配置自动续期..."

        # 添加自动续期 cron 任务
        (sudo crontab -l 2>/dev/null; echo "0 3 * * * certbot renew --quiet --post-hook 'systemctl reload nginx'") | sudo crontab -

        echo "✅ 自动续期已配置（每天凌晨 3 点检查）"
        echo ""
        echo "📝 下一步：运行 ~/ai-resume-optimizer/enable-https.sh"
    fi
fi
