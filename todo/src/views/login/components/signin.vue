<template>
  <!-- 动态绑定id，通过buildView函数构建，以实现组件的可复用性 -->
  <div :id="buildView(component)">
    <!-- 使用动态类名构建表单的外包装，利用buildWrap函数 -->
    <div :class="buildWrap(component, 'form')">
      <!-- 表单组件 -->
      <!-- 表单引用，用于表单验证等操作 -->
      <!-- 表单数据对象 -->
      <!-- 表单验证规则 -->
      <!-- 标签宽度 -->
      <!-- 自定义样式类 -->
      <!-- 表单尺寸 -->
      <!-- 标签位置 -->
      <!-- 显示验证状态图标 -->
      <el-form
        ref="userFormRef"
        :model="userForm"
        :rules="rules"
        label-width="120px"
        class="demo-userForm"
        :size="formSize"
        label-position="left"
        status-icon
      >
        <!-- 用户名输入项 -->
        <el-form-item label="username" prop="username">
          <el-input
            v-model="userForm.username"
            placeholder="please input your username"
          />
        </el-form-item>
        <!-- 密码输入项，显示密码切换按钮 -->
        <el-form-item label="password" prop="password">
          <el-input
            v-model="userForm.password"
            show-password
            type="password"
            placeholder="please input your password"
          />
        </el-form-item>
        <!-- 操作按钮 -->
        <el-form-item>
          <el-button type="primary" @click="submitForm(userFormRef)">Signin</el-button>
          <el-button type="warning" @click="toSignup"
            >No Account? Click to Signup</el-button
          >
        </el-form-item>
      </el-form>
    </div>
    <!-- 提示信息 -->
    <div :class="buildWrap(component, 'infos')">
      <el-alert
        title="username"
        type="info"
        description="you should input your username and the length should be 5 to 16"
        show-icon
      />
      <el-alert
        title="password"
        type="info"
        description="you should input the password and the length should be 5 to 16"
        show-icon
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
// 导入所需模块和组件
import {
  AvatarMap,
  Avatars,
  build,
  buildView,
  buildWrap,
  useTeam,
  UserLoginForm,
} from "../../../core";
import { ref, reactive } from "vue";
import api from "../../../api";
import { ElMessage, ElMessageBox } from "element-plus";
import { user as userPinia } from "../../../store/src/user";
import type { FormInstance, FormRules } from "element-plus";

// 组件名称，用于构建动态id和类名
const component = "Signin";
// 使用Pinia管理用户状态
const userStore = userPinia();
// 表单尺寸，默认为"default"
const formSize = ref("default");
// 表单引用，用于操作表单
const userFormRef = ref<FormInstance>();
// 表单数据模型
const userForm = reactive<UserLoginForm>({
  username: "",
  password: "",
});

// 表单验证规则
const rules = reactive<FormRules<UserLoginForm>>({
  username: [
    { required: true, message: "Please input your username", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
  password: [
    { required: true, message: "Please input your password", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
});

// 提交表单的方法
const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate(async (valid, _fields) => {
    if (valid) {
      const data = await api.user.signin(userForm);
      if (typeof data !== "undefined") {
        ElMessage({
          type: "success",
          message: "Sign in successfully",
        });
        // 设置用户信息和登录状态
        userStore.setUser(data);
        userStore.setSignIn();
      }
    } else {
      return;
    }
  });
};

// 定义组件向父组件发出事件的能力
const emits = defineEmits(["to-signup"]);

// 跳转到注册页面的方法
const toSignup = () => {
  emits("to-signup");
};
</script>

<style lang="scss" scoped>
@import "../../../styles/views/signin.scss";
</style>
