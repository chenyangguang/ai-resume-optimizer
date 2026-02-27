#!/bin/bash

# 在 gitvim.com 主服务器上运行
# 证书更新后，自动推送到所有从服务器

set -e

# ===== 配置 =====
CERT_PATH="/etc/letsencrypt/live/gitvim.com"
LOG_FILE="/var/log/ssl-distribute.log"

# 从服务器列表（IP 或域名）
SLAVE_SERVERS=(
    "8.130.134.196"           # resume.gitvim.com
    # "服务器2的IP"           # writing.gitvim.com
    # "服务器3的IP"
    # "服务器4的IP"
    # 添加更多服务器...
)

# SSH 用户
SSH_USER="root"

# 从服务器上的证书存放路径
REMOTE_CERT_PATH="/etc/nginx/ssl/gitvim"
# ================

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

log "🚀 开始分发证书到 ${#SLAVE_SERVERS[@]} 台服务器..."
log ""

# 检查本地证书
if [ ! -f "$CERT_PATH/fullchain.pem" ] || [ ! -f "$CERT_PATH/privkey.pem" ]; then
    log "❌ 本地证书文件不存在"
    exit 1
fi

# 遍历所有从服务器
for SERVER in "${SLAVE_SERVERS[@]}"; do
    log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    log "📤 正在推送到: $SERVER"

    # 创建远程目录
    ssh "$SSH_USER@$SERVER" "mkdir -p $REMOTE_CERT_PATH" 2>>"$LOG_FILE"

    # 推送证书
    if scp "$CERT_PATH/fullchain.pem" "$SSH_USER@$SERVER:$REMOTE_CERT_PATH/cert.pem" 2>>"$LOG_FILE"; then
        log "✅ cert.pem 推送成功"
    else
        log "❌ cert.pem 推送失败"
        continue
    fi

    if scp "$CERT_PATH/privkey.pem" "$SSH_USER@$SERVER:$REMOTE_CERT_PATH/key.pem" 2>>"$LOG_FILE"; then
        log "✅ key.pem 推送成功"
    else
        log "❌ key.pem 推送失败"
        continue
    fi

    # 设置权限
    ssh "$SSH_USER@$SERVER" "chmod 644 $REMOTE_CERT_PATH/cert.pem && chmod 600 $REMOTE_CERT_PATH/key.pem" 2>>"$LOG_FILE"
    log "✅ 权限设置完成"

    # 测试并重载 Nginx
    if ssh "$SSH_USER@$SERVER" "nginx -t" 2>>"$LOG_FILE"; then
        ssh "$SSH_USER@$SERVER" "systemctl reload nginx" 2>>"$LOG_FILE"
        log "✅ Nginx 重载成功"
    else
        log "❌ Nginx 配置测试失败"
        continue
    fi

    log "✅ $SERVER 完成"
done

log ""
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "🎉 所有服务器证书分发完成！"
log ""
