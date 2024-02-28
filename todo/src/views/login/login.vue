<template>
  <!-- 根元素，其id通过buildView函数和组件名动态生成 -->
  <div :id="buildView(component)">
    <!-- 使用Element UI的Tabs组件实现标签页，其中tab位置设置为左侧 -->
    <el-tabs tab-position="left" style="height: 100%" v-model="tabActive">
      <!-- 登录标签页 -->
      <el-tab-pane label="Signin" name="signin">
        <!-- 登录组件，使用自定义事件监听切换到注册视图 -->
        <Signin @to-signup="toSignup"></Signin>
      </el-tab-pane>
      <!-- 注册标签页 -->
      <el-tab-pane label="Signup" name="signup">
        <!-- 注册组件，使用自定义事件监听切换到登录视图 -->
        <Signup @to-signin="toSignin"></Signup>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script lang="ts" setup>
// 导入所需的方法和组件
import {
  AvatarMap,
  Avatars,
  TeamAvatars,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../../core"; // 假设这些是核心功能或工具方法
import { ref, reactive } from "vue"; // 导入Vue的响应式API
import Signin from "./components/signin.vue"; // 导入登录组件
import Signup from "./components/signup.vue"; // 导入注册组件

const component = "Login"; // 定义当前组件名，用于动态生成id等

const tabActive = ref("signin"); // 使用ref创建响应式数据，控制当前激活的标签页，默认为登录

// 定义切换到注册视图的方法
const toSignup = () => {
  tabActive.value = "signup";
};

// 定义切换到登录视图的方法
const toSignin = () => {
  tabActive.value = "signin";
};
</script>

<style lang="scss" scoped>
@import "../../styles/views/login.scss"; // 导入专门的样式文件
</style>
