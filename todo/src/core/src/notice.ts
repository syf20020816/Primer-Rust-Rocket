// 导出通知类型
export type Notice = {
  // 通知标题
  title: string
  // 通知描述
  description: string
  // 通知者
  notifier: string
  // 通知类型
  type: Notices
  // 通知数据
  data?: any
  // 通知日期
  date: string
  // 通知id
  id: string
}

// 通知类型
export type Notices = 'todo' | 'normal'
