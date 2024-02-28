<template>
  <div id="create_preview">
    <div class="create_wrapper">
      <div class="date_wrapper">
        <span>Today is {{ date.toLocaleDateString() }}</span>
        <span>{{ week[date.getDay()] }}</span>
      </div>
      <div class="info_wrapper">
        <div
          class="info_item"
          v-for="(item, index) in userStore.todoInfoList"
          :key="index"
        >
          <div class="title">{{ item.label }}</div>
          <div class="value">{{ item.value }}</div>
        </div>
      </div>
      <div class="work_wrapper">
        <el-button type="primary" @click="openAddDialog">Add New TODO</el-button>
        <!-- <el-button type="danger">Remove All TODOs</el-button>
        <el-button type="success">Complete All TODOs</el-button> -->
      </div>
      <div class="todo_wrapper">
        <TODOItem
          :is-compelete="rule !== 10"
          v-if="isShow"
          :is-change="true"
          :current-todo="currentTodo"
          @change="changeTodo"
          @delete="deleteTodo"
          @refresh="refreshTodo"
        ></TODOItem>
      </div>
    </div>
    <div class="preview_wrapper">
      <div class="todo-item" v-for="(item, index) in datas" :key="index">
        <div class="left">
          <span class="dot_wrapper">
            <span class="priority" :style="getPriorityDot(item)"></span>
            <span
              class="priority"
              style="border-radius: 50%"
              :style="getStatusDot(item)"
            ></span>
          </span>
          <div class="todo-name">{{ item.name }}</div>
        </div>
        <div class="right">
          <el-icon size="18px" class="icons" @click="showTodoDetails(item)"
            ><QuestionFilled
          /></el-icon>
        </div>
      </div>
    </div>
  </div>
  <el-dialog v-model="addTodoVisible" :title="dialogTitle" width="680">
    <div>
      <el-form
        ref="todoFormRef"
        :rules="rules"
        label-position="left"
        label-width="120px"
        :model="todoForm"
        style="max-width: 600px"
        status-icon
      >
        <el-form-item label="Todo Name" prop="name">
          <el-input v-model="todoForm.name" />
        </el-form-item>
        <el-form-item label="Priority">
          <el-select
            v-model="todoForm.priority"
            placeholder="Select Priority"
            style="width: 240px"
          >
            <el-option
              v-for="item in priorityOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="Tags">
          <el-popover
            placement="bottom"
            title="Add Tag"
            :width="360"
            trigger="click"
            :visible="addTagVisible"
          >
            <div class="tag-popover-wrapper">
              <el-input
                v-model="todoTag.label"
                placeholder="input tag name"
                clearable
              ></el-input>
              <el-select v-model="todoTag.effect" placeholder="choose effect">
                <el-option
                  v-for="item in effectOptions"
                  :label="item.label"
                  :value="item.value"
                  :key="item.value"
                >
                </el-option>
              </el-select>
              <el-select v-model="todoTag.type" placeholder="choose type">
                <el-option
                  v-for="item in typeOptions"
                  :label="item.label"
                  :value="item.value"
                  :key="item.value"
                >
                </el-option>
              </el-select>
              <el-button type="primary" @click="addTag">Add</el-button>
            </div>
            <template #reference>
              <el-button
                type="primary"
                :icon="CirclePlusFilled"
                @click="addTagVisible = !addTagVisible"
              >
              </el-button>
            </template>
          </el-popover>
          <div class="tags-wrapper">
            <el-tag
              :effect="item.effect"
              round
              closable
              @close="removeTag(item)"
              v-for="item in todoForm.tags"
              :key="item"
              :type="item.type"
              >{{ item.label }}</el-tag
            >
          </div>
        </el-form-item>
        <el-form-item label="Description">
          <el-input
            v-model="todoForm.description"
            type="text"
            maxlength="30"
            show-word-limit
            placeholder="Please input"
          />
        </el-form-item>
        <el-form-item label="Information">
          <el-input
            v-model="todoForm.information"
            :rows="2"
            type="textarea"
            placeholder="Please input"
          />
        </el-form-item>
        <el-form-item label="Date" prop="date">
          <el-date-picker
            v-model="todoForm.date"
            type="datetimerange"
            :shortcuts="shortcuts"
            range-separator="To"
            start-placeholder="Start date"
            end-placeholder="End date"
            @visible-change="checkDate"
          />
        </el-form-item>
        <el-form-item label="Focus?">
          <el-switch v-model="todoForm.isFocus" />
        </el-form-item>
        <el-form-item label="Annex?">
          <el-upload
            style="width: 100%"
            ref="uploadRef"
            :auto-upload="false"
            v-model:file-list="fileList"
            class="upload-demo"
            :multiple="true"
            @change="uploadAndConvertBase64"
            action="https://127.0.0.1:10016/api/v1/file"
          >
            <el-button type="primary" :icon="CirclePlusFilled"
              >Click to choose file</el-button
            >
            <template #tip>
              <div class="el-upload__tip">file size 10MB</div>
              <div v-if="isChange">
                <div>Annexs have been uploaded</div>
                <el-tag
                  @close="removeUploadFile(item)"
                  closable
                  effect="plain"
                  type="info"
                  v-for="item in todoForm.annexs"
                  :key="item.name"
                  >{{ item.name }}</el-tag
                >
              </div>
            </template>
          </el-upload>
        </el-form-item>
      </el-form>
    </div>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="addTodoVisible = false">Cancel</el-button>
        <el-button type="primary" @click="addNewTodo(todoFormRef)">
          {{ dialogBtn }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import { ref, reactive, computed, toRaw } from 'vue'
// 导入Todo项定义
import { TODOItem } from '../index'
// 导入Element Plus图标
import { CircleClose, QuestionFilled, CirclePlusFilled } from '@element-plus/icons-vue'
// 导入业务逻辑、工具函数和类型定义
import { Todo, Priorities, Avatars, Status, usePriorityColor, useAvatar, useStatus, convertFileToBase64, ITagProps, priorityOptions, effectOptions, typeOptions, Annex } from '../../../core'
// 导入Element Plus组件类型定义
import { type UploadProps, type UploadUserFile, type UploadInstance, ElMessage, type UploadFile, type UploadFiles, FormRules, FormInstance } from 'element-plus'
// 导入API服务
import api from '../../../api'
// 导入用户状态管理
import { user as userPinia } from '../../../store/src/user'

// 定义接收的props类型
const props = defineProps<{
  datas: Todo[]
}>()
// 使用用户状态管理
const userStore = userPinia()
// 当前日期
const date = new Date()
// 控制添加Todo对话框的显示状态
const addTodoVisible = ref(false)
// Todo表单的引用
const todoFormRef = ref<FormInstance>()
// 控制添加标签对话框的显示状态
const addTagVisible = ref(false)
// 星期数组
const week = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
// 上传组件的引用
const uploadRef = ref<UploadInstance>()
// 标记Todo是否在变更中
const isChange = ref(false)
// Todo标签数据
const todoTag = ref<ITagProps>({
  type: '',
  effect: 'dark',
  label: ''
})
// 变更的Todo项
const changeTodoItem = ref<{
  id: string
  owner: string
}>({
  id: '',
  owner: ''
})
// 文件列表
const fileList = ref<UploadUserFile[]>([])

// 当前操作的Todo
const currentTodo = ref<any>()
// 计算属性，判断当前Todo是否显示详情
const isShow = computed(() => {
  if (typeof currentTodo.value === 'undefined') {
    return false
  }
  const empty = Object.keys(currentTodo.value).length === 0 && currentTodo.value.constructor === Object
  return !empty
})
// 获取优先级对应的点的样式
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low
  return `background-color : ${usePriorityColor(priority)}`
})
// 获取状态对应的点的样式
const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START
  return `background-color : ${useStatus(status)}`
})
// 对话框标题，根据是添加还是修改Todo变化
const dialogTitle = computed(() => {
  if (isChange.value) {
    return 'Change Todo'
  }
  return 'Add New Todo'
})

// 对话框按钮文本，根据是添加还是修改Todo变化
const dialogBtn = computed(() => {
  if (isChange.value) {
    return 'Change'
  }
  return 'Add'
})


const showTodoDetails = (item: Todo) => {
  currentTodo.value = item
}

// 计算当前用户对Todo的权限
const rule = computed(()=>{
  let todoRule = 0;
  if(currentTodo.value?.performers.filter((x:any)=>x.username===userStore.user.username).length!==0){
    todoRule+=1;
}
  if(currentTodo.value?.reviewers.filter((x:any)=>x.username===userStore.user.username).length!==0){
    todoRule +=10;
  }
  console.log(todoRule)
  return todoRule
})

// Todo表单数据类型定义
interface TodoRuleForm {
  name: string
  priority: Priorities
  date: [Date, Date]
  tags: Array<ITagProps>
  description: string
  information: string
  annexs: Array<{
    name: string
    data: string
  }>
  isFocus: boolean
}

const todoForm = reactive<TodoRuleForm>({
  name: '',
  priority: Priorities.Mid,
  date: [new Date(), new Date()],
  tags: [],
  description: '',
  information: '',
  annexs: [],
  isFocus: false
})

// 表单验证规则
const rules = reactive<FormRules<TodoRuleForm>>({
  name: [
    { required: true, message: 'Please input Todo name', trigger: 'blur' },
    { min: 1, max: 16, message: 'Length should be 1 to 16', trigger: 'blur' }
  ],
  date: [
    {
      required: true,
      message: 'Please set start and end time',
      trigger: 'blur visible-change'
    }
  ]
})

// 检查日期是否设置
const checkDate = () => {
  let { date } = todoForm
  if (!date) {
    todoForm.date = [new Date(), new Date()]
    ElMessage({
      type: 'warning',
      message: 'Please set start and end time'
    })
  }
}

// 将表单数据转换为Todo对象
const convertTodo = (): Todo => {
  let during = todoForm.date[1].getTime() - todoForm.date[0].getTime()
  let currentTime = new Date().getTime() - todoForm.date[0].getTime()
  let status = currentTime < 0 ? Status.NOT_START : Status.IN_PROGRESS
  let { username } = userStore.user
  let todo: Todo = {
    owner: username,
    name: todoForm.name,
    priority: todoForm.priority,
    /// 审核人
    reviewers: [],
    performers: [],
    date: {
      start: todoForm.date[0].toLocaleString(),
      end: todoForm.date[1].toLocaleString(),
      during
    },
    tags: toRaw(todoForm.tags),
    status,
    description: todoForm.description,
    information: todoForm.information,
    /// 附件
    annexs: toRaw(todoForm.annexs),
    isFocus: todoForm.isFocus
  }
  todoForm.annexs = []
  return todo
}
// 添加新的Todo
const addNewTodo = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      let todo = convertTodo()
      if (isChange.value) {
        Object.assign(todo, { owner: changeTodoItem.value.owner ?? '' })
        console.log(todo)
        const data = await api.todo.updateTodo(userStore.user.username, changeTodoItem.value.id, todo)
        if (typeof data !== 'undefined') {
          ElMessage({
            type: 'success',
            message: 'Update Todo successfully'
          })
          userStore.setUser(data)
        }
      } else {
        const data = await api.todo.addNewTodo(todo)
        if (typeof data !== 'undefined') {
          ElMessage({
            type: 'success',
            message: 'Create new Todo successfully'
          })
          userStore.setUser(data)
        }
      }
    } else {
      console.log('error submit!', fields)
    }
  })
}

const uploadAndConvertBase64 = (uploadFile: UploadFile, _uploadFiles: UploadFiles) => {
  let file = uploadFile.raw
  if (typeof file !== 'undefined') {
    convertFileToBase64(file).then(base64 => {
      todoForm.annexs.push({
        name: uploadFile.name,
        data: base64
      })
    })
  }
}
// 打开添加Todo对话框
const openAddDialog = () => {
  addTodoVisible.value = true
  isChange.value = false
}

const addTag = () => {
  let { tags } = todoForm

  if (typeof tags === 'undefined') {
    tags = new Array()
  }
  if (todoTag.value.label === '') {
    ElMessage({
      type: 'warning',
      message: 'you should add tag name'
    })
    return
  }
  for (let tag of tags) {
    let tagStr = JSON.stringify(toRaw(tag))
    let newTagStr = JSON.stringify(toRaw(todoTag.value))
    if (tagStr === newTagStr) {
      ElMessage({
        type: 'warning',
        message: 'you already have the same tag'
      })
      return
    } else {
      continue
    }
  }

  tags.push(toRaw(todoTag.value))
  todoForm.tags = tags
  todoTag.value = {
    type: '',
    effect: 'dark',
    label: ''
  } as ITagProps
}

// 自定义日期选择器选项
const shortcuts = [
  {
    text: 'Next day',
    value: () => {
      const end = new Date()
      const start = new Date()
      end.setTime(start.getTime() + 3600 * 1000 * 24 * 1)
      return [start, end]
    }
  },
  {
    text: 'Next week',
    value: () => {
      const end = new Date()
      const start = new Date()
      end.setTime(start.getTime() + 3600 * 1000 * 24 * 7)
      return [start, end]
    }
  },
  {
    text: 'Next month',
    value: () => {
      const end = new Date()
      const start = new Date()
      end.setTime(start.getTime() + 3600 * 1000 * 24 * 30)
      return [start, end]
    }
  }
];

// 移除Tag
const removeTag = (tag: ITagProps) => {
  todoForm.tags = todoForm.tags.filter(item => item !== tag)
}

// 修改TODO
const changeTodo = (todo: Todo) => {
  todoForm.name = todo.name
  todoForm.description = todo.description ?? ''
  todoForm.tags = todo.tags
  todoForm.information = todo.information ?? ''
  todoForm.date = [new Date(todo.date.start), new Date(todo.date.end)]
  todoForm.isFocus = todo.isFocus
  todoForm.priority = todo.priority
  todoForm.annexs = todo.annexs ?? []
  addTodoVisible.value = true
  isChange.value = true
  changeTodoItem.value.id = todo.id!
  changeTodoItem.value.owner = todo.owner
}

const deleteTodo = () => {
  currentTodo.value = {}
}

const removeUploadFile = (file: { name: string; data: string }) => {
  todoForm.annexs = todoForm.annexs.filter(f => f.name !== file.name)
}

const refreshTodo = (_id: string) => {
  currentTodo.value = {}
}
</script>

<style lang="scss" scoped>
@use '../../../styles/src/var.scss' as *;
#create_preview {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;

  .create_wrapper {
    width: 65%;
    .date_wrapper {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: flex-start;
      span {
        margin: 0 6px;
        font-weight: 700;
      }
    }
    .info_wrapper {
      box-sizing: border-box;
      padding: 6px;
      border-top: 1px solid $bg-color-hover;
      width: 100%;
      height: fit-content;
      border-bottom: 1px solid $bg-color-hover;
      .info_item {
        font-size: 14px;
        height: 2em;
        display: flex;
        align-items: center;
        justify-content: space-between;
        box-sizing: border-box;
        padding: 0 6px;
        transition: all 0.25s ease-in-out;
        cursor: pointer;
        &:hover {
          background-color: $bg-color-hover;
          color: $force-color;
        }
      }
    }
    .work_wrapper {
      box-sizing: border-box;
      padding: 6px;
      display: flex;
      align-items: center;
      justify-content: flex-end;
    }
    .todo_wrapper {
      height: 432px;
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      box-sizing: border-box;
      padding: 12px;
    }
  }
  .preview_wrapper {
    height: 100%;
    width: 35%;
    box-sizing: border-box;
    border-left: 1px dashed $bg-color-hover;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    flex-wrap: wrap;
    align-content: flex-start;
    overflow-y: scroll;
    scrollbar-width: thin;
    .todo-item {
      margin-bottom: 6px;
      height: 60px;
      width: 96%;
      background-color: $bg-color-hover;
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: flex-start;
      box-sizing: border-box;
      padding: 12px;
      .left {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        width: calc(100% - 24px);
        .dot_wrapper {
          display: inline-block;
          height: 1.5em;
          display: flex;
          align-items: center;
          justify-content: center;
          margin-right: 12px;
          .priority {
            display: inline-block;
            height: 12px;
            width: 12px;
            border-radius: 2px;
            margin: 0 4px;
          }
        }
        .todo-name {
          font-size: 18px;
          width: calc(100% - 24px);
          font-weight: 700;
          text-align: left;
        }
      }
      .right {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 24px;
        .icons {
          cursor: pointer;
        }
      }
    }
  }
}
.tag-popover-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  .el-select {
    width: 100%;
    margin: 6px 0;
  }
}
.tags-wrapper {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  flex-wrap: wrap;
  align-content: flex-start;
  .el-tag {
    margin: 6px;
  }
}
</style>
