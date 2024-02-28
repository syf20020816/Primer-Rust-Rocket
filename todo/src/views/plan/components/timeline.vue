<template>
  <div id="timeline-view">
    <!-- 时间轴的左侧部分，显示Todo项的时间线 -->
    <div class="left">
      <!-- 使用Element Plus的时间轴组件 -->
      <el-timeline>
        <!-- 遍历传入的Todo数据，并为每个Todo创建一个时间轴项 -->
        <!-- 显示Todo的起始和结束日期 -->
        <el-timeline-item
          :timestamp="`${item.date.start}~${item.date.end}`"
          placement="top"
          v-for="(item, index) in datas"
          :key="index"
        >
          <!-- 点击Todo项时，设置当前Todo为被点击的Todo -->
          <el-card @click="currentTodo = item">
            <h4>
              <!-- 显示Todo的优先级和状态，使用计算属性来设置样式 -->
              <span class="priority" :style="getPriorityDot(item)"></span>
              <span
                class="priority"
                style="border-radius: 50%"
                :style="getStatusDot(item)"
              ></span>
              <!-- Todo的名称 -->
              <span class="collapse-title">{{ item.name }}</span>
            </h4>
            <!-- Todo的描述 -->
            <p>{{ item.description }}</p>
            <!-- 显示Todo的标签 -->
            <div class="operation-btn-wrapper">
              <el-tag
                style="margin: 0 6px"
                v-for="tag in item.tags"
                :key="tag.label"
                :type="tag.type"
                size="small"
                class="mx-tag"
                round
                :effect="tag.effect"
              >
                {{ tag.label }}
              </el-tag>
            </div>
          </el-card>
        </el-timeline-item>
      </el-timeline>
    </div>
    <!-- 时间轴的右侧部分，显示选中的Todo的详细信息 -->
    <!-- 不能改变TODO，不能点击完成按钮 -->
    <div class="right">
      <TODOItem
        :current-todo="currentTodo"
        :is-change="false"
        :is-compelete="false"
      ></TODOItem>
    </div>
  </div>
</template>

<script lang="ts" setup>
// 引入Vue的响应式API和计算属性
import { ref, computed } from "vue";
// 引入Todo类型和优先级、状态相关的功能
import { Todo, Priorities, usePriorityColor, Status, useStatus } from "../../../core";
// 引入TODOItem组件
import { TODOItem } from "../index";
// 定义接收的props，即Todo数组
const props = defineProps<{ datas: Todo[] }>();

// 定义一个响应式引用，用于存储当前选中的Todo
const currentTodo = ref<Todo>();
// 计算属性，用于根据Todo的优先级获取相应的样式
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

// 计算属性，用于根据Todo的状态获取相应的样式
const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;
#timeline-view {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  .left {
    width: 50%;
    height: 100%;
    box-sizing: border-box;
    padding-right: 12px;
    overflow-y: scroll;
    scrollbar-width: thin;
    .el-card {
      cursor: pointer;
      border: none;
      color: $bg-color-light;
    }
    :deep(.el-card__body) {
      padding: 6px;
      background-color: $bg-color-hover;
      border: none;
      h4 {
        margin: 12px 0;
        .priority {
          display: inline-block;
          height: 12px;
          width: 12px;
          border-radius: 2px;
          margin: 0 4px;
        }
      }
    }
    .operation-btn-wrapper {
      box-sizing: border-box;
      padding-bottom: 16px;
    }
  }
  .right {
    width: 50%;
    box-sizing: border-box;
    padding: 0 6px;
    height: 100%;
    border-left: 1px dashed $bg-color-hover;
  }
}
</style>
