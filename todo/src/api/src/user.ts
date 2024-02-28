import { Request } from '../axios/index'
import { User, UserLoginForm, UserInfoChangeForm, Avatars } from '../../core'
import type { ApiResponse } from './type'

// 使用Request类的init方法初始化axios实例
const request = Request.init()

/**
 * 用户登录
 * @param params 登录表单数据，包括用户名和密码
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const signin = async (params: UserLoginForm): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/user/signin/', params)
  return data
}

/**
 * 用户注册
 * @param params 注册表单数据，包括用户名和密码
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const signup = async (params: UserLoginForm): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/user/signup/', params)
  return data
}

/**
 * 获取用户信息
 * @param username 用户名
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const getUserInfo = async (username: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get('/user/info/' + username)
  return data
}

/**
 * 设置用户信息
 * @param username 用户名
 * @param params 用户信息变更表单，可能包括用户的个人信息更改
 * @returns 返回包含用户信息的ApiResponse对象
 */
export const setUserInfo = async (username: string, params: UserInfoChangeForm): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/user/info/' + username, params)
  return data
}

/**
 * 设置用户头像
 * @param username 用户名
 * @param avatar 头像类型
 * @returns 返回一个布尔值，表示设置头像是否成功
 */
export const setUserAvatar = async (username: string, avatar: Avatars): Promise<boolean> => {
  const { data } = await request.get('/user/info/' + username + '/' + avatar)
  return data
}
