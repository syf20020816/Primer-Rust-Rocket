import { Status, Todo, User } from '../../core'
import { Request } from '../axios/index'
import { ApiResponse } from './type'

// 使用Request类的init方法初始化axios实例
const request = Request.init()

/**
 * 添加新的待办事项
 * @param todo 待添加的待办事项对象
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const addNewTodo = async (todo: Todo): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/todo/create', todo)
  return data
}

/**
 * 删除待办事项
 * @param username 用户名
 * @param id 待办事项的ID
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const deleteTodo = async (username: string, id: string): Promise<ApiResponse<User>> => {
  const { data } = await request.delete('/todo/' + username + '/' + id)
  return data
}

/**
 * 更新待办事项
 * @param username 用户名
 * @param id 待办事项的ID
 * @param todo 更新后的待办事项对象
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const updateTodo = async (username: string, id: string, todo: Todo): Promise<ApiResponse<User>> => {
  const { data } = await request.put('/todo/' + username + '/' + id, todo)
  return data
}

/**
 * 标记待办事项为失败
 * @param username 用户名
 * @param id 待办事项的ID
 * @param todo 包含失败状态的待办事项对象
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const failedTodo = async (username: string, id: string, todo: Todo): Promise<ApiResponse<User>> => {
  const { data } = await request.put('/todo/failed/' + username + '/' + id, todo)
  return data
}

/**
 * 更新待办事项的状态
 * @param id 待办事项的ID
 * @param status 新的状态值
 * @returns 返回一个布尔值，表示更新状态是否成功
 */
export const updateTodoStatus = async (id: String, status: Status): Promise<boolean> => {
  const { data } = await request.get('/todo/' + id + '/' + status)
  return data
}

/**
 * 标记待办事项为完成
 * @param username 用户名
 * @param todoId 待办事项的ID
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const completedTodo = async (username: string, todoId: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get('/todo/complete/' + username + '/' + todoId)
  return data
}
