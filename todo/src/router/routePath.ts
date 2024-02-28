/**
 * ============================================
 * @author:syf20020816@outlook.com
 * @since:20230223
 * @version:0.2.0
 * @type:ts
 * @description:vue-router设置页面路由地址
 * ============================================
 */

// 用户信息页面
import User from '../views/user.vue'
// TODO计划页面
import Plan from '../views/plan/plan.vue'
// 重要TODO页面
import Main from '../views/main.vue'
// 历史TODO页面
import History from '../views/history.vue'
// 团队协同页面
import Collaborate from '../views/collaborate/collaborate.vue'
import { RouteRecordRaw } from 'vue-router'
export const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    component: User
  },
  {
    path: '/plan',
    component: Plan
  },
  {
    path: '/main',
    component: Main
  },
  {
    path: '/history',
    component: History
  },
  {
    path: '/collaborate',
    component: Collaborate
  }
]
