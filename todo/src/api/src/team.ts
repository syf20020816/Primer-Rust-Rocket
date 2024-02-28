import { Request } from '../axios/index'
import { User, Team } from '../../core'
import type { ApiResponse } from './type'

// 初始化Request类以使用axios实例
const request = Request.init()

/**
 * 创建团队
 * @param username 用户名
 * @param name 团队名
 * @returns 返回创建团队后的ApiResponse对象，包含User信息
 */
export const createTeam = async (username: string, name: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get(`/team/${username}/${name}`)
  return data
}

/**
 * 更新团队成员信息
 * @param membername 成员名
 * @param team 更新后的团队信息
 * @returns 返回一个布尔值，表示是否更新成功
 */
export const updateTeamMember = async (membername: string, team: Team): Promise<boolean> => {
  const { data } = await request.put(`/team/${membername}`, team)
  return data
}

/**
 * 更新团队信息
 * @param team 更新后的团队信息对象
 * @returns 返回一个布尔值，表示是否更新成功
 */
export const updateTeamInfo = async (team: Team): Promise<boolean> => {
  const { data } = await request.put(`/team`, team)
  return data
}

/**
 * 为团队创建待办事项
 * @param teamId 团队ID
 * @param todo 待办事项内容
 * @returns 返回一个布尔值，表示是否创建成功
 */
export const createTeamTodo = async (teamId: string, todo: any): Promise<boolean> => {
  const { data } = await request.post(`/team/todo/${teamId}`, todo)
  return data
}
