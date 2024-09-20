import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';

import { createRouter, createWebHistory } from 'vue-router';
import Home from './views/Home.vue';
import Zipmod from './views/zipmod/index.vue';

const routes = [
  { path: '/home', name: 'Home', component: Home },
  { path: '/zipmod', name: 'Zipmod', component: Zipmod },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

const app = createApp(App)
app.use(ElementPlus)
app.use(router)
app.mount('#app')
