<template>
  <div :id="buildView(component)">
    <!-- 主容器，使用动态ID，基于组件名称生成 -->
    <div :class="buildWrap(component, 'teams')">
      <!-- 团队列表容器 -->
      <div v-if="userStore.user.teams?.length" style="height: 100%; width: 100%">
        <!-- 显示用户所属团队列表 -->
        <div
          v-for="(item, index) in userStore.user.teams"
          :key="index"
          class="team_wrapper"
          @click="currentTeam = item"
        >
          <!-- 循环显示每个团队，并设置点击事件以选择当前团队 -->
          <div class="team-avatar">
            <!-- 团队头像 -->
            <img :src="useTeam(item.avatar)" alt="" class="teamIcons" />
          </div>
          <div class="team-details">
            <!-- 团队详情 -->
            <h5 class="name">{{ item.name }}</h5>
            <p class="desc">{{ item.description }}</p>
          </div>
        </div>
      </div>
      <div v-else>
        <!-- 当没有团队时显示提示信息 -->
        <h4>😅You currently do not have a team, please create it first</h4>
        <div class="add-wrapper" @click="createNewTeam">
          <!-- 添加团队按钮 -->
          <div v-html="useSvg(SVGs.MENU_COLLABORATE, 32)"></div>
        </div>
      </div>
    </div>
    <div :class="buildWrap(component, 'details')">
      <!-- 团队详情容器 -->
      <div :class="buildWrap('details', 'team-member')">
        <!-- 团队成员列表容器 -->
        <div v-if="currentTeam" style="height: 100%; width: 100%">
          <!-- 如果已选择团队，显示团队成员 -->
          <div
            class="team-member-wrapper"
            v-for="item in currentTeam?.members"
            :key="item.username"
            @click="chooseMember(item)"
          >
            <!-- 循环显示团队成员，并设置点击事件 -->
            <div class="team-avatar">
              <!-- 成员头像 -->
              <img :src="useAvatar(item.avatar)" alt="" class="teamIcons" />
            </div>
            <div class="team-details">
              <!-- 成员详情 -->
              <h5 class="name">{{ item.name }}</h5>
              <p class="desc">{{ item.email }}</p>
            </div>
          </div>
        </div>
        <div v-else>
          <!-- 当没有选择团队时显示提示信息 -->
          <h4>You can choose a team and get team members</h4>
        </div>
      </div>
      <div :class="buildWrap('details', 'panel')">
        <!-- 控制面板容器 -->
        <Panel
          :data="currentTeam"
          @create="createNewTeam"
          @add="addMember"
          :add-member-disabled="addMemberDisabled"
          @change-team="refresh"
        ></Panel>
        <!-- 面板组件，用于团队管理操作 -->
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
// Vue 3 <script setup> 格式，使用Composition API
import { ref, computed } from "vue";
import Panel from "./components/panel.vue"; // 引入面板组件
import { Team, buildView, buildWrap, useAvatar, useTeam } from "../../core"; // 引入辅助函数
import api from "../../api"; // 引入API接口
import { user as userPinia } from "../../store/src/user"; // 引入Pinia用户状态管理
import { SVGs, useSvg } from "../../components"; // 引入SVG处理函数
import { ElMessage, ElMessageBox } from "element-plus"; // 引入Element Plus消息和对话框组件

const component = "Collaborate"; // 组件名称
const userStore = userPinia(); // 用户状态存储实例

const currentTeam = ref<Team>(); // 当前选中的团队
const currentMember = ref(); // 当前选中的成员

const addMemberDisabled = computed(() => {
  // 计算属性，判断是否禁用添加成员按钮
  return typeof currentTeam.value === "undefined";
});

const chooseMember = (item: any) => {
  // 选择当前成员的函数
  currentMember.value = item;
};

const createNewTeam = () => {
  // 创建新团队的函数
  ElMessageBox.prompt("create a new team for self", "Create Team", {
    confirmButtonText: "Create",
    cancelButtonText: "Cancel",
    inputPlaceholder: "Please enter a new team name",
  }).then(({ value }) => {
    let name = value.trim();
    if (name.length === 0) {
      ElMessage({
        type: "warning",
        message: "Please do not enter an empty team name",
      });
      return;
    }
    api.team.createTeam(userStore.user.username, name).then((user) => {
      console.log(user);
      if (typeof user !== "undefined") {
        userStore.setUser(user);
      }
      ElMessage({
        type: "success",
        message: "Team created successfully",
      });
    });
  });
};

const addMember = () => {
  // 添加新团队成员的函数
  ElMessageBox.prompt("add new team member", "Add Member", {
    confirmButtonText: "Add",
    cancelButtonText: "Cancel",
    inputPlaceholder: "Please enter member's username",
  }).then(({ value }) => {
    let name = value.trim();
    if (name.length === 0) {
      ElMessage({
        type: "warning",
        message: "Please do not enter an empty member username",
      });
      return;
    }

    let members = currentTeam.value?.members;

    if (members?.filter((member) => member.username === name).length !== 0) {
      ElMessage({
        type: "warning",
        message: "The current user already exists",
      });
      return;
    }

    api.team
      .updateTeamMember(name, currentTeam.value!)
      .then((update) => {
        if (update) {
          refresh();
          ElMessage({
            type: "success",
            message: "Add member successfully",
          });
        } else {
          ElMessage({
            type: "error",
            message: "Add member failed, please check member's username",
          });
        }
      })
      .catch(() => {});
  });
};

const refresh = () => {
  // 刷新团队信息的函数
  api.user.getUserInfo(userStore.user.username).then((user) => {
    userStore.setUser(user!);
    currentTeam.value = undefined;
  });
};
</script>

<style lang="scss" scoped>
@import "../../styles/views/collaborate.scss"; // 引入特定的SCSS样式文件
</style>
