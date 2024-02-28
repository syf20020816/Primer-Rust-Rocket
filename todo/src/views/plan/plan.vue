<template>
  <div :id="buildView(component)">
    <!-- 使用 el-tabs 组件创建标签页 -->
    <el-tabs class="plan-tabs">
      <!-- 创建第一个标签页，用于创建和预览待办事项 -->
      <el-tab-pane>
        <template #label>
          <!-- 自定义标签页标题，包含 SVG 图标和文本 -->
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.CREATE_PREVIEW, 16)"></div>
            <span class="title">Create and Preview</span>
          </span>
        </template>
        <!-- 标签页内容，传递用户待办事项数据到 CreatePreview 组件 -->
        <div class="c_p_wrapper">
          <CreatePreview :datas="userStore.todos"></CreatePreview>
        </div>
      </el-tab-pane>
      <!-- 创建第二个标签页，用于显示日历视图 -->
      <el-tab-pane>
        <template #label>
          <!-- 自定义标签页标题，包含 SVG 图标和文本 -->
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.CALENDER, 16)"></div>
            <span class="title">Calendar</span>
          </span>
        </template>
        <!-- 标签页内容，传递用户待办事项数据到 Calendar 组件 -->
        <div class="calendar_wrapper">
          <Calendar :datas="userStore.todos"></Calendar>
        </div>
      </el-tab-pane>
      <!-- 创建第三个标签页，用于显示时间线视图 -->
      <el-tab-pane>
        <template #label>
          <!-- 自定义标签页标题，包含 SVG 图标和文本 -->
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.TIMELINE, 16)"></div>
            <span class="title">Timeline</span>
          </span>
        </template>
        <!-- 标签页内容，传递用户待办事项数据到 Timeline 组件 -->
        <div class="timeline_wrapper">
          <Timeline :datas="userStore.todos"></Timeline>
        </div>
      </el-tab-pane>
      <!-- 创建第四个标签页，用于显示表格视图 -->
      <el-tab-pane>
        <template #label>
          <!-- 自定义标签页标题，包含 SVG 图标和文本 -->
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.TABLE, 16)"></div>
            <span class="title">Table</span>
          </span>
        </template>
        <!-- 标签页内容，传递用户待办事项数据到 Table 组件 -->
        <div class="table_wrapper">
          <Table :datas="userStore.todos"></Table>
        </div>
      </el-tab-pane>
      <!-- 创建第五个标签页，用于显示待处理的待办事项 -->
      <el-tab-pane>
        <template #label>
          <!-- 自定义标签页标题，包含 SVG 图标和文本 -->
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.PENDING, 16)"></div>
            <span class="title">Pending</span>
          </span>
        </template>
        <!-- 标签页内容，传递待处理的待办事项数据到 Pending 组件 -->
        <div class="table_wrapper">
          <Pending :datas="pendingTodos"></Pending>
        </div>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>


<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { Avatars, buildView, Priorities, Status, Todo } from "../../core";
import { SVGs, useSvg } from "../../components";
import { CreatePreview, Calendar, Timeline, Table, Pending } from "./index";
import { user as userPinia } from "../../store/src/user";
const component = "Plan";
const userStore = userPinia();

// 计算属性获取状态为阻塞的TODO
const pendingTodos = computed(() => {
  return userStore.todos.filter((todo) => todo.status === Status.PENDING);
});
</script>

<style lang="scss">
@import "../../styles/views/plan.scss";
</style>
