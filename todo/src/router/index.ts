// 导入vue-router模块
import { createRouter, createWebHashHistory } from 'vue-router'
// 导入路由路径
import { routes } from './routePath'

// 创建路由
const router = createRouter({
  // 使用hash模式
  history: createWebHashHistory(),
  // 设置路由路径
  routes
})

// 导出路由
export default router
