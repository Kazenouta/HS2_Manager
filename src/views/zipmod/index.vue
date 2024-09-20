<template>
  <el-form>
    <el-row>
      <el-col :span="20">
        <el-form-item label="Mod 总数" >
      <el-input readonly v-model="modCount"></el-input>
    </el-form-item>
      </el-col>
      <el-col :span="4">
        <el-button type="primary" @click="refreshModCount">刷新</el-button>
      </el-col>
    </el-row>
    
  </el-form>


    <el-table :data="tableData" style="width: 90%; padding-right: 20px; margin-left: 20px; margin-top: 0px">
      <el-table-column prop="date" label="Date" width="180" />
      <el-table-column prop="name" label="Name" width="180" />
      <el-table-column prop="address" label="Address" />
    </el-table>

</template>


<script lang="ts">
import bgImage from '@/assets/bg.png';
import { invoke } from '@tauri-apps/api';


export default {
  name: 'Zipmod',
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
      modCount: 0,
      tableData: []
    }
  },
  methods: {
    async refreshModCount() {
      const modCount = await invoke('get_mod_count');
      this.modCount = modCount;
    },
    async getModList() {
      const modList = await invoke('get_mod_list');
      this.tableData = modList;
    }
  },
}
</script>

<style scoped>
.el-form {
  margin: 20px;
  width: 400px
}

.el-form-item {
  background-color: #eedfe4;
  width: 300px;
  padding-left: 15px;
}
</style>