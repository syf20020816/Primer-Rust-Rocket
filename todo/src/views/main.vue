<template>
  <div :id="buildView(component)">
    <!-- 构建描述部分的布局 -->
    <div :class="buildWrap(component, 'desc')">
      <h4>
        <!-- 描述文本，告知用户该页面的功能 -->
        <span style="margin: 0 16px">
          You can view and handle all high level and emergent level todos on this page
        </span>
        <!-- 切换开关，用于在关注的待办事项和紧急的待办事项之间切换 -->
        <el-switch v-model="todoType" active-text="Focus" inactive-text="Fatal" />
      </h4>
    </div>
    <!-- 构建详情部分的布局，展示待办事项时间线 -->
    <div :class="buildWrap(component, 'detail')">
      <!-- Timeline 组件用于展示待办事项，:datas 绑定待办事项数据 -->
      <Timeline :datas="todos"></Timeline>
    </div>
  </div>
</template>

<script lang="ts" setup>
// 引入 Vue 相关函数
import { reactive, computed, ref } from "vue";
// 引入相关的函数和组件
import { AvatarMap, TeamAvatars, build, buildView, buildWrap, useTeam } from "../core";
import { Timeline } from "./plan";
import { user as userPinia } from "../store/src/user"; // 从 Pinia 引入 user store

const component = "Main"; // 组件名称
const userStore = userPinia(); // 使用 Pinia 管理的用户状态
const todoType = ref(true); // 定义待办事项类型的响应式引用，true 表示关注的待办事项，false 表示紧急的待办事项

// 计算属性，根据 todoType 的值动态返回对应的待办事项列表
const todos = computed(() => {
  let { focus, fatal } = userStore.user.todos; // 从用户状态中解构出关注和紧急的待办事项

  // 根据 todoType 的值返回对应的待办事项列表
  if (todoType.value) {
    return focus; // 如果 todoType 为 true，返回关注的待办事项
  }
  return fatal; // 否则返回紧急的待办事项
});
</script>


<style lang="scss" scoped>
@import "../styles/views/main.scss";
</style>
