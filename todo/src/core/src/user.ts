import { Avatars } from './avatar'
import { TodoBox } from './todo'
import { Team } from './team'
import { Option } from './common'

// 用户登录表单类型定义，包含用户名和密码字段
type UserLoginForm = {
  username: string  // 用户名字段
  password: string  // 密码字段
}

// 定义用户信息类型，包含用户的多个基本信息和设置选项
type User = {
  username: string  // 用户名
  name: string  // 用户的真实姓名
  avatar: Avatars  // 用户的头像，类型来自'./avatar'
  email: string  // 用户的电子邮箱地址
  teamNumber: number  // 用户所属团队的数量
  todoNumber: number  // 用户当前待办事项的数量
  totalTodo: number  // 用户所有待办事项的总数
  todos: TodoBox  // 用户的待办事项箱，类型来自'./todo'
  teams: Option<Array<Team>>  // 用户所属的团队列表，使用泛型数组包装团队类型，并通过Option类型表示可能的可选状态
  sendEmail: boolean  // 用户是否同意接收电子邮件通知
  sendMsg: boolean  // 用户是否同意接收消息通知
}

// 用户信息变更表单类型定义，用于提交用户信息的更新请求
type UserInfoChangeForm = {
  name: string  // 用户的真实姓名
  email: string  // 用户的电子邮箱地址
  sendEmail: boolean  // 用户是否同意接收电子邮件通知
  sendMsg: boolean  // 用户是否同意接收消息通知
}

// 使用export type导出类型，使得这些类型能够在其他TypeScript文件中被导入和使用
export type { UserLoginForm, User, UserInfoChangeForm }
