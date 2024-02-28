/**
 * ==============================
 *            ReadMe
 * 1. 作为Api的出口文件
 * 2. 若使用此方式（axios-api模块化）请注释掉main.js中案例提供的默认$http的全局axios
 * ==============================
 */

// 导入各API模块
import { addNewTodo, deleteTodo, updateTodo, updateTodoStatus, completedTodo, failedTodo } from './src/todo'
import { signin, signup, getUserInfo, setUserInfo, setUserAvatar } from './src/user'
import { createTeam, updateTeamMember, updateTeamInfo, createTeamTodo } from './src/team'

// 将所有导入的函数按照其功能（用户、待办事项、团队）进行组织，并导出为一个对象
export default {
  // 用户相关API
  user: {
    signin,
    signup,
    getUserInfo,
    setUserInfo,
    setUserAvatar
  },
  // 待办事项相关API
  todo: {
    addNewTodo,
    deleteTodo,
    updateTodo,
    updateTodoStatus,
    completedTodo,
    failedTodo
  },
  // 团队相关API
  team: {
    createTeam,
    updateTeamMember,
    updateTeamInfo,
    createTeamTodo
  }
}
