# Gitvim.com SSL 证书自动分发配置指南

## 架构说明

```
gitvim.com 主服务器
    ↓ (Let's Encrypt 自动续期)
    ↓ (续期成功触发 deploy hook)
    ↓ (自动推送到从服务器)
    ├→ resume.gitvim.com (8.130.134.196)
    ├→ writing.gitvim.com
    ├→ 其他服务器...
    └→ 服务器 N
```

---

## 第一步：在主服务器上配置

### 1. 编辑从服务器列表

```bash
nano /root/ai-resume-optimizer/distribute-certs-to-slaves.sh
```

**修改这部分：**
```bash
SLAVE_SERVERS=(
    "8.130.134.196"           # resume.gitvim.com
    "服务器2的IP"             # writing.gitvim.com
    "服务器3的IP"
    "服务器4的IP"
    # 添加更多服务器...
)
```

### 2. 测试分发脚本

```bash
# 手动测试
/root/ai-resume-optimizer/distribute-certs-to-slaves.sh

# 查看日志
tail -f /var/log/ssl-distribute.log
```

### 3. 配置自动分发

**创建 deploy hook：**
```bash
sudo mkdir -p /etc/letsencrypt/renewal-hooks/deploy/
sudo nano /etc/letsencrypt/renewal-hooks/deploy/distribute-certs.sh
```

**内容：**
```bash
#!/bin/bash
# 证书更新后自动分发
/root/ai-resume-optimizer/distribute-certs-to-slaves.sh
```

**设置权限：**
```bash
sudo chmod +x /etc/letsencrypt/renewal-hooks/deploy/distribute-certs.sh
```

### 4. 测试自动续期

```bash
# 模拟续期（不会真的更新证书）
sudo certbot renew --dry-run

# 查看日志
tail -f /var/log/letsencrypt-deploy.log
```

---

## 第二步：在从服务器上配置

### 1. 创建证书目录

```bash
sudo mkdir -p /etc/nginx/ssl/gitvim
sudo chmod 755 /etc/nginx/ssl/gitvim
```

### 2. 配置 Nginx

**编辑配置：**
```bash
sudo nano /etc/nginx/sites-available/resume.gitvim.com
```

**SSL 配置：**
```nginx
ssl_certificate /etc/nginx/ssl/gitvim/cert.pem;
ssl_certificate_key /etc/nginx/ssl/gitvim/key.pem;
```

### 3. 测试配置

```bash
sudo nginx -t
sudo systemctl reload nginx
```

---

## 第三步：配置 SSH 免密登录

**在主服务器上运行：**

```bash
# 生成 SSH 密钥（如果没有）
ssh-keygen -t rsa -b 4096

# 复制公钥到从服务器
ssh-copy-id root@8.130.134.196
ssh-copy-id root@服务器2的IP
ssh-copy-id root@服务器3的IP
# ... 更多服务器

# 测试免密登录
ssh root@8.130.134.196 "hostname"
```

---

## 工作流程

### 证书自动续期流程

```
1. Let's Encrypt 检测证书即将过期
   ↓
2. 主服务器自动运行 certbot renew
   ↓
3. 证书更新成功
   ↓
4. 触发 deploy hook: distribute-certs.sh
   ↓
5. 执行分发脚本: distribute-certs-to-slaves.sh
   ↓
6. 遍历所有从服务器：
   - 推送 cert.pem
   - 推送 key.pem
   - 设置权限
   - 重载 Nginx
   ↓
7. 所有服务器证书同步完成
```

### 手动触发分发

```bash
# 在主服务器上运行
/root/ai-resume-optimizer/distribute-certs-to-slaves.sh

# 查看日志
tail -f /var/log/ssl-distribute.log
```

---

## 日志文件

| 日志 | 路径 | 说明 |
|------|------|------|
| 分发日志 | /var/log/ssl-distribute.log | 证书分发详细日志 |
| Deploy Hook | /var/log/letsencrypt-deploy.log | Let's Encrypt 触发日志 |
| Certbot 日志 | /var/log/letsencrypt/letsencrypt.log | 证书续期日志 |

---

## 监控和告警

### 检查证书有效期

```bash
# 在从服务器上运行
openssl x509 -in /etc/nginx/ssl/gitvim/cert.pem -noout -dates

# 输出示例
notBefore=Feb 15 23:26:24 2026 GMT
notAfter=May 16 23:26:23 2026 GMT
```

### 添加监控脚本

```bash
# 在从服务器上创建
nano /root/check-cert.sh
```

```bash
#!/bin/bash
# 检查证书有效期，如果少于 30 天则告警

EXPIRY=$(openssl x509 -in /etc/nginx/ssl/gitvim/cert.pem -noout -enddate | cut -d= -f2)
EXPIRY_EPOCH=$(date -d "$EXPIRY" +%s)
NOW_EPOCH=$(date +%s)
DAYS_LEFT=$(( ($EXPIRY_EPOCH - $NOW_EPOCH) / 86400 ))

if [ $DAYS_LEFT -lt 30 ]; then
    echo "⚠️ 证书即将过期！剩余 $DAYS_LEFT 天"
    # 发送告警（邮件、飞书等）
    exit 1
fi
```

---

## 故障排查

### 问题 1：SSH 连接失败

```bash
# 检查 SSH 服务
ssh root@从服务器IP "hostname"

# 如果失败，检查：
# 1. SSH 服务是否运行
# 2. 防火墙是否开放 22 端口
# 3. 公钥是否正确复制
```

### 问题 2：Nginx 重载失败

```bash
# 在从服务器上检查配置
sudo nginx -t

# 查看错误日志
sudo tail -f /var/log/nginx/error.log
```

### 问题 3：证书分发失败

```bash
# 在主服务器上查看日志
tail -f /var/log/ssl-distribute.log

# 手动测试
/root/ai-resume-optimizer/distribute-certs-to-slaves.sh
```

---

## 多服务器管理

### 添加新服务器

1. **编辑脚本：**
   ```bash
   nano /root/ai-resume-optimizer/distribute-certs-to-slaves.sh
   ```

2. **添加 IP：**
   ```bash
   SLAVE_SERVERS=(
       "8.130.134.196"
       "新服务器IP"  # 新增
   )
   ```

3. **配置免密登录：**
   ```bash
   ssh-copy-id root@新服务器IP
   ```

4. **测试分发：**
   ```bash
   /root/ai-resume-optimizer/distribute-certs-to-slaves.sh
   ```

### 移除服务器

1. **编辑脚本，注释或删除对应 IP**
2. **无需其他操作**

---

## 安全建议

1. **SSH 密钥安全**
   - 使用强密码保护私钥
   - 定期轮换密钥

2. **证书权限**
   - cert.pem: 644
   - key.pem: 600

3. **日志审计**
   - 定期检查分发日志
   - 监控异常访问

4. **网络安全**
   - 使用防火墙限制 SSH 访问
   - 仅允许主服务器连接

---

## 测试清单

- [ ] 主服务器 SSH 免密登录所有从服务器
- [ ] 分发脚本手动执行成功
- [ ] 从服务器 Nginx 配置正确
- [ ] Deploy hook 已创建并执行
- [ ] Certbot dry-run 测试通过
- [ ] 日志正常记录

---

_配置完成后，证书将自动在所有服务器间同步！_
