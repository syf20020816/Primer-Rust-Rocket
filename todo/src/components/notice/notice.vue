<template>
  <!-- 通知容器 -->
  <div id="notice_container">
    <header>
      <!-- 显示通知标题 -->
      <h4>{{ data.title }}</h4>
    </header>
    <footer>
      <!-- 显示通知来源 -->
      <div>from: {{ data.notifier }}</div>
      <!-- 显示通知日期 -->
      <div>date: {{ data.date }}</div>
      <!-- 检查通知按钮，点击触发checkNotice方法 -->
      <div class="check" @click="checkNotice">check</div>
    </footer>
  </div>
</template>

<script lang="ts" setup>
// 引入Vue相关函数
import { ref, reactive } from "vue";
// 引入通知类型定义
import { Notice } from "../../core";
// 引入Element Plus组件
import { ElMessage, ElMessageBox } from "element-plus";

// 定义接收的props
const props = defineProps<{
  data: Notice; // 通知数据类型
}>();

// 检查通知的方法
const checkNotice = () => {
  // 类型标记，用于区分不同的操作
  let type = 0;
  // 根据通知的类型，决定取消按钮的文本
  let cancelButtonText =
    props.data.type === "todo"
      ? ((): string => {
          // 如果有审核人，则设置类型为1，并返回对应的文本
          if (props.data.data.reviewers?.length !== 0) {
            type = 1;
            return "Send Notifications to Reviewers";
          }
          // 没有审核人，则设置类型为2，并返回对应的文本
          type = 2;
          return "Change Date Self";
        })()
      : "Cancel"; // 如果不是todo类型的通知，则取消按钮显示为“Cancel”

  // 显示确认对话框
  ElMessageBox.confirm(props.data.description, "Handle", {
    confirmButtonText: "I have been completed", // 确认按钮文本
    cancelButtonText, // 取消按钮文本
    type: "warning", // 对话框类型
  })
    .then(() => {
      // 确认操作
      ElMessage({
        type: "success",
        message: "Handled", // 操作成功的提示信息
      });
    })
    .catch(() => {
      // 取消操作
      let message = "You can go to plan and delay TODO date or pending this TODO";
      // 根据类型，调整取消操作后的提示信息
      if (type === 1) {
        message = "System has been sent notification to your reviewers";
      }

      // 显示提示信息
      ElMessage({
        type: "info",
        message,
      });
    });
};
</script>


<style lang="scss" scoped>
#notice_container {
  border-bottom: 2px dashed #444c58;
  padding-bottom: 6px;

  h4 {
    margin: 6px 0;
  }

  .check {
    text-align: end;
    color: #de511a;
    font-weight: 700;
    font-style: oblique;
    cursor: pointer;
    float: right;
    transform: translateY(-16px);
  }
}
</style>
