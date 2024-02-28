import Avatar1Png from '../../assets/avatar/avatar1.png'
import Avatar2Png from '../../assets/avatar/avatar2.png'
import Avatar3Png from '../../assets/avatar/avatar3.png'
/**
 * 头像枚举
 */
export enum Avatars {
  Worker = 'Worker',
  Miner = 'Miner',
  Adventurer = 'Adventurer'
}

/**
 * 头像枚举映射
 */
export const AvatarMap = new Map<Avatars, any>([
  [Avatars.Worker, Avatar1Png],
  [Avatars.Miner, Avatar2Png],
  [Avatars.Adventurer, Avatar3Png]
])

/**
 * 使用头像函数
 * 通过枚举得到映射结果
 * @param avatarEnum
 * @returns
 */
export const useAvatar = (avatarEnum: Avatars) => {
  return AvatarMap.get(avatarEnum)
}
