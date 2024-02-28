// 用户个人中心页
<template>
  <!-- 用户界面的根元素，其id通过buildView方法动态生成，依据组件名 -->
  <div :id="buildView(component)">
    <!-- 用户头像展示区域，点击可以打开选择头像的抽屉 -->
    <div :class="buildWrap(component, 'avatar')">
      <img
        @click="avatarDrawer = true"
        id="userAvatar"
        :src="userStore.useAvatar(userStore.user.avatar)"
        alt="Avatar"
      />
      <!-- 用户基本信息展示，包括用户名和电子邮件 -->
      <div :class="buildWrap('avatar', 'userInfo')">
        <div v-html="useSvg(SVGs.LOGO, 32)"></div>
        <div class="title">
          Hello!{{ userStore.user.name }} Wishing you a pleasant day
        </div>
        <div class="email">Email:{{ userStore.user.email }}</div>
      </div>
    </div>
    <!-- 用户详细信息区域，包括只读的用户信息和用户繁忙程度评分 -->
    <div :class="buildWrap(component, 'details')">
      <div class="details-title">
        <span>UserInfo(ReadOnly)</span>
        <!-- 用户繁忙程度评分，使用element-plus的评分组件展示 -->
        <div class="info-extra">
          <el-rate
            v-model="busyValue"
            :icons="busyIcons"
            show-score
            disabled
            :void-icon="Coffee"
            :colors="['#409eff', '#FF9900', '#ff0000']"
            :score-template="busyTemplate"
          />
          <!-- 按钮，用于跳转至处理待办事项的页面 -->
          <el-button type="primary" @click="toPlan">Solve TODOs</el-button>
        </div>
      </div>
      <!-- 循环展示用户信息列表 -->
      <div class="info-details-list">
        <div class="info-item" v-for="(item, index) in userInfoList" :key="index">
          <div class="item-title">{{ item.label }} :</div>
          <div class="item-value">{{ item.value }}</div>
        </div>
      </div>
      <!-- 展示用户解决待办事项的进度 -->
      <el-progress
        :percentage="countSolve"
        :stroke-width="15"
        striped
        striped-flow
        :duration="100"
        :format="(percentage:any) => (percentage === 100 ? 'Full' : `${percentage}%`)"
      />
    </div>
    <!-- 用户团队信息展示区域 -->
    <div :class="buildWrap(component, 'teams')">
      <!-- 仅当用户有团队时显示 -->
      <div style="display: flex" v-if="hasTeam">
        <!-- 循环展示用户的所有团队 -->
        <div class="team-item" v-for="(item, index) in userStore.user.teams" :key="index">
          <div class="header">
            <img :src="useTeam(item.avatar)" alt="" class="teamIcons" />
            <div class="info">
              <div>Team : {{ item.name }}</div>
              <div>Role : {{ setTeamRole(item.owner) }}</div>
            </div>
          </div>
          <div style="font-size: 14px; width: 100%">
            {{ item.description }}
          </div>
          <el-button type="info" @click="router.push({ path: '/collaborate' })"
            >GOTO</el-button
          >
        </div>
      </div>
      <!-- 当用户没有团队时显示的提示信息 -->
      <div v-else class="team-no">
        <p>
          You now have no Team.
          <a @click="toCollaborate" class="create-wrapper">Click here to create!</a>
        </p>
      </div>
    </div>
  </div>
  <!-- 头像选择抽屉，用于用户更换头像 -->
  <el-drawer
    v-model="avatarDrawer"
    title="Please choose your Avatar"
    :with-header="true"
    custom-class="avatarDrawer"
    size="400px"
  >
    <div class="drawer-details">
      <!-- 头像列表，用户可以点击选择 -->
      <div class="avatar-list">
        <div
          @click="chooseAvatar(item)"
          class="avatar-item"
          v-for="(item, index) in avatarList"
          :key="index"
        >
          <img class="common-img" :src="userStore.useAvatar(item.label)" alt="Avatar" />
          <div class="infos">
            <div>avatar icon : {{ item.label }}</div>
          </div>
        </div>
      </div>
      <!-- 显示当前选择的头像 -->
      <div class="chooseInfo">
        <div>Your Avatar : {{ userStore.user.avatar }}</div>
        <div>Chosen : {{ chosenAvatar }}</div>
      </div>
      <!-- 按钮用于提交头像更换请求 -->
      <div style="width: 100%">
        <el-button type="warning" @click="changeAvatar">
          <strong>change avatar</strong>
        </el-button>
      </div>
    </div>
  </el-drawer>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
//引入图标
import { Coffee, Platform, WarningFilled } from "@element-plus/icons-vue";
import {
  AvatarMap,
  Avatars,
  TeamAvatars,
  User,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../core";
import api from "../api"; // 引入 API 调用方法
import { user } from "../store/src/user"; // 引入用户状态管理
import { SVGs, useSvg } from "../components"; // 引入 SVG 组件和相关 hooks
import { useRouter } from "vue-router"; // 引入 Vue Router
import { ElMessage } from "element-plus"; // 引入 Element Plus 消息提示组件

// 控制头像选择抽屉的显示状态
const avatarDrawer = ref(false);
const component = "User";
const router = useRouter();
const userStore = user();
// 选中的头像
const chosenAvatar = ref(userStore.user.avatar);
// 计算用户的繁忙程度
const busyValue = computed(() => {
  let { todos, todoNumber } = userStore.user;
  let { fatal, focus, mid, low } = todos ?? {
    fatal: [],
    focus: [],
    mid: [],
    low: [],
  };
  // 根据不同优先级的待办事项计算得分
  let score = fatal.length * 5 + mid.length * 2 + low.length * 1;
  let total = todoNumber * 5;
  // 返回繁忙程度的分数,最大值为5最小值为0
  return (score / total) * 5;
});
const busyIcons = [Coffee, Platform, WarningFilled];

/**头像列表 */
const avatarList = computed(() => {
  let avatars: { label: Avatars; value: any }[] = [];
  AvatarMap.forEach((value, key, _map) => {
    avatars.push({
      label: key,
      value,
    });
  });
  return avatars;
});

/**选择头像事件 */
const chooseAvatar = (item: { label: Avatars; value: any }) => {
  chosenAvatar.value = item.label;
};

/**修改用户头像 */
const changeAvatar = async () => {
  let username = userStore.getUsername();
  if (username) {
    const data = await api.user.setUserAvatar(username, chosenAvatar.value);
    data &&
      (() => {
        userStore.user.avatar = chosenAvatar.value;
        ElMessage({
          type: "success",
          message: "Avatar changed successfully",
        });
      })();
  }
  avatarDrawer.value = false;
};

/**计算用户繁忙程度 */
const busyTemplate = computed(() => {
  let msg = "Free";
  if (busyValue.value >= 4) {
    msg = "Busy";
  } else if (busyValue.value >= 2) {
    msg = "Moderate";
  }
  return `busy level: ${msg}!`;
});

// 用户信息列表包括用户名、姓名、邮箱等
const userInfoList = computed(() => {
  let { user } = userStore;
  return [
    {
      label: "UserName",
      value: user.username,
    },
    {
      label: "Name",
      value: user.name,
    },
    {
      label: "Email",
      value: user.email,
    },
    {
      label: "Avatar",
      value: user.avatar,
    },
    {
      label: "TeamNumber",
      value: user.teamNumber,
    },
    {
      label: "Todos",
      value: user.todoNumber,
    },
    {
      label: "TODO-fatal",
      value: user.todos?.fatal.length ?? 0,
    },
    {
      label: "TODO-mid",
      value: user.todos?.mid.length ?? 0,
    },
    {
      label: "TODO-low",
      value: user.todos?.low.length ?? 0,
    },
    {
      label: "TODO-Focus",
      value: user.todos?.focus.length ?? 0,
    },
  ];
});

// 计算用户解决的待办事项百分比
const countSolve = computed(() => {
  let { todoNumber, totalTodo } = userStore.user;
  if (todoNumber === 0 || totalTodo === 0) {
    return 100;
  }
  let solve = (1 - todoNumber / totalTodo).toFixed(2);

  let solveF = parseFloat(solve) * 100;
  return solveF;
});

// 根据是否为团队所有者返回用户角色
const setTeamRole = computed(() => (owner: string) => {
  let { username } = userStore.user;
  if (username === owner) {
    return "Manager";
  } else {
    return "Parter";
  }
});

// 判断用户是否属于某个团队
const hasTeam = computed(() => {
  return userStore.user.teams?.length ?? 0 !== 0;
});

// 路由跳转到协作页面
const toCollaborate = () => {
  router.push({ path: "collaborate" });
};

// 路由跳转到计划页面
const toPlan = () => {
  router.push({ path: "plan" });
};
</script>

<style lang="scss">
@import "../styles/views/user.scss";
</style>
