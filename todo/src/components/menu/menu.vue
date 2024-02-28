<!--language: Vue-->
//对Menu菜单项的封装
<template>
  <div :id="buildView(component)">
    <!-- 使用El-Menu -->
    <el-menu
      active-text-color="#ff9c4b"
      background-color="#252423"
      class="el-menu-vertical-demo"
      default-active="0"
      text-color="#fff"
      @select="selectMenu"
    >
      <!-- 循环生成每个menu项 -->
      <el-menu-item :index="item.index" v-for="item in menuList" :key="item.index">
        <el-tooltip :content="item.label" placement="right">
          <component :is="getMenuIcon(item)"></component>
        </el-tooltip>
      </el-menu-item>
    </el-menu>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, defineComponent, VNode, h, computed } from "vue";
import { buildView, buildWrap } from "../../core";
import { SVGs, useSvg } from "../index";
import { useRouter } from "vue-router";
const component = "Menu";
defineComponent({
  name: component,
});

const router = useRouter();
/**菜单激活项 */
const menuActiveIndex = ref("0");

/**渲染Menu菜单列表中的icon字段 */
const renderMenuIcon = (icon: SVGs) =>
  h("div", { innerHTML: useSvg(icon, 18), class: "menu-item-icon" });

type MenuItem = { index: string; icon: VNode; label: string; iconActive: VNode };
/** Menu菜单列表 */
const menuList = reactive<MenuItem[]>([
  {
    index: "0",
    icon: renderMenuIcon(SVGs.MENU_MY),
    iconActive: renderMenuIcon(SVGs.MENU_MY_ACTIVE),
    label: "我的 mine",
  },
  {
    index: "1",
    icon: renderMenuIcon(SVGs.MENU_COLLABORATE),
    iconActive: renderMenuIcon(SVGs.MENU_COLLABORATE_ACTIVE),
    label: "协作 collaborate",
  },
  {
    index: "2",
    icon: renderMenuIcon(SVGs.MENU_PLAN),
    iconActive: renderMenuIcon(SVGs.MENU_PLAN_ACTIVE),
    label: "计划 plan",
  },
  {
    index: "3",
    icon: renderMenuIcon(SVGs.MENU_MAIN),
    iconActive: renderMenuIcon(SVGs.MENU_MAIN_ACTIVE),
    label: "重要 main",
  },
  {
    index: "4",
    icon: renderMenuIcon(SVGs.MENU_HISTORY),
    iconActive: renderMenuIcon(SVGs.MENU_HISTORY_ACTIVE),
    label: "历史 history",
  },
]);

/**计算菜单图标 */
const getMenuIcon = computed(() => (item: MenuItem) => {
  return menuActiveIndex.value === item.index ? item.iconActive : item.icon;
});

//激活菜单事件
const selectMenu = (key: string, _keyPath: string[]) => {
  menuActiveIndex.value = key;
  const map = new Map<string, string>([
    ["0", "/"],
    ["1", "collaborate"],
    ["2", "plan"],
    ["3", "main"],
    ["4", "history"],
  ]);
  router.push({ path: map.get(key) || "/" });
};
</script>

<style lang="scss">
@import "../../styles/components/menu.scss";
</style>
