<template>
  <div :id="buildView(component)">
    <!-- LOGO -->
    <div v-html="useSvg(SVGs.LOGO, 24)" :class="buildWrap(component, 'logo-wrap')"></div>
    <!-- APP 名字 -->
    <span class="title">MY <span class="clip">TODO</span></span>
    <!-- 搜索框 -->
    <div class="search-wrap">
      <!-- 搜索框点击搜索按钮后显示下拉框 -->
      <el-popover placement="bottom" :width="600" :visible="showSearch">
        <template #reference>
          <el-input
            v-if="userStore.isSignIn"
            v-model="searchValue"
            placeholder="Search TODOs"
            class="input-with-select"
          >
            <template #prepend>
              <div>TODO</div>
            </template>
            <template #append>
              <el-button :icon="Search" @click="onSearch" />
            </template>
          </el-input>
        </template>
        <!-- 具体下拉框内容 -->
        <div class="search-result-wrapper">
          <header class="header">
            <h4>Search Result:</h4>
            <el-icon @click="showSearch = false">
              <CircleClose />
            </el-icon>
          </header>
          <!-- 下拉框中心部分 -->
          <center class="center">
            <div
              v-for="item in searchResults"
              :key="item.name"
              class="search-item-wrapper"
            >
              <div class="name">
                <span><strong>TODO Name</strong>: {{ item.name }}</span>
              </div>
              <div class="item-des-wrapper">
                <p>TODO Description:{{ item.description }}</p>
                <div>TODO Date:{{ item.date.start }} ~ {{ item.date.end }}</div>
              </div>
            </div>
          </center>
        </div>
      </el-popover>
    </div>
    <!-- 右侧工具栏 -->
    <div :class="buildWrap(component, 'right-wrap')">
      <div v-if="userStore.isSignIn">
        <!-- 设置按钮 -->
        <div
          @click="openSetting"
          v-html="useSvg(SVGs.SETTING, 24)"
          :class="buildWrap(component, 'logo-wrap')"
        ></div>
        <!-- 带有勋章提示的通知按钮 -->
        <el-popover placement="bottom" :width="300" trigger="click">
          <template #reference>
            <div :class="buildWrap(component, 'logo-wrap')">
              <el-badge :value="userStore.msgBox.length" :max="99">
                <div v-html="useSvg(SVGs.NOTE, 24)"></div>
              </el-badge>
            </div>
          </template>
          <Notice v-for="item in userStore.msgBox" :key="item.id" :data="item"></Notice>
        </el-popover>
        <!-- 带有退出登录的下拉框头像按钮 -->
        <div :class="buildWrap(component, 'logo-wrap')">
          <el-popconfirm
            width="220"
            confirm-button-text="OK"
            cancel-button-text="Cancel"
            :icon="InfoFilled"
            icon-color="#626AEF"
            title="Logout Now?"
            @confirm="Logout"
          >
            <template #reference>
              <img
                :src="userAvatar"
                alt=""
                height="48"
                width="48"
                style="border-radius: 50%"
              />
            </template>
          </el-popconfirm>
        </div>
      </div>
    </div>
    <!-- 设置相关侧边弹出栏 -->
    <el-drawer
      ref="settingDrawerRef"
      v-model="settingDrawer"
      :before-close="handleClose"
      direction="rtl"
      class="setting-drawer"
      size="460px"
    >
      <template #header>
        <div class="setting-drawer-title">Personal Settings</div>
      </template>
      <div class="setting-drawer-content">
        <el-form
          label-position="left"
          ref="ruleFormRef"
          :model="ruleForm"
          :rules="rules"
          label-width="120px"
          class="demo-ruleForm"
          :size="formSize"
          status-icon
        >
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                The name is arbitrary, but it is recommended to use a real name
              </p>
            </el-alert>
            <el-form-item label="Name" prop="name">
              <el-input v-model="userStore.user.name" />
            </el-form-item>
          </el-space>
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                Email is a required field. Please ensure that the email you filled in is
                your work email and can be received normally
              </p>
            </el-alert>
            <el-form-item label="Email" prop="email">
              <el-input v-model="userStore.user.email" />
            </el-form-item>
          </el-space>
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                Enabling means that when you complete TODO, the system will send an email
                to the reviewer (not developed)
              </p>
            </el-alert>
            <el-form-item label="Send Email" prop="sendEmail">
              <el-switch v-model="userStore.user.sendEmail" disabled />
            </el-form-item>
          </el-space>
          <el-space fill>
            <el-alert type="info" show-icon :closable="false">
              <p class="alert-wrapper">
                Enabling means that when you complete TODO, the system will send a system
                message to the reviewer
              </p>
            </el-alert>
            <el-form-item label="Send Message" prop="sendMessage">
              <el-switch v-model="userStore.user.sendMsg" />
            </el-form-item>
          </el-space>
          <el-form-item>
            <el-button type="primary" @click="submitForm(ruleFormRef)">
              Save Settings
            </el-button>
            <el-button @click="resetForm(ruleFormRef)">Reset</el-button>
          </el-form-item>
        </el-form>
      </div>
    </el-drawer>
  </div>
</template>

<script lang="ts" setup>
import { Search } from '@element-plus/icons-vue'
import { ref, reactive, defineComponent, computed } from 'vue'
import { buildView, buildWrap, Todo, UserInfoChangeForm } from '../../core'
import { SVGs, useSvg, Notice } from '../index'
import { user } from '../../store/src/user'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'

import { InfoFilled, CircleClose } from '@element-plus/icons-vue'
import api from '../../api'

// 定义组件名称
const component = 'Header'
defineComponent({
  name: component
})

// 使用用户状态管理
const userStore = user()
// 搜索框的值
const searchValue = ref('')
// 设置抽屉的显示状态
const settingDrawer = ref(false)
// 是否显示搜索结果
const showSearch = ref(false)

// 使用计算属性获取用户头像
const userAvatar = computed(() => {
  return userStore.useAvatar(userStore.user.avatar)
})

// 搜索结果数组
const searchResults = ref<Todo[]>([])

// 处理搜索逻辑
const onSearch = () => {
  // 去除搜索字符串的前后空格
  const formatSearchValue = searchValue.value.trim()
  console.log(formatSearchValue)
  let { todos } = userStore
  // 过滤待办事项匹配搜索字符串
  searchResults.value = todos.filter(todo => todo.name.includes(formatSearchValue))
  showSearch.value = true;
}

// 打开设置抽屉
const openSetting = () => {
  settingDrawer.value = true
}

// 关闭设置抽屉
const handleClose = () => {
  settingDrawer.value = false
}

// 表单大小
const formSize = ref('default')
// 表单实例引用
const ruleFormRef = ref<FormInstance>()
// 表单数据模型
const ruleForm = reactive<UserInfoChangeForm>({
  name: userStore.user.name,
  email: userStore.user.email,
  sendEmail: userStore.user.sendEmail,
  sendMsg: userStore.user.sendMsg
})

// 表单校验规则
const rules = reactive<FormRules<UserInfoChangeForm>>({
  name: [
    { required: true, message: 'Please input your name', trigger: 'blur' },
    { min: 1, max: 16, message: 'Length should be 1 to 16', trigger: 'blur' }
  ],
  email: [
    {
      required: true,
      message: 'Please input email address',
      trigger: 'blur'
    },
    {
      type: 'email',
      message: 'Please input correct email address',
      trigger: ['blur', 'change']
    }
  ]
})

// 提交表单
const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate(async (valid, _fields) => {
    if (valid) {
      let username = userStore.getUsername()
      if (username) {
        let form = {
          name: userStore.user.name,
          email: userStore.user.email,
          sendEmail: userStore.user.sendEmail,
          sendMsg: userStore.user.sendMsg
        }
        // 调用 API 更新用户信息
        const data = await api.user.setUserInfo(username, form)
        if (typeof data !== 'undefined') {
          // 显示成功消息
          ElMessage({
            type: 'success',
            message: 'Configuration modification successful'
          })
          // 更新用户状态
          userStore.setUser(data)
        }
      }
    } else {
      // 显示错误消息
      ElMessage({
        type: 'error',
        message: 'Save failed, please try again'
      })
    }
  })
  // 关闭设置抽屉
  settingDrawer.value = false
}

// 重置表单
const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.resetFields()
}

// 登出操作
const Logout = () => {
  // 清理缓存和用户信息
  userStore.logout()
}
</script>

<style lang="scss" scoped>
@import "../../styles/components/header.scss";
</style>
