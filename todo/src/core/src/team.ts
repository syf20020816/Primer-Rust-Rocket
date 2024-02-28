import Team1SVG from '../../assets/team/team1.svg'
import Team2SVG from '../../assets/team/team2.svg'
import Team3SVG from '../../assets/team/team3.svg'
import Team4SVG from '../../assets/team/team4.svg'
import { Todo } from './todo'
import { User } from './user'

/**
 * 头像枚举
 */
export enum TeamAvatars {
  Team1 = 'Team1',
  Team2 = 'Team2',
  Team3 = 'Team3',
  Team4 = 'Team4'
}

/**
 * 头像枚举映射
 */
export const TeamMap = new Map<TeamAvatars, any>([
  [TeamAvatars.Team1, Team1SVG],
  [TeamAvatars.Team2, Team2SVG],
  [TeamAvatars.Team3, Team3SVG],
  [TeamAvatars.Team4, Team4SVG]
])

/**
 * 使用头像函数
 * 通过枚举得到映射结果
 * @param teamEnum TeamAvatars
 * @returns
 */
export const useTeam = (teamEnum: TeamAvatars) => {
  return TeamMap.get(teamEnum)
}

export type Team = {
  // 团队ID
  id: string
  // 团队名称
  name: string
  // 团队成员信息
  members: Array<User>
  // 团队持有人
  owner: string
  // 团队头像
  avatar: TeamAvatars
  // 团队概述和说明
  description: string
  // 团队创建日期
  date: Date
  // 团队所属TODO
  todos: Todo[]
}
