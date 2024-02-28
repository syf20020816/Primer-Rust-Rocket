import { createApp } from "vue";
import App from "./App.vue";
//import vue router
import router from "./router/index";
//import global style
import "./styles/global.scss";
//import Element-plus
import ElementPlus from "element-plus";
//导入Element-plus默认样式，若使用自定义则注释掉
// import 'element-plus/dist/index.css'
//使用pinia
import pinia from "./store/pinia";


const app = createApp(App);
app.use(router);
app.use(ElementPlus);
app.use(pinia);
app.mount("#app");
