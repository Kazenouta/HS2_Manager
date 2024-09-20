<template>
  <el-form>
    <el-row>
      <el-col :span="19">
        <el-form-item label="游戏根目录:">
          <el-input v-model="gamePath" placeholder="请输入游戏根目录">
          </el-input>
        </el-form-item>
      </el-col>
      <el-col :span="5">
        <el-button type="primary" @click="savePath">提交</el-button>
      </el-col>
    </el-row>
    <el-row>
      <el-col :span="24">
        <el-form-item label="Mod 个数:">
          <el-input v-model="gamePath" placeholder="请输入游戏根目录">
          </el-input>
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>

</template>

<script lang="ts">
import bgImage from '@/assets/bg.png';
import { invoke } from '@tauri-apps/api';


export default {
  name: 'Home',
  
  data() {
    return {
      backgroundStyle: {
        backgroundImage: `url(${bgImage})`,
        backgroundSize: 'cover',
        backgroundPosition: 'left top',
        backgroundRepeat: 'no-repeat',
        height: '100vh', // 确保 div 有高度
        width: '100vw', // 确保 div 宽度充满整个屏幕
      },
      gamePath: '',
    }
  },
  mounted() {
    this.getBaseInfo();
  },
  methods: {
    async savePath() {
      let param = {
        name: 'dir_root',
        value: this.gamePath,
      }
      // let result = await invoke('update_config', {param: JSON.stringify(param)});
      let result = await invoke('update_config', param);

      console.log(result);
    },
    async getBaseInfo() {
      let result = await invoke('get_base_info');
      // console.log(result);
      this.gamePath = JSON.parse(result as string).dir_root;
    }

  },
}

</script>


<style scoped>


.el-form {
  margin: 50px;
  width: 400px
}

.el-form-item {
  background-color: #eedfe4;
  width: 300px;
  padding-left: 15px;
}
</style>