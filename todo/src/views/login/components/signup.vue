<template>
  <!-- 根元素，其id根据组件动态生成，有利于CSS定制和JavaScript操作 -->
  <div :id="buildView(component)">
    <!-- 使用动态类名构建表单容器的包裹层，方便样式的应用和管理 -->
    <div :class="buildWrap(component, 'form')">
      <!-- 表单组件，使用Element Plus的el-form，绑定表单数据、规则、尺寸等 -->
      <!-- 通过ref引用表单实例，用于表单验证和重置且显示表单验证状态图标 -->
      <!-- rules 绑定表单验证规则 -->
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
        <!-- 表单项：用户名 -->
        <el-form-item label="username" prop="username">
          <el-input
            v-model="userForm.username"
            placeholder="please input your username"
          />
        </el-form-item>
        <!-- 表单项：姓名 -->
        <el-form-item label="name" prop="name">
          <el-input v-model="userForm.name" placeholder="please input your name" />
        </el-form-item>
        <!-- 表单项：邮箱 -->
        <el-form-item label="email" prop="email">
          <el-input v-model="userForm.email" placeholder="please input your email" />
        </el-form-item>
        <!-- 表单项：密码 -->
        <el-form-item label="password" prop="password">
          <el-input
            v-model="userForm.password"
            show-password
            type="password"
            placeholder="please input your password"
          />
        </el-form-item>
        <!-- 表单项：确认密码 -->
        <el-form-item label="check pwd" prop="pwdCheck">
          <el-input
            show-password
            type="password"
            v-model="userForm.pwdCheck"
            placeholder="please input your password again"
          />
        </el-form-item>
        <!-- 表单操作按钮：提交和重置 -->
        <el-form-item>
          <el-button type="primary" @click="submitForm(userFormRef)">
            Create Account</el-button
          >
          <el-button @click="resetForm(userFormRef)">Reset All</el-button>
        </el-form-item>
      </el-form>
    </div>
    <!-- 信息提示区，使用el-alert组件展示各类提示信息 -->
    <div :class="buildWrap(component, 'infos')">
      <el-alert
        title="About Signup"
        type="warning"
        description="If you wanna use MYTODO, you should signup first"
        show-icon
      />
      <el-alert
        title="username"
        type="info"
        description="you should have a username and the length should be 5 to 16"
        show-icon
      />
      <el-alert
        title="name"
        type="info"
        description="you should input your name, recommend to use your real name"
        show-icon
      />
      <el-alert
        title="email"
        type="info"
        description="you should input your email, it will be used for work"
        show-icon
      />
      <!-- 返回登录界面按钮 -->
      <el-button type="success" @click="toSignin">back to signin</el-button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  buildView,
  buildWrap,
} from "../../../core"; // 导入辅助函数
import { ref, reactive } from "vue";

import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from "element-plus"; // 导入Element Plus组件和类型
import api from "../../../api"; // 导入API服务
import {user as userPinia} from '../../../store/src/user' // 导入Pinia用户状态管理

const component = "Signup"; // 组件名，用于构建id和类名
const userStore = userPinia(); // 实例化用户状态管理
const emits = defineEmits(["to-signin"]); // 定义组件可以发出的事件

// 定义用户表单的接口
interface UserForm {
  username: string;
  name: string;
  email: string;
  password: string;
  pwdCheck: string;
}

const formSize = ref("default");
const userFormRef = ref<FormInstance>();
// 初始化表单数据
const userForm = reactive<UserForm>({
  username: "",
  name: "",
  email: "",
  password: "",
  pwdCheck: "",
});

// 表单验证规则
const rules = reactive<FormRules<UserForm>>({
  username: [
    { required: true, message: "Please input your username", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
  name: [
    { required: true, message: "Please input your name", trigger: "blur" },
    { min: 1, max: 16, message: "Length should be 1 to 16", trigger: "blur" },
  ],
  email: [
    {
      required: true,
      message: "Please input email address",
      trigger: "blur",
    },
    {
      type: "email",
      message: "Please input correct email address",
      trigger: ["blur", "change"],
    },
  ],
  password: [
    { required: true, message: "Please input your password", trigger: "blur" },
    { min: 5, max: 16, message: "Length should be 5 to 16", trigger: "blur" },
  ],
});

// 提交表单的异步函数
const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      if(userForm.password !== userForm.pwdCheck){
        ElMessage({
          type:"error",
          message:"The passwords filled in twice are different, please check!"
        })
        return;
      }else{
        const data = await api.user.signup(userForm);
        if(typeof data !== 'undefined'){
          ElMessageBox.confirm('Registration successful, do you want to log in directly?','Sign up Result',{
            confirmButtonText: 'Signin Now',
            cancelButtonText: 'Cancel',
            type: 'success',
          }).then(()=>{
            userStore.setUser(data);
            userStore.setSignIn();
          })
        }
      }
    } else {
      console.log("error submit!", fields);
    }
  });
};
// 重置表单的函数
const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  formEl.resetFields();
};
// 触发父组件的to-signin事件，用于切换视图
const toSignin = () => {
  emits("to-signin");
};
</script>

<style lang="scss" scoped>
@import "../../../styles/views/signup.scss";
</style>
