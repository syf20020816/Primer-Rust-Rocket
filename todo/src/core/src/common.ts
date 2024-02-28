export type Option<T> = T | null

export const convertFileToBase64 = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    // 创建 FileReader 对象
    const reader = new FileReader()

    // 读取操作完成（无论成功或失败）后触发的事件处理器
    reader.onload = () => {
      // 读取成功，reader.result 包含转换为 base64 的文件内容
      resolve(reader.result as string)
    }

    // 读取操作发生错误时触发的事件处理器
    reader.onerror = error => {
      // 读取失败，拒绝 Promise
      reject(error)
    }

    // 使用 readAsDataURL 方法读取文件内容，结果为数据URL（base64 编码）
    reader.readAsDataURL(file)
  })
}

/**
 * 将Base64编码的字符串转换为Blob对象
 * @param base64Data Base64编码的字符串
 * @param contentType 文件的MIME类型，默认为 'application/octet-stream'
 */
export const base64ToBlob = (base64Data: string, contentType: string = 'application/octet-stream'): Blob => {
  // Base64字符串中实际的数据部分通常会以逗号分隔，之前的部分包含了一些描述信息
  const base64ContentArray = base64Data.split(',')
  // 解码Base64字符串
  const byteString = atob(base64ContentArray[1])
  // 创建一个用于存储解码后二进制数据的数组
  const byteArray = new Uint8Array(byteString.length)
  for (let i = 0; i < byteString.length; i++) {
    byteArray[i] = byteString.charCodeAt(i)
  }
  // 使用解码后的数据创建Blob对象
  const blob = new Blob([byteArray], { type: contentType })
  return blob
}

/**
 * 触发浏览器下载文件
 * @param blob 文件内容的Blob对象
 * @param fileName 下载文件时使用的文件名
 */
export const downloadBlob = (blob: Blob, fileName: string): void => {
  // 为Blob对象创建一个临时URL
  const url = URL.createObjectURL(blob)
  // 创建一个隐藏的<a>元素，并设置其href为Blob对象的URL
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = fileName
  document.body.appendChild(anchor)
  // 模拟点击<a>元素以触发下载
  anchor.click()
  // 移除<a>元素
  document.body.removeChild(anchor)
  // 释放之前创建的临时URL
  URL.revokeObjectURL(url)
}
