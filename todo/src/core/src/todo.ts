import { TagProps } from 'element-plus'
import { Avatars } from './avatar'
import { Option } from './common'
import { Annex } from './annex'
import { User } from './user'

/** 优先级等级枚举，用于定义任务或项目的紧急程度 */
export enum Priorities {
  Emergent = 'Emergent', // 紧急
  High = 'High',         // 高
  Mid = 'Mid',           // 中
  Low = 'Low'            // 低
}

/** 优先级选项数组，用于前端选择组件 */
export const priorityOptions = [
  {
    value: Priorities.Low,    // 低优先级
    label: Priorities.Low     // 显示标签为"Low"
  },
  {
    value: Priorities.Mid,    // 中优先级
    label: Priorities.Mid     // 显示标签为"Mid"
  },
  {
    value: Priorities.High,   // 高优先级
    label: Priorities.High    // 显示标签为"High"
  },
  {
    value: Priorities.Emergent, // 紧急优先级
    label: Priorities.Emergent  // 显示标签为"Emergent"
  }
]

/** 优先级类型定义，包含颜色和名称 */
export type Priority = {
  color: string         // 优先级颜色
  name: Priorities      // 优先级名称
}

/** 优先级的颜色映射器，用于根据优先级获取对应的颜色 */
const PriorityColorMap = new Map<Priorities, string>([
  [Priorities.Emergent, '#E86D5E'], // 紧急
  [Priorities.High, '#F69D50'],     // 高
  [Priorities.Mid, '#6CB6FF'],      // 中
  [Priorities.Low, '#ADAC9A']       // 低
])

/** 获取优先级颜色的函数 */
export const usePriorityColor = (priority: Priorities): string => {
  return PriorityColorMap.get(priority) || '#ADAC9A'
}

/** 日期类型定义，用于描述任务或事件的开始和结束时间 */
export type IDate = {
  start: string  // 开始时间
  end: string    // 结束时间
  during: number // 持续时间，以某种计量单位表示
}

/** 待办状态枚举，描述任务的当前状态 */
export enum Status {
  NOT_START = 'not start',   // 未开始
  IN_PROGRESS = 'in progress', // 进行中
  COMPLETED = 'completed',   // 已完成
  PENDING = 'pending',       // 阻塞中
  FAILED = 'failed'          // 失败
}

/** 状态和颜色的映射，用于根据任务状态获取对应的颜色 */
const StatusTypeMap = new Map<Status, string>([
  [Status.NOT_START, '#ADAC9A'],     // 未开始
  [Status.IN_PROGRESS, '#56D4DD'],   // 进行中
  [Status.COMPLETED, '#8DDB80'],     // 已完成
  [Status.PENDING, '#8EBAC7'],       // 阻塞中
  [Status.FAILED, '#FF5555']         // 失败
])

/** 获取任务状态颜色的函数 */
export const useStatus = (status: Status): string => {
  return StatusTypeMap.get(status) || '#ADAC9A'
}

/** 标签属性类型，用于定义任务或项目标签的视觉表现 */
export type ITagProps = {
  type: 'info' | 'success' | 'warning' | '' | 'danger' // 标签类型
  effect: 'dark' | 'light' | 'plain'                    // 标签效果
  label: string                                         // 标签文本
}

/** 标签效果选项数组，用于前端选择组件 */
export const effectOptions = [
  // 定义不同的标签效果选项
  { label: 'dark', value: 'dark' },
  { label: 'light', value: 'light' },
  { label: 'plain', value: 'plain' }
]

/** 标签类型选项数组，用于前端选择组件 */
export const typeOptions = [
  // 定义不同的标签类型选项
  { label: 'default', value: '' },
  { label: 'info', value: 'info' },
  { label: 'success', value: 'success' },
  { label: 'warning', value: 'warning' },
  { label: 'danger', value: 'danger' }
]

/** 待办事项类型定义，描述一个待办事项的各种属性 */
export type Todo = {
  id?: string                      // 待办事项的唯一标识
  owner: string                    // 待办事项的所有者
  name: string                     // 待办事项的名称
  priority: Priorities             // 待办事项的优先级
  reviewers: Array<User>           // 审核人列表
  performers: Array<User>          // 执行人列表
  date: IDate                      // 待办事项的日期
  tags: Array<ITagProps>           // 待办事项的标签列表
  status: Status                   // 待办事项的状态
  description: Option<string>      // 待办事项的描述
  information: Option<string>      // 待办事项的额外信息
  annexs: Option<Array<Annex>>     // 附件列表
  isFocus: boolean                 // 是否为关注的待办事项
}

/** 待办事项箱类型定义，用于分类存放不同优先级和状态的待办事项 */
export type TodoBox = {
  low: Array<Todo>     // 低优先级的待办事项列表
  mid: Array<Todo>     // 中等优先级的待办事项列表
  fatal: Array<Todo>   // 高优先级和紧急优先级的待办事项列表
  focus: Array<Todo>   // 关注的待办事项列表
  history: Array<Todo> // 历史待办事项列表
}
