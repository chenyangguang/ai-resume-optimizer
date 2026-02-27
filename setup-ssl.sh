#!/bin/bash

# 等待证书上传完成后的配置脚本

SSL_DIR="/root/ssl"
CRT_FILE="$SSL_DIR/gitvim.com.crt"
KEY_FILE="$SSL_DIR/gitvim.com.key"

echo "🔍 检查证书文件..."

if [ ! -f "$CRT_FILE" ]; then
    echo "❌ 证书文件不存在: $CRT_FILE"
    echo ""
    echo "📝 请上传证书到以下位置："
    echo "   证书: $CRT_FILE"
    echo "   私钥: $KEY_FILE"
    echo ""
    echo "💡 上传命令（在另一台服务器执行）："
    echo "   scp /path/to/gitvim.com.crt root@8.130.134.196:/root/ssl/"
    echo "   scp /path/to/gitvim.com.key root@8.130.134.196:/root/ssl/"
    echo ""
    echo "上传完成后，运行: ~/ai-resume-optimizer/setup-ssl.sh"
    exit 1
fi

if [ ! -f "$KEY_FILE" ]; then
    echo "❌ 私钥文件不存在: $KEY_FILE"
    exit 1
fi

echo "✅ 证书文件已找到"

# 更新 Nginx 配置中的证书路径
echo "📝 更新 Nginx 配置..."
sed -i "s|/path/to/gitvim.com.crt|$CRT_FILE|g" ~/ai-resume-optimizer/nginx-https.conf
sed -i "s|/path/to/gitvim.com.key|$KEY_FILE|g" ~/ai-resume-optimizer/nginx-https.conf

# 部署 Nginx 配置
echo "🔧 部署 Nginx 配置..."
sudo cp ~/ai-resume-optimizer/nginx-https.conf /etc/nginx/sites-available/ai-resume-optimizer-https
sudo ln -sf /etc/nginx/sites-available/ai-resume-optimizer-https /etc/nginx/sites-enabled/

# 测试 Nginx 配置
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
echo "⏳ 等待 5 秒后测试..."
sleep 5

if curl -s https://resume.gitvim.com/api/health > /dev/null; then
    echo "✅ HTTPS 访问成功！"
else
    echo "⚠️  HTTPS 访问测试失败，请检查 DNS 是否已解析"
fi

echo ""
echo "🎉 配置完成！"
echo ""
echo "📍 访问地址："
echo "   HTTP:  http://resume.gitvim.com"
echo "   HTTPS: https://resume.gitvim.com"
