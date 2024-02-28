<template>
  <div id="pending-view">
    <div class="left">
      <!-- 时间轴类型的列表 -->
      <el-timeline>
        <el-timeline-item
          :timestamp="`${item.date.start}~${item.date.end}`"
          placement="top"
          v-for="(item, index) in datas"
          :key="index"
        >
          <el-card @click="currentTodo = item">
            <h4>
              <span class="priority" :style="getPriorityDot(item)"></span>
              <span
                class="priority"
                style="border-radius: 50%"
                :style="getStatusDot(item)"
              ></span>
              <!-- TODO的名称 -->
              <span class="collapse-title">{{ item.name }}</span>
            </h4>
            <p>{{ item.description }}</p>
            <!-- TODO的所有标签 -->
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
            <div>
              <el-tooltip
                content="Convert Pending TODO to In Progress TODO and set Priority Emergent"
                placement="top"
              >
              <!-- 转换为紧急TODO的按钮 -->
                <el-button type="warning" @click="convertToEmergent"
                  >Resurrection</el-button
                >
              </el-tooltip>
              <el-tooltip content="Convert Pending TODO to Failed TODO" placement="top">
                <!-- 转换为失败状态的TODO的按钮 -->
                <el-button type="danger" @click="convertToFailed"
                  >Let it failed</el-button
                >
              </el-tooltip>
            </div>
          </el-card>
        </el-timeline-item>
      </el-timeline>
    </div>
    <div class="right">
      <!-- 显示TODO，不能够更改，不能够点击完成TODO -->
      <TODOItem
        :current-todo="currentTodo"
        :is-change="false"
        :is-compelete="false"
      ></TODOItem>
    </div>
  </div>
</template>

<script lang="ts" setup>
// 引入Vue相关功能，包括响应式引用(ref, reactive)，计算属性(computed)和toRaw
import { ref, reactive, computed, toRaw } from "vue";
// 引入Todo类型、优先级、状态相关的功能和钩子
import { Todo, Priorities, usePriorityColor, Status, useStatus } from "../../../core";
// 引入TODOItem组件
import { TODOItem } from "../index";
// 引入API服务
import api from "../../../api";
// 引入用户状态管理
import { user } from "../../../store/src/user";
// 引入element-plus的消息提示组件
import { ElMessage } from "element-plus";

// 定义组件接收的props，类型为Todo数组
const props = defineProps<{ datas: Todo[] }>();
// 使用用户状态管理
const userStore = user();
// 定义当前操作的Todo
const currentTodo = ref<Todo>();

// 计算属性，用于获取Todo项的优先级颜色
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

// 计算属性，用于获取Todo项的状态颜色
const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});

// 转换当前Todo的状态为进行中和优先级为紧急，同时处理参与者和审核者的数据格式
const convertTodo = () => {
  let todo: any = toRaw(currentTodo.value);
  todo!.status = Status.IN_PROGRESS;
  todo!.priority = Priorities.Emergent;
  if (todo?.performers.length ?? 0 !== 0) {
    todo.performers! = todo?.performers.map((x: any) => x.username);
  }
  if (todo?.reviewers.length ?? 0 !== 0) {
    todo.reviewers! = todo?.reviewers.map((x: any) => x.username);
  }
  // 设置Todo的所有者为当前用户
  Object.assign(todo, { owner: userStore.user.username });
  return todo;
};

// 将当前Todo标记为紧急并更新，成功后显示成功消息并更新用户状态
const convertToEmergent = async () => {
  let todo = convertTodo();
  const data = await api.todo.updateTodo(userStore.user.username, todo!.id!, todo!);
  if (data) {
    ElMessage({
      type: "success",
      message: "Revive TODO successfully",
    });
    const user = await api.user.getUserInfo(userStore.user.username);
    userStore.setUser(user!);
  }
  currentTodo.value = undefined;
};

// 将当前Todo标记为失败并更新，成功后显示成功消息
const convertToFailed = async () => {
  let todo = convertTodo();
  const data = await api.todo.failedTodo(userStore.user.username, todo!.id!, todo!);
  if (data) {
    ElMessage({
      type: "success",
      message: "Failed TODO successfully",
    });

    userStore.setUser(data!);
  }
  currentTodo.value = undefined;
};
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;
#pending-view {
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
