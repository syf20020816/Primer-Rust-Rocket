import { Todo } from './todo'

// 将对象转换为字符串
const toStore = (target: any): string => {
  return JSON.stringify(target)
}

// 初始化本地存储
const initLocalStorage = () => {
  // init local storage
  // - todo-sigin-in : false
  // - todo-user: {}
  window.localStorage.setItem('todo-sign-in', toStore(false))
  window.localStorage.setItem('todo-user', toStore(new Object()))
}

// 导出初始化函数
export const init = () => {
  initLocalStorage()
}
