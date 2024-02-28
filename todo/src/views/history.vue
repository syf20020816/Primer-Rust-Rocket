<template>
  <div :id="buildView(component)">
    <!-- 构建左侧布局，用于显示日历 -->
    <div :class="buildWrap(component, 'left')">
      <el-calendar>
        <!-- 自定义日历单元格，用于显示日历中每一天的数据 -->
        <template #date-cell="{ data }">
          <!-- 点击日历单元格时触发 getTODO 函数 -->
          <div @click="getTODO(data)" style="height: 100%; width: 100%">
            <!-- 显示日期，如果被选中则使用特定样式和标记 -->
            <p :class="data.isSelected ? 'is-selected' : ''">
              {{ data.day.split("-").slice(1).join("-") }}
              {{ data.isSelected ? "🚩" : "" }}
            </p>
          </div>
        </template>
      </el-calendar>
    </div>
    <!-- 构建中间布局，用于显示时间轴 -->
    <div :class="buildWrap(component, 'mid')">
      <!-- 如果有待办事项则显示时间轴，否则显示无事项提示 -->
      <el-timeline v-if="todos?.length">
        <!-- 遍历 todos 显示每一个待办事项 -->
        <el-timeline-item
          v-for="(item, index) in todos"
          :key="index"
          :timestamp="`${item.date.start}~${item.date.end}`"
        >
          <!-- 显示待办事项名称 -->
          <h4>{{ item.name }}</h4>
          <!-- 点击查看详情按钮，设置当前待办事项为点击的待办事项 -->
          <el-button type="primary" @click="currentItem = item">📃Check</el-button>
        </el-timeline-item>
      </el-timeline>
      <!-- 如果没有待办事项则显示提示信息 -->
      <div v-else>
        <h4>There are no history TODOs for the current date</h4>
      </div>
    </div>
    <!-- 构建右侧布局，用于显示待办事项详情 -->
    <div :class="buildWrap(component, 'right')">
      <!-- TODOItem 组件，显示当前选中的待办事项详情 -->
      <TODOItem
        :current-todo="currentItem"
        :is-change="false"
        :is-compelete="false"
      ></TODOItem>
    </div>
  </div>
</template>

<script lang="ts" setup>
// 引入所需的 Vue 函数和其他相关的功能
import { ref, reactive } from "vue";
import {
  AvatarMap,
  Avatars,
  Todo,
  Priorities,
  Status,
  TeamAvatars,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../core";
import { TODOItem } from "./plan";
import { user } from "../store/src/user";

// 定义组件名称
const component = "History";
// 使用用户存储
const userStore = user();

// 定义当前选中的待办事项
const currentItem = ref<Todo>();

// 定义待办事项列表
const todos = ref<Todo[]>();
// 根据日期筛选待办事项的函数
const filterTodos = (date: Date) => {
  let time = new Date(date.toLocaleDateString()).getTime();
  todos.value = userStore.user.todos.history.filter((todo) => {
    let end = new Date(new Date(todo.date.end).toLocaleDateString()).getTime();
    let start = new Date(new Date(todo.date.start).toLocaleDateString()).getTime();

    // 如果待办事项的开始时间和结束时间包含该日期，则筛选出该待办事项
    if (start <= time && end >= time) {
      return true;
    }
    return false;
  });
};

// 获取特定日期的待办事项并更新当前待办事项
const getTODO = (data: any) => {
  let date = data.date;
  currentItem.value = undefined; // 清除当前选中的待办事项
  filterTodos(date); // 筛选待办事项
};

// 初始化时筛选当前日期的待办事项
filterTodos(new Date());
</script>


<style lang="scss" scoped>
@import "../styles/views/history.scss";
</style>
