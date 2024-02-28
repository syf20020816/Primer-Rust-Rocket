<template>
  <div id="calendar-view">
    <!-- 左侧部分为日历视图 -->
    <div class="left">
      <!-- 使用Element Plus的日历组件 -->
      <el-calendar>
        <!-- 自定义日历单元格的内容 -->
        <template #date-cell="{ data }">
          <!-- 当单元格被点击时，调用getTODO方法，并确保点击区域为整个单元格 -->
          <div @click="getTODO(data)" style="height: 100%; width: 100%">
            <!-- 显示日期，并在选中的日期旁边显示一个小旗子图标 -->
            <p :class="data.isSelected ? 'is-selected' : ''">
              <!-- 只显示月份和日期，忽略年份 -->
              {{ data.day.split("-").slice(1).join("-") }}
              <!-- 如果是选中的日期，则显示一个小旗子图标 -->
              {{ data.isSelected ? "🚩" : "" }}
            </p>
          </div>
        </template>
      </el-calendar>
    </div>
    <!-- 右侧部分为待办事项列表视图 -->
    <div class="right">
      <!-- 当有待办事项时，使用Element Plus的折叠面板展示待办事项 -->
      <el-collapse accordion v-if="todos?.length">
        <!-- 循环展示待办事项，每个待办事项为一个折叠面板项 -->
        <el-collapse-item :name="item.id" v-for="item in todos" :key="item.id">
          <!-- 自定义面板标题 -->
          <template #title>
            <div class="collapse-title-wrapper">
              <div>
                <!-- 显示待办事项的优先级和状态 -->
                <span class="priority" :style="getPriorityDot(item)"></span>
                <span
                  class="priority"
                  style="border-radius: 50%"
                  :style="getStatusDot(item)"
                ></span>
                <!-- 显示待办事项的名称 -->
                <span class="collapse-title">{{ item.name }}</span>
              </div>
            </div>
          </template>
          <!-- 面板内容区域，展示待办事项的详细信息 -->
          <div style="height: 360px">
            <TODOItem
              :is-compelete="false"
              :current-todo="item"
              :is-change="false"
              @refresh="refreshTodo"
              @delete="refreshTodo"
            ></TODOItem>
          </div>
        </el-collapse-item>
      </el-collapse>
      <!-- 当没有待办事项时，显示一条友好的消息 -->
      <div v-else>
        <h3>Wishing you a pleasant day</h3>
        <p>There are no TODOs to be processed for the current date</p>
      </div>
    </div>
  </div>
</template>


<script lang="ts" setup>
// 引入Vue的响应式API、计算属性、以及组件挂载时的生命周期钩子
import { ref, computed, onMounted } from "vue";
// 引入Todo模型、优先级和状态的相关功能
import { Todo, Priorities, usePriorityColor, Status, useStatus } from "../../../core";
// 引入TODOItem组件
import { TODOItem } from "../index";

// 从父组件接收Todo数据作为props
const props = defineProps<{ datas: Todo[] }>();
// 定义组件可以触发的自定义事件，这里为"getDate"
const emits = defineEmits(["getDate"]);

// 定义一个响应式引用，用于存储筛选后的Todo列表
const todos = ref<Todo[]>();
// 计算属性，根据Todo的优先级返回对应颜色的样式字符串
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item;
  return `background-color : ${usePriorityColor(priority || Priorities.Low)}`;
});

// 计算属性，根据Todo的状态返回对应颜色的样式字符串
const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});

// 根据指定日期筛选Todo列表
const filterTodos = (date: Date) => {
  let time = new Date(date.toLocaleDateString()).getTime();
  // 使用filter方法筛选出在指定日期范围内的Todo项
  todos.value = props.datas.filter((todo) => {
    let end = new Date(new Date(todo.date.end).toLocaleDateString()).getTime();
    let start = new Date(new Date(todo.date.start).toLocaleDateString()).getTime();

    return start <= time && end >= time;
  });
};

// 获取指定日期的Todo项并筛选
const getTODO = (data: any) => {
  let date = data.date;
  console.log(date);
  filterTodos(date);
};

// 用于刷新Todo项的函数，目前仅打印传入的id
const refreshTodo = (_id: string) => {
  console.log(_id);
};

// 组件挂载时，初始化筛选为当前日期的Todo项
filterTodos(new Date());
</script>

<style lang="scss">
@use '../../../styles/src/var.scss' as *;
#calendar-view {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  .left {
    height: 100%;
    box-sizing: border-box;
    padding: 0 6px;
    width: 50%;
    .el-calendar {
      background-color: $bg-color-hover;
    }
    .el-calendar-day {
      &:hover {
        background-color: $force-color !important;
      }
    }
    .current {
      &:hover {
        background-color: $force-color !important;
      }
    }
    .is-today {
      color: $bg-color-dark !important;
    }
    :deep(.el-calendar-table td.is-selected) {
      background-color: $force-color !important;
    }
    .is-selected {
      // background-color: var(--el-calendar-selected-bg-color);
      background-color: $force-color;
    }
  }
  .right {
    height: 100%;
    width: 50%;
    padding: 0 6px;
    box-sizing: border-box;
    border-left: 1px dashed $bg-color-hover;
    overflow-y: scroll;
    scrollbar-width: thin;
    .priority {
      display: inline-block;
      height: 12px;
      width: 12px;
      border-radius: 2px;
      margin: 0 4px;
    }
    .collapse-title-wrapper {
      box-sizing: border-box;
      padding: 0 8px;
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .collapse-title {
        font-weight: 700;
        color: $bg-color-light;
      }
    }

    .el-collapse-item__header {
      background-color: $bg-color-menu !important;
    }
    .el-collapse-item__content {
      margin: 0;
      padding: 0;
      background-color: $bg-color-hover !important;
    }
  }
}
</style>
