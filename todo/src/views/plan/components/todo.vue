<template>
  <!-- 仅当currentTodo定义时展示todo项 -->
  <div class="todo" v-if="typeof currentTodo !== 'undefined'">
    <!-- 头部区域：包含优先级、todo名称和操作按钮 -->
    <div class="header">
      <!-- 优先级点的展示 -->
      <span class="dot_wrapper">
        <span class="priority" :style="getPriorityDot(currentTodo)"></span>
      </span>
      <!-- Todo名称 -->
      <div class="todo-name">{{ currentTodo.name }}</div>
      <!-- 完成按钮，仅当isCompelete为true时显示 -->
      <el-button type="success" @click="completeTodo" v-if="isCompelete"
        >✅complete</el-button
      >
      <!-- 更改按钮，仅当isChange为true时显示 -->
      <el-button type="warning" @click="changeTodo" v-if="isChange">change</el-button>
      <!-- 删除按钮 -->
      <el-button type="danger" @click="deleteTodo">❌discard</el-button>
      <!-- 待处理按钮，仅当isPending计算属性为true时显示 -->
      <el-button type="info" @click="pendingTodo" v-if="isPending">🦥pending</el-button>
    </div>
    <!-- 详情区域 -->
    <div class="details">
      <div class="left">
        <!-- 标签展示 -->
        <div class="tags">
          tag :
          <el-tag
            v-for="tag in currentTodo.tags"
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
        <!-- 审核人展示 -->
        <div class="reviews">
          <div style="text-align: left">Reviews:</div>
          <div
            class="review-item"
            v-for="(review, index) in currentTodo.reviewers"
            :key="index"
          >
            <img class="review-avatar" :src="useAvatar(review.avatar)" alt="" />
            <div class="review-info">
              <div class="name">{{ review.name }}</div>
              <div class="email">{{ review.email }}</div>
            </div>
          </div>
        </div>
        <!-- 执行人展示 -->
        <div class="reviews">
          <div style="text-align: left">Performers:</div>
          <div
            class="review-item"
            v-for="(performer, index) in currentTodo.performers"
            :key="index"
          >
            <img class="review-avatar" :src="useAvatar(performer.avatar)" alt="" />
            <div class="review-info">
              <div class="name">{{ performer.name }}</div>
              <div class="email">{{ performer.email }}</div>
            </div>
          </div>
        </div>
      </div>
      <div class="right">
        <!-- 日期展示 -->
        <div class="date">
          <div style="text-align: left">
            DateTime : {{ currentTodo.date.start }} ~ {{ currentTodo.date.end }}
          </div>
          <div style="text-align: right">
            During : {{ countDuring(currentTodo.date.during) }}
          </div>
        </div>
        <!-- 状态展示 -->
        <div class="status">
          <span style="margin-right: 6px">Status :</span>
          <el-tag
            round
            :color="useStatus(currentTodo.status)"
            class="mx-tag"
            effect="dark"
          >
            {{ currentTodo.status }}
          </el-tag>
        </div>
        <!-- 描述展示 -->
        <div class="des">
          <span style="font-weight: 700">Description :</span>
          <p>{{ currentTodo.description }}</p>
        </div>
        <!-- 附件下载按钮 -->
        <div class="more">
          <span>Annex and Details :</span>
          <!-- 仅当有附件时显示下载按钮 -->
          <el-button
            type="default"
            v-if="currentTodo.annexs?.length !== 0"
            @click="downloadAnnexs"
            >Download</el-button
          >
        </div>
      </div>
    </div>
  </div>
</template>


<script lang="ts" setup>
// 导入Vue相关功能
import { ref, reactive, computed } from "vue";
// 导入核心功能、工具方法和状态管理
import {
  Todo, // Todo类型定义
  Priorities, // 优先级枚举
  usePriorityColor, // 优先级颜色钩子
  useAvatar, // 头像钩子（未在此段代码使用）
  useStatus, // 状态钩子（未在此段代码使用）
  downloadBlob, // Blob下载方法
  base64ToBlob, // Base64转Blob方法
  Status, // 状态枚举
} from "../../../core";
import api from "../../../api"; // API方法
import { ElMessage } from "element-plus"; // Element Plus的消息提示组件
import { user as userPinia } from "../../../store/src/user"; // 用户状态管理

// 定义组件接收的props
const props = defineProps<{
  currentTodo?: Todo; // 当前操作的Todo
  isChange: boolean; // 是否为更改操作标志
  isCompelete: boolean; // 是否已完成标志
}>();
const userStore = userPinia(); // 使用Pinia的用户状态
const emits = defineEmits(["change", "delete", "refresh"]); // 定义事件发射器

// 计算属性，用于获取代办事项的优先级颜色
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

// 计算代办事项持续时间
const countDuring = (timestamp: number): string => {
  return `${(timestamp / 1000 / 60 / 60).toFixed(2)} h`;
};

// 下载附件方法
const downloadAnnexs = () => {
  props.currentTodo?.annexs?.forEach((annex) => {
    let contentType = annex.data.split(";base64,")[0].replace("data:", "");
    let blob = base64ToBlob(annex.data, contentType);
    downloadBlob(blob, annex.name);
  });
};

// 完成代办事项方法
const completeTodo = async () => {
  let id = props.currentTodo!.id!;
  const data = await api.todo.completedTodo(userStore.user.username, id);
  if (typeof data !== "undefined") {
    ElMessage({
      type: "success",
      message: "Complete Todo successfully",
    });
    userStore.setUser(data);
  }
  emits("refresh", props.currentTodo?.id!);
};

// 发射更改代办事项事件
const changeTodo = () => {
  emits("change", props.currentTodo);
};

// 删除代办事项方法
const deleteTodo = async () => {
  let id = props.currentTodo!.id!;
  const data = await api.todo.deleteTodo(userStore.user.username, id);
  if (typeof data !== "undefined") {
    ElMessage({
      type: "success",
      message: "Delete Todo successfully",
    });
    userStore.setUser(data);
  }
  emits("delete");
};

// 将代办事项状态更改为待处理
const pendingTodo = async () => {
  let id = props.currentTodo!.id!;
  const data = await api.todo.updateTodoStatus(id, Status.PENDING);
  if (data) {
    ElMessage({
      type: "success",
      message: "Pending TODO successfully",
    });
    const user = await api.user.getUserInfo(userStore.user.username);
    userStore.setUser(user!);
    emits("refresh", props.currentTodo?.id!);
  }
};

// 计算属性，判断代办事项是否处于待处理状态
const isPending = computed(() => {
  let status = props.currentTodo?.status;
  return status !== Status.PENDING;
});
</script>


<style lang="scss" scoped>
@use '../../../styles/src/var.scss' as *;
.todo {
  color: $bg-color-light;
  height: 100%;
  border-radius: 4px;
  background-color: $bg-color-hover;
  width: 100%;
  box-sizing: border-box;
  padding: 16px;
  .header {
    height: 60px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .dot_wrapper {
      display: inline-block;
      height: 1.5em;
      display: flex;
      align-items: center;
      justify-content: center;
      margin-right: 8px;
      .priority {
        display: inline-block;
        height: 12px;
        width: 12px;
        border-radius: 2px;
        background-color: #d02828;
      }
    }
    .todo-name {
      font-size: 18px;
      width: calc(100% - 24px);
      font-weight: 700;
      text-align: left;
    }
  }
  .details {
    border-top: 1px solid $bg-color-light;
    padding-top: 12px;
    width: 100%;
    height: calc(100% - 72px);
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    .left {
      height: 100%;
      width: 36%;
      .tags {
        height: 24px;
        width: 100%;
        overflow: hidden;
        display: flex;
        align-items: center;
        justify-content: flex-start;
      }
      .mx-tag {
        margin: 0 4px;
      }
      .reviews {
        margin: 6px 0;
        height: calc(50% - 12px);
        overflow-y: scroll;
        scrollbar-width: thin;
        .review-item {
          padding: 0 6px;
          box-sizing: border-box;
          cursor: pointer;
          height: 60px;
          margin: 6px 0;
          width: 96%;
          display: flex;
          align-items: center;
          justify-content: space-between;
          transition: all 0.25s ease-in-out;
          background-color: $bg-color-menu;
          &:hover {
            background-color: $bg-color-header;
          }
          .review-avatar {
            height: 32px;
            width: 32px;
            border-radius: 50%;
          }
          .review-info {
            box-sizing: border-box;
            padding: 0 12px;
            width: calc(100% - 36px);
            .name {
              text-align: left;
              font-size: 14px;
              font-weight: 700;
              color: $force-color;
            }
            .email {
              overflow: hidden;
              text-align: left;
              font-size: 12px;
            }
          }
        }
      }
    }
    .right {
      height: 100%;
      width: 64%;
      .date {
        box-sizing: border-box;
        padding-bottom: 6px;
        border-bottom: 1px solid $bg-color-light;
      }
      .status {
        padding-top: 6px;
        height: 24px;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: flex-start;
      }
      .des {
        box-sizing: border-box;
        padding-top: 6px;
        overflow: hidden;
        height: calc(100% - 150px);
        text-align: left;
        p {
          margin: 6px;
        }
      }
      .more {
        display: flex;
        align-items: center;
        justify-content: space-between;
      }
    }
  }
}
</style>
