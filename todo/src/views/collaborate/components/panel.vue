<template>
  <div id="team-panel-wrapper">
    <div class="panel-tool-wrapper">
      <el-button type="primary" @click="createNewTeam">Create Team</el-button>
      <el-button type="primary" @click="addMember" :disabled="addMemberDisabled"
        >Add Member</el-button
      >
      <el-button
        type="primary"
        @click="addTodoVisible = true"
        :disabled="addMemberDisabled"
      >
        Create Team TODO
      </el-button>
    </div>
    <div class="panel-todo-detail-wrapper" v-if="props.data?.todos?.length">
      <div class="left">
        <div
          v-for="item in props.data?.todos"
          :key="item.id"
          class="todo-wrapper"
          @click="currentTodo = item"
        >
          <span class="priority" :style="getPriorityDot(item)"></span>
          <span style="font-weight: 700"> {{ item.name }}</span>
        </div>
      </div>
      <div class="right">
        <TODOItem
          v-if="rule.todoRule === 1 || rule.todoRule === 11 || rule.todoRule === 10"
          :is-change="true"
          :is-compelete="rule.todoRule === 1 || rule.todoRule === 11"
          :current-todo="currentTodo"
          @change="changeTodo"
        ></TODOItem>
        <div v-else>
          <h4>The current TODO does not belong to you</h4>
        </div>
      </div>
    </div>
    <div v-else class="panel-todo-detail-wrapper" style="justify-content: center">
      <h4>This Team now has no TODOs</h4>
    </div>
    <div class="panel-team-detail-wrapper">
      <div v-if="isShowTeam">
        <header class="header">
          <img :src="useTeam(data!.avatar)" alt="" class="teamIcons" />
          <el-button type="info" @click="openSetTeam">Set Team</el-button>
        </header>
        <center class="center">
          <el-descriptions :title="data?.name" :column="2">
            <el-descriptions-item label="Team Owner" width="50%">{{
              data?.owner
            }}</el-descriptions-item>
            <el-descriptions-item label="Members">{{
              data?.members.length
            }}</el-descriptions-item>
            <el-descriptions-item label="Description" :span="2">{{
              data?.description
            }}</el-descriptions-item>
          </el-descriptions>
        </center>
      </div>
      <div v-else>
        <h4>Choose A Team Then You Will Get Infos</h4>
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
        <el-form-item label="Reviewers">
          <el-select
            style="width: 360px"
            multiple
            v-model="todoForm.reviewers"
            placeholder="choose reviewers"
          >
            <el-option
              v-for="item in props.data?.members"
              :label="item.name"
              :value="item.username"
              :key="item.username"
            >
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="Performers">
          <el-select
            style="width: 360px"
            multiple
            v-model="todoForm.performers"
            placeholder="choose performer"
          >
            <el-option
              v-for="item in props.data?.members"
              :label="item.name"
              :value="item.username"
              :key="item.username"
            >
            </el-option>
          </el-select>
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
        <el-button @click="setTeamVisible = false">Cancel</el-button>
        <el-button type="primary" @click="addNewTodo(todoFormRef)">
          {{ dialogBtn }}
        </el-button>
      </div>
    </template>
  </el-dialog>
  <el-dialog v-model="setTeamVisible" title="Set Team Info" width="600">
    <div class="panel-wrapper">
      <div class="panel-item-wrapper">
        <span class="paneltitle">Team Name</span>
        <el-input v-model="teamForm.name" />
      </div>
      <div class="panel-item-wrapper">
        <span class="paneltitle">Team Avatar</span>
        <div class="icons-wrapper">
          <div v-for="(item, index) in teamAvatarsOptions" :key="index">
            <img
              :src="useTeam(item.value)"
              alt=""
              class="panel-team-icon"
              :style="teamSelected(index)"
              @click="selectTeamAvatar(index)"
            />
          </div>
        </div>
      </div>
      <div class="panel-item-wrapper">
        <span class="paneltitle">Description</span>
        <el-input
          v-model="teamForm.description"
          maxlength="30"
          placeholder="Please input team description"
          show-word-limit
        />
      </div>
    </div>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="setTeamVisible = false">Cancel</el-button>
        <el-button type="primary" @click="changeTeam"> Change </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import { ref, reactive, computed, toRaw } from "vue";
import {
  Avatars,
  ITagProps,
  Priorities,
  Status,
  Team,
  TeamMap,
  Todo,
  convertFileToBase64,
  effectOptions,
  priorityOptions,
  typeOptions,
  useAvatar,
  useTeam,
  TeamAvatars,
  User,
usePriorityColor,
} from "../../../core";
import { CircleClose, QuestionFilled, CirclePlusFilled } from '@element-plus/icons-vue'
import { type UploadProps, type UploadUserFile, type UploadInstance, ElMessage, type UploadFile, type UploadFiles, FormRules, FormInstance } from 'element-plus'
import { user as userPinia } from "../../../store/src/user";
import api from "../../../api";
import { TODOItem } from "../../plan";

const props = defineProps<{
  data?: Team;
  addMemberDisabled: boolean;
}>();



const userStore = userPinia();
const emits = defineEmits(["create", "add", "changeTeam"]);
const todoFormRef = ref<FormInstance>()
const createNewTeam = () => {
  emits("create");
};
const addMember = () => {
  emits("add");
};
const isShowTeam = computed(() => {
  return typeof props?.data !== "undefined";
});
const currentTodo = ref<Todo>()
const setTeamVisible = ref(false);
const addTagVisible = ref(false)
const addTodoVisible = ref(false)
const fileList = ref<UploadUserFile[]>([])
const teamForm = reactive({
  name: "",
  avatar: TeamAvatars.Team1,
  description: ""
})
const rule = computed(()=>{
  let teamRule = "partner"
  if(props.data?.owner===userStore.user.username){
    teamRule = "owner"
  }
  // 0: 这个TODO和你无关
  // 1: 这个TODO你是执行者
  // 10: 这个TODO你是审核者
  // 11: 这个TODO你即使执行者也是审核者
  let todoRule = 0
  if(currentTodo.value?.performers.filter(x=>x.username===userStore.user.username).length!==0){
    todoRule+=1;
}
  if(currentTodo.value?.reviewers.filter(x=>x.username===userStore.user.username).length!==0){
    todoRule +=10;
  }
  return{
    teamRule,
    todoRule
  }
})

const addNewTodo = async (formEl: FormInstance | undefined) => {
  if (!formEl) return;
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      let todo = convertTodo();

      let reviewers = toRaw(todoForm.reviewers);
      let performers =toRaw( todoForm.performers);
      Object.assign(todo, {
        reviewers,
        performers
      })
      if (isChange.value) {
        Object.assign(todo, { owner: changeTodoItem.value.owner ?? "" });
        console.log(todo);
        const data = await api.todo.updateTodo(
          userStore.user.username,
          changeTodoItem.value.id,
          todo
        );
        if (typeof data !== "undefined") {
          ElMessage({
            type: "success",
            message: "Update Todo successfully",
          });
          userStore.setUser(data);
        }
      } else {
        let id= props.data!.id;
        const data = await api.team.createTeamTodo(id,todo);
        if (data) {
          ElMessage({
            type: "success",
            message: "Create new Todo successfully",
          });
          const user = await api.user.getUserInfo(userStore.user.username);
          userStore.setUser(user!);
        }
      }
    } else {
      console.log("error submit!", fields);
    }
  });
};
const todoTag = ref<ITagProps>({
  type: '',
  effect: 'dark',
  label: ''
})

interface TodoRuleForm {
  name: string;
  priority: Priorities;
  date: [Date, Date];
  tags: Array<ITagProps>;
  description: string;
  information: string;
  annexs: Array<{
    name: string;
    data: string;
  }>;
  isFocus: boolean;
  performers: string[];
  reviewers: string[];
}
const changeTodoItem = ref<{
  id: string
  owner: string
}>({
  id: '',
  owner: ''
})

const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low
  return `background-color : ${usePriorityColor(priority)}`
})


const removeTag = (tag: ITagProps) => {
  todoForm.tags = todoForm.tags.filter(item => item !== tag)
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

const dialogTitle = computed(() => {
  if (isChange.value) {
    return 'Change Todo'
  }
  return 'Add New Todo'
})


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

const removeUploadFile = (file: { name: string; data: string }) => {
  todoForm.annexs = todoForm.annexs.filter(f => f.name !== file.name)
}


const todoForm = reactive<TodoRuleForm>({
  name: "",
  priority: Priorities.Mid,
  date: [new Date(), new Date()],
  tags: [],
  description: "",
  information: "",
  reviewers: [],
  performers: [],
  annexs: [],
  isFocus: false,
});

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

const changeTodo = (todo: Todo) => {
  console.log(todo)
  todoForm.name = todo.name
  todoForm.description = todo.description ?? ''
  todoForm.tags = todo.tags
  todoForm.information = todo.information ?? ''
  todoForm.date = [new Date(todo.date.start), new Date(todo.date.end)]
  todoForm.isFocus = todo.isFocus
  todoForm.priority = todo.priority
  todoForm.annexs = todo.annexs ?? []
  todoForm.reviewers = todo.reviewers.map(x=>x.username)
  todoForm.performers = todo.performers.map(x=>x.username)
  addTodoVisible.value = true
  isChange.value = true
  changeTodoItem.value.id = todo.id!
  changeTodoItem.value.owner = todo.owner
}

const convertTodo = (): Todo => {
  let during = todoForm.date[1].getTime() - todoForm.date[0].getTime();
  let currentTime = new Date().getTime() - todoForm.date[0].getTime();
  let status = currentTime < 0 ? Status.NOT_START : Status.IN_PROGRESS;
  let { username } = userStore.user;

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
      during,
    },
    tags: toRaw(todoForm.tags),
    status,
    description: todoForm.description,
    information: todoForm.information,
    /// 附件
    annexs: toRaw(todoForm.annexs),
    isFocus: todoForm.isFocus,
  };
  todoForm.annexs = [];
  return todo;
};

const isChange = ref(false)

const dialogBtn = computed(() => {
  if (isChange.value) {
    return 'Change'
  }
  return 'Add'
})

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

const teamAvatarsOptions = ref([
  {
    value: TeamAvatars.Team1,
    select: false
  },
  {
    value: TeamAvatars.Team2,
    select: false
  },
  {
    value: TeamAvatars.Team3,
    select: false
  },
  {
    value: TeamAvatars.Team4,
    select: false
  }
])

const openSetTeam = () => {
  setTeamVisible.value = true
  teamForm.avatar = props.data!.avatar

  teamAvatarsOptions.value.map(option => {
    if (option.value === props.data!.avatar) {
      option.select = true;
    }
    return option
  })


  teamForm.name = props.data!.name
  teamForm.description = props.data!.description
}

const teamSelected = computed(() => (index: number) => {
  if (teamAvatarsOptions.value[index].select) {
    return "border: 1px solid #ff9c4b"
  }

  return "border: 1px solid transparent"
})

const selectTeamAvatar = (index: number) => {
  teamAvatarsOptions.value.map(option => {
    option.select = false;
    return option
  });
  teamAvatarsOptions.value[index].select = true
  teamForm.avatar = teamAvatarsOptions.value[index].value
}

const changeTeam = async () => {
  let team: Team = {
    id: props.data!.id,
    name: teamForm.name,
    members: props.data!.members,
    owner: props.data!.owner,
    avatar: teamForm.avatar,
    description: teamForm.description,
    date: props.data!.date,
    todos:props.data!.todos
  }
  const data = await api.team.updateTeamInfo(team);
  if (data) {
    ElMessage({
      type: "success",
      message: "Updated team info successfully"
    })
    emits('changeTeam')
  } else {
    ElMessage({
      type: "error",
      message: "Updated team info failed"
    })
  }
  setTeamVisible.value = false;
}
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;

#team-panel-wrapper {
  width: 100%;
  height: 100%;
  border-radius: 10px;
  overflow: scroll;
  display: flex;
  align-items: center;
  justify-content: center;
  border-right: 1px solid $bg-color-hover;
  flex-wrap: wrap;
  align-content: space-between;

  .panel-tool-wrapper {
    height: 60px;
    width: 100%;
    padding: 6px;
    box-sizing: border-box;
    margin: 12px 16px;
    background-color: $bg-color-menu;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }

  .panel-todo-detail-wrapper {
    height: calc(100% - 300px);
    width: 100%;
    background-color: $bg-color-hover;
    box-sizing: border-box;
    margin: 8px 16px;
    border-radius: 4px;
    padding: 6px;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    .left {
      scrollbar-width: thin;
      overflow-y: scroll;
      height: 100%;
      width: 240px;
      box-sizing: border-box;
      padding-right: 6px;
      border-right: 1px dashed $bg-color-light;
      .todo-wrapper {
        cursor: pointer;
        width: 100%;
        background-color: $bg-color-menu;
        height: 46px;
        display: flex;
        align-items: center;
        justify-content: flex-start;
        box-sizing: border-box;
        padding: 16px;
        border-radius: 4px;
        margin: 3px 0;
        .priority {
          display: inline-block;
          height: 12px;
          width: 12px;
          border-radius: 2px;
          background-color: #d02828;
          margin-right: 6px;
        }
      }
    }
    .right {
      height: 100%;
      width: calc(100% - 240px);
    }
  }

  .panel-team-detail-wrapper {
    height: 200px;
    width: 100%;
    margin: 8px 16px;
    background-color: $bg-color-hover;
    box-sizing: border-box;
    padding: 6px;

    .header {
      height: 72px;
      padding: 6px 0;
      box-sizing: border-box;
      display: flex;
      align-items: center;
      justify-content: space-between;

      .teamIcons {
        height: 60px;
        width: 60px;
        border-radius: 6px;
      }
    }

    .center {
      .el-descriptions {
        margin: 0;
        background-color: #3b3a39;
      }

      :deep(.el-descriptions__body) {
        background-color: #3b3a39 !important;
      }

      :deep(.el-descriptions__label) {
        color: $bg-color-light;
        font-weight: 700;
      }

      :deep(.el-descriptions__content) {
        color: $bg-color-light;
      }
    }
  }
}

.panel-wrapper {
  .panel-item-wrapper {
    height: 72px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .paneltitle {
      width: 120px;
      text-align: left;
    }

    .icons-wrapper {
      display: flex;
      align-items: center;
      justify-content: space-between;
      width: calc(100% - 120px);
    }

    .el-input {
      width: calc(100% - 120px);
    }
  }

  .panel-team-icon {
    cursor: pointer;
    height: 60px;
    border-radius: 6px;
    width: 60px;
    border: 1px solid transparent;
    transition: all 0.25s ease-in-out;

    &:hover {
      border: 1px solid $force-color;
    }
  }
}
</style>
