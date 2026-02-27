#!/bin/bash

# 从主服务器自动同步 SSL 证书
# 由 cron 定时任务调用（每周执行一次）

set -e

# ===== 配置 =====
MAIN_SERVER="gitvim.com"  # 主服务器域名或 IP
REMOTE_CERT_PATH="/etc/letsencrypt/live/gitvim.com"
LOCAL_CERT_PATH="/etc/nginx/ssl/gitvim"

LOG_FILE="/var/log/ssl-sync.log"
# ================

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

log "🚀 开始同步证书..."

# 创建本地目录
mkdir -p "$LOCAL_CERT_PATH"

# 从主服务器复制证书
if scp "$MAIN_SERVER:$REMOTE_CERT_PATH/fullchain.pem" "$LOCAL_CERT_PATH/cert.pem" 2>>"$LOG_FILE"; then
    log "✅ cert.pem 同步成功"
else
    # 如果 fullchain.pem 不存在，尝试 cert.pem
    if scp "$MAIN_SERVER:$REMOTE_CERT_PATH/cert.pem" "$LOCAL_CERT_PATH/cert.pem" 2>>"$LOG_FILE"; then
        log "✅ cert.pem 同步成功"
    else
        log "❌ cert.pem 同步失败"
        exit 1
    fi
fi

if scp "$MAIN_SERVER:$REMOTE_CERT_PATH/privkey.pem" "$LOCAL_CERT_PATH/key.pem" 2>>"$LOG_FILE"; then
    log "✅ key.pem 同步成功"
else
    log "❌ key.pem 同步失败"
    exit 1
fi

# 设置权限
chmod 644 "$LOCAL_CERT_PATH/cert.pem"
chmod 600 "$LOCAL_CERT_PATH/key.pem"

log "✅ 权限设置完成"

# 检查证书有效期
EXPIRY=$(openssl x509 -in "$LOCAL_CERT_PATH/cert.pem" -noout -enddate | cut -d= -f2)
log "📅 证书有效期至: $EXPIRY"

# 重载 Nginx
if nginx -t 2>>"$LOG_FILE"; then
    systemctl reload nginx
    log "✅ Nginx 重载成功"
else
    log "❌ Nginx 配置测试失败"
    exit 1
fi

log "🎉 证书同步完成！"
log ""
