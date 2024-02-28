<template>
  <!-- Header组件，作为页面顶部导航 -->
  <Header></Header>
  <!-- 根据用户登录状态显示主内容区域或登录界面 -->
  <div class="app-main-wrapper" v-if="userStore.isSignIn">
    <!-- 左侧菜单组件 -->
    <div class="main-wrapper-menu"><Menu></Menu></div>
    <!-- 主视图区，根据路由动态展示内容 -->
    <router-view></router-view>
  </div>
  <!-- 未登录时显示的登录组件 -->
  <div v-else class="app-main-wrapper">
    <Login></Login>
  </div>
</template>

<script setup lang="ts">
// 引入Vue组件和Pinia状态管理
import Header from "./components/header/header.vue";
import Menu from "./components/menu/menu.vue";
import { user as userPinia } from "./store/src/user";
import { init } from "./core";
import { onMounted } from "vue";
import Login from "./views/login/login.vue";
import api from "./api";

// 使用Pinia创建用户状态的实例
const userStore = userPinia();

// 检查并设置用户的登录状态
userStore.checkSetIsSignIn();

// 应用初始化函数，用于加载用户信息
const initApp = async () => {
  let username = userStore.getUsername();
  if (typeof username !== "undefined") {
    // 请求用户信息
    const data = await api.user.getUserInfo(username.toString());

    if (typeof data !== "undefined") {
      // 设置用户信息和登录状态
      userStore.setUser(data);
      userStore.setSignIn();
    }
  }
};

// 组件挂载时调用初始化函数
onMounted(() => {
  initApp();
});
</script>

<style lang="scss" scoped>
/* 主内容区域的样式 */
.app-main-wrapper {
  width: 100%;
  height: calc(100vh - 60px); /* 高度为视窗高度减去头部高度 */
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  //元素不换行
  flex-wrap: nowrap;
  //超出隐藏
  overflow: hidden;
  .main-wrapper-menu {
    box-sizing: border-box;
    width: 60px;
    height: 100%;
    border-right: 1px solid #ff9c4b; /* 菜单右侧边框 */
  }
}
</style>
