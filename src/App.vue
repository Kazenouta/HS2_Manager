<template>
  <el-container class="el-container">
    <el-aside class="el-aside">
      <el-menu class="el-menu" :default-active="activeName" :collapse-transition="false">
        <el-menu-item index="0" @click="navigateTo('Home')">
          首页
          <!-- <router-link to="/home">首页</router-link> -->
        </el-menu-item>
        <el-divider class="el-divider" />
        <el-menu-item index="1" @click="navigateTo('Zipmod')" name="Mods">
          Mods
          <!-- <router-link to="/zipmod">Mods</router-link> -->
        </el-menu-item>
        <el-divider class="el-divider" />
        <el-menu-item index="2">
          <router-link to="/chara">人物卡</router-link>
        </el-menu-item>
        <el-divider class="el-divider" />
        <el-menu-item index="3">
          <router-link to="/abdata">服装卡</router-link>
        </el-menu-item>
        <el-divider class="el-divider" />
        <el-menu-item>
          <el-icon>
            <setting />
          </el-icon>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-main class="el-main">
    <!-- <div style="width: 90%;"> -->
      <router-view />

    <!-- </div> -->
    </el-main>
  </el-container>

</template>

<script lang="ts">
import { useRouter } from 'vue-router';
import {Setting} from '@element-plus/icons-vue'

export default {
  name: 'App',
  components: {
    Setting,
  },
  data() {
    return {
      activeName: '0',
    };
  },
  setup() {
    const router = useRouter();

    const navigateTo = (page: any) => {
      router.push({ name: page });
    };

    return {
      navigateTo,
    };
  },
};
</script>


<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}

.el-container {
  padding: 0px;
  margin: 0px;
  display: flex;
  height: 100%;
  width: 100%;
  box-sizing: border-box;
}

.el-aside {
  width: 85px;
  background-color: #babdc0 !important;
  position: fixed;
  height: 100vh;
  top: 0px;
  left: 0;
}

.el-menu {
  background-color: #babdc0 !important;
}

.el-menu .el-menu-item {
  display: flex;
  /* 使用弹性盒子布局 */
  justify-content: center;
  /* 水平居中 */
  align-items: center;
  /* 垂直居中 */
  height: 50px;
  /* 根据需要设置高度 */
}

.el-menu .el-menu-item.is-active {
  color: #ec2863;
  /* 修改选中项的字体颜色 */
  background-color: #f0f0f0;
  /* 修改选中项的背景颜色 */
}

.el-main {
  background-image: url('@/assets/bg.png');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  height: 100vh;
  width: 100%;
  padding: 20px;
  position: fixed;
  top: 0px;
  left: 85px;
}

.el-divider {
  margin: 0;
  border-color: rgb(233, 88, 156);
}

</style>
