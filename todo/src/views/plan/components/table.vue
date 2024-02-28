<template>
  <!-- 表格视图容器 -->
  <div id="table-view">
    <!-- 表格元素，数据来源于datas属性 -->
    <el-table :data="datas">
      <!-- 表格列配置 -->
      <el-table-column type="index" label="Index" width="100px" fixed="left" />
      <el-table-column label="Name" prop="name" width="200px" />
      <el-table-column label="Priority" prop="priority" width="200px" />
      <!-- 审核人列，自定义内容展示 -->
      <el-table-column label="Reviewers" width="200px" prop="reviewers">
        <template #default="{ row, $index }">
          <div>{{ getReviewerName(row, $index) }}</div>
        </template>
      </el-table-column>
      <!-- 执行者列，自定义内容展示 -->
      <el-table-column label="Performers" prop="performers" width="200px">
        <template #default="{ row, $index }">
          <div>{{ getPerformersName(row, $index) }}</div>
        </template>
      </el-table-column>
      <!-- 日期列，自定义内容展示 -->
      <el-table-column label="Date" prop="date" width="360px">
        <template #default="{ row, $index }">
          <div>{{ getDate(row, $index) }}</div>
        </template>
      </el-table-column>
      <!-- 标签列，自定义内容展示 -->
      <el-table-column label="Tags" prop="tags" width="240px">
        <template #default="{ row, $index }">
          <span>
            <!-- 标签循环展示 -->
            <el-tag
              style="margin: 0 4px"
              v-for="tag in getTags(row, $index)"
              :key="tag.label"
              :type="tag.type"
              size="small"
              class="mx-tag"
              round
              :effect="tag.effect"
              >{{ tag.label }}</el-tag
            >
          </span>
        </template>
      </el-table-column>
      <!-- 操作列 -->
      <el-table-column align="right" width="120px" fixed="right">
        <template #header>
          <span>Operation</span>
        </template>
      </el-table-column>
      <!-- 展开列，用于展示更多信息 -->
      <el-table-column type="expand" fixed="right">
        <template #default="props">
          <div class="expand-table-wrapper">
            <TODOItem
              :current-todo="props.row"
              :is-change="false"
              :is-compelete="false"
            ></TODOItem>
          </div>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script lang="ts" setup>
// 引入Vue相关功能，包括响应式引用(ref, reactive)和计算属性(computed)
import { ref, reactive, computed } from "vue";
// 引入Todo类型定义，用于类型标注和类型安全
import { Todo } from "../../../core";
// 引入TODOItem组件，可能用于展开行展示待办事项的详细信息
import { TODOItem } from "../index";

// 使用defineProps定义组件接收的props，指定datas属性，其类型为Todo数组
const props = defineProps<{
  datas: Todo[];
}>();

// 计算属性getReviewerName，用于获取待办事项中审核人的姓名
const getReviewerName = computed(() => (row: any, index: number) => {
  // 安全访问row.reviewers数组中的name属性，如果不存在则返回空字符串
  return row.reviewers[index]?.name ?? "";
});

// 计算属性getPerformersName，用于获取待办事项中执行者的姓名
const getPerformersName = computed(() => (row: any, index: number) => {
  // 安全访问row.performers数组中的name属性，如果不存在则返回空字符串
  return row.performers[index]?.name ?? "";
});

// 计算属性getDate，用于格式化待办事项的日期范围
const getDate = computed(() => (row: any, _index: number) => {
  // 安全解构row.date对象，获取开始和结束日期，如果不存在则默认为空字符串
  let { start, end } = row.date ?? {
    start: "",
    end: "",
  };
  // 格式化日期范围为"start ~ end"的形式，如果某个日期不存在则不显示该日期
  return `${start ?? ""} ~ ${end ?? ""}`;
});

// 计算属性getTags，用于获取待办事项的标签数组
const getTags = computed(() => (row: any, _index: number): {
  type: string;
  effect: string;
  label: string;
}[] => {
  // 直接返回row.tags数组，包含每个标签的类型(type)、效果(effect)和标签文本(label)
  return row.tags;
});
</script>

<style lang="scss">
@use '../../../styles/src/var.scss' as *;

#table-view {
  box-sizing: border-box;
  height: 100%;
  // padding: 6px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  overflow-y: scroll;
  scrollbar-width: thin;
  flex-wrap: wrap;

  // 覆盖el-table默认颜色
  // 注意：这里覆盖了element-plus的样式，如果需要使用element-plus的样式，需要将element-plus的样式覆盖
  // 例如：@import "element-plus/theme-chalk/src/common/var.scss";
  // 或者使用!important来覆盖样式
  .el-table {
    --el-table-bg-color: #3b3a39;
    --el-table-header-bg-color: #252423;
    --el-table-tr-bg-color: #3b3a39;
    --el-table-row-hover-bg-color: #252423;
    --el-table-border: 1px solid #3b3a39;
    --el-table-border-color: #3b3a39;
    --el-table-text-color: #fff;
    --el-table-header-text-color: #bfbfbf;
    --el-table-expanded-cell-bg-color: #252423;

    .el-table__body-wrapper tr td.el-table-fixed-column--left {
      background: #3b3a39;
    }

    .el-table__body-wrapper tr td.el-table-fixed-column--right {
      background: #3b3a39;
    }
  }

  .expand-table-wrapper {
    width: calc(100% - 240px);
    display: flex;
    min-width: 620px;
    align-items: center;
    box-sizing: border-box;
    padding: 16px;
    justify-content: center;
    height: 420px;
  }
}
</style>
