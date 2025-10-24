#!/bin/bash

set -e

echo "🚀 开始部署 Rust FullStack 应用..."

# 检查 Docker 是否安装
if ! command -v docker &> /dev/null; then
    echo "❌ Docker 未安装，请先安装 Docker"
    exit 1
fi

# 检查 Docker Compose 是否安装
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose 未安装，请先安装 Docker Compose"
    exit 1
fi

# 创建环境变量文件
if [ ! -f .env ]; then
    echo "📝 创建 .env 文件..."
    cp .env.example .env
    echo "⚠️  请编辑 .env 文件配置您的环境变量"
fi

# 构建和启动服务
echo "🔨 构建 Docker 镜像..."
docker-compose build

echo "🚀 启动服务..."
docker-compose up -d

echo "⏳ 等待服务启动..."
sleep 30

# 检查服务状态
echo "🔍 检查服务状态..."
docker-compose ps

# 显示访问信息
echo ""
echo "🎉 部署完成!"
echo ""
echo "📊 服务访问地址:"
echo "  前端: http://localhost"
echo "  后端 API: http://localhost:8080"
echo "  API 文档: http://localhost:8080/swagger-ui"
echo ""
echo "🛠️  管理命令:"
echo "  查看日志: docker-compose logs -f"
echo "  停止服务: docker-compose down"
echo "  重启服务: docker-compose restart"
echo "  更新服务: docker-compose pull && docker-compose up -d"
echo ""
echo "⚠️  首次启动请等待数据库初始化完成"