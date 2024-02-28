import { defineStore } from 'pinia'
import { Avatars, Notice, Status, Todo, useAvatar, User } from '../../core'
import api from '../../api'
import { ElMessage, ElNotification } from 'element-plus'

// 定义一个Pinia store，通常用于管理用户状态，如登录状态、用户信息等
export const user = defineStore('user', {
  // state定义了这个store的状态数据
  state: () => {
    return {
      user: {} as User, // 用户信息，使用TypeScript断言为User类型
      isSignIn: false, // 登录状态标志
      todoInfoList: [] as { label: string; value: number | string }[], // 待办事项信息列表，每项包含标签和值
      todos: [] as Todo[], // 用户的待办事项数组
      watcher: null as null | number, // 用于存放setInterval返回的计时器ID，用于待办事项监控
      msgBox: [] as Notice[] // 消息盒子，用于存放通知信息
    }
  },
  actions: {
    useAvatar, // 引入useAvatar操作，可能用于头像处理
    checkSetIsSignIn() {
      // 检查并设置用户的登录状态
      let flag = window.localStorage.getItem('todo-sign-in')
      // 如果localStorage中有标志，则设置为登录状态，否则设置为未登录
      this.isSignIn = Boolean(flag)
    },
    setUser(user: User) {
      // 设置用户信息，并更新localStorage，同时更新待办列表和启动待办监视器
      this.user = user
      window.localStorage.setItem('todo-user', JSON.stringify(this.user))
      this.updateTodoList()
      this.todoWatcher()
    },
    setSignIn() {
      // 设置用户为登录状态，并在localStorage中记录
      window.localStorage.setItem('todo-sign-in', this.user.username.toString())
      this.isSignIn = true
    },
    getUsername() {
      // 获取用户名，如果无法获取，则执行注销操作
      let username = this.user.username ?? window.localStorage.getItem('todo-sign-in')
      if (username) {
        return username
      } else {
        this.logout()
      }
    },
    logout() {
      // 注销操作，清除localStorage，重置状态
      window.localStorage.clear()
      this.isSignIn = false
      this.user = {} as User
      this.msgBox = []
    },
    updateTodoList() {
      // 更新待办事项列表，根据不同的优先级进行分类
      let { todos, todoNumber } = this.user
      let { low, mid, fatal, focus } = todos
      const today = new Date(new Date().toLocaleDateString()).getTime()
      // 检查待办事项是否为当天的
      const countIsToday = (todos: Todo[]): number => {
        return todos.filter(todo => {
          let start = new Date(todo.date.start)
          let startDate = new Date(start.toLocaleDateString()).getTime()
          return startDate == today
        }).length
      }
      // 计算各类待办事项的数量，并更新待办信息列表
      const totalToday = countIsToday(low) + countIsToday(mid) + countIsToday(fatal)

      this.todoInfoList = [
        {
          label: 'TODOs for today',
          value: totalToday
        },
        {
          label: 'Emergent TODOs',
          value: fatal.length + focus.length
        },
        {
          label: 'Normal TODOs',
          value: low.length + mid.length
        },
        {
          label: 'All TODOs',
          value: low.length + mid.length + fatal.length
        }
      ]

      this.todos = [...low, ...mid, ...fatal]
    },
    notifyTodoSys(todo: Todo) {
      // 为待办事项发送通知，如果它还没有被通知过
      let id = 'todo-' + todo.id!
      let flag = this.msgBox.filter(item => item.id === id).length === 0
      if (flag) {
        // 如果该待办事项还没有通知，则添加到消息盒子
        this.msgBox.push({
          title: 'Notifications: Todo-' + todo.name,
          description: 'The current TODO has timed out, please choose a processing strategy',
          notifier: 'System',
          type: 'todo',
          data: todo,
          date: new Date().toLocaleString(),
          id
        })
      }
    },
    todoWatcher() {
      // 监视待办事项的状态，定时检查并更新状态
      if (this.watcher !== null) {
        clearInterval(this.watcher) // 如果已经有计时器在运行，则先清除
      }
      this.watcher = setInterval(() => {
        let current = new Date().getTime()
        this.todos.forEach(async (todo: Todo) => {
          let { date, status, id } = todo
          // 对于非完成、非挂起、非失败的待办事项，检查其状态并进行更新
          if (status !== Status.COMPLETED && status !== Status.PENDING && status !== Status.FAILED) {
            let start = current - new Date(date.start).getTime()
            if (start > 0 && status !== Status.IN_PROGRESS) {
              // 如果待办事项已开始但状态未更新，则更新状态
              todo.status = Status.IN_PROGRESS
              const data = await api.todo.updateTodoStatus(id!, todo.status)
              if (data) {
                // 状态更新成功后，重新获取用户信息并更新
                const user = await api.user.getUserInfo(this.user.username)
                this.setUser(user!)
              } else {
                ElMessage({
                  type: 'error',
                  message: 'User Update Error!'
                })
              }
            }

            let remain = new Date(date.end).getTime() - current
            if (remain < 0) {
              // 如果待办事项已结束，通知用户处理
              this.notifyTodoSys(todo)
            }
          }
        })
      }, 15 * 1000) // 每15秒检查一次
    }
  }
})
