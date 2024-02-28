/**
 * 引入axios库及相关类型，以及element-plus的消息提示组件
 */
import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage } from 'element-plus'

/**
 * 定义Request类，用于封装axios实例及其配置方法
 */
export class Request {
  // axiosInstance作为Request类的静态属性，用于存储axios实例
  public static axiosInstance: AxiosInstance

  /**
   * init方法用于初始化axios实例
   */
  public static init() {
    // 使用axios.create创建新的axios实例，并配置基本属性
    this.axiosInstance = axios.create({
      baseURL: 'http://localhost:10016/api/v1', // API基础路径
      timeout: 360000, // 请求超时时间
      headers: {
        // 默认请求头配置
        'Access-Control-Allow-Origin': '*', // 允许所有源的跨域请求
        'Access-Control-Allow-Credentials': 'true' // 允许携带凭证
      }
    })
    this.initInterceptors() // 初始化拦截器
    return this.axiosInstance // 返回配置好的axios实例
  }

  /**
   * initInterceptors方法用于初始化请求和响应拦截器
   */
  public static initInterceptors() {
    // 请求拦截器
    this.axiosInstance.interceptors.request.use(
      config => {
        // 从localStorage中获取token
        const token = window.localStorage.getItem('todo-token')
        // 如果token存在，则将token添加到请求头中
        if (token !== null && config?.headers) {
          config.headers.token = token
        }
        return config // 返回配置好的请求配置
      },
      (error: any) => {
        // 请求配置错误处理
        console.log(error)
      }
    )

    // 响应拦截器
    this.axiosInstance.interceptors.response.use(
      (response: AxiosResponse) => {
        // 如果响应状态码为200
        if (response.status === 200) {
          // 并且响应体中的code为200，则返回响应数据
          if (response.data['code'] === 200) {
            return response.data
          } else {
            // 如果code不为200，则调用错误处理函数
            this.errorHandle(response.data.msg)
            return undefined
          }
        } else {
          // 如果响应状态码不为200，也调用错误处理函数
          this.errorHandle(JSON.stringify(response))
          return null
        }
      },
      (error: any) => {
        // 响应错误处理
        this.errorHandle(error)
      }
    )
  }

  /**
   * 错误处理函数，使用element-plus的消息提示组件进行错误提示
   */
  public static errorHandle(res: string) {
    ElMessage({
      type: 'error', // 消息类型为错误
      message: res // 显示错误信息
    })
  }
}
