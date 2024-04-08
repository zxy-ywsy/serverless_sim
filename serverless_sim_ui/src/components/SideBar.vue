<template>
  <div class="col_container sidebar">
    <!-- 新增的网络拓扑图链接 -->
    <div @click="navigateToTopology">网络拓扑图</div>
    <div v-for="(value, key) in records" :key="key" @click="item_click(key)">
      {{ "_" + key in selected_keys ? ">" : "" }}{{ value }}
    </div>
  </div>
</template>

<script lang="ts">
import { request } from "@/request";
import {page} from "@/page";

export default {
  data() {
    return {
      records: ["当前仿真监控", 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
      _select_bar: (idx: number, name: string) => {},

    };
  },

  mounted() {
    request
        .history_list()
        .request()
        .then((history_list) => {
          console.log("history list", history_list);
          this.records = ["当前仿真监控"].concat(history_list.data.list);
        });
  },

  methods: {
    init(_select_bar: (select: number, select_name: string) => void) {
      this._select_bar = _select_bar;
    },
    item_click(key) {
      this._select_bar(key, this.records[key]);
    },
    navigateToTopology() {
      this.$router.push({ name: 'NetworkTopology' });
    }
  },
};
</script>

<style scoped>
.row {
  display: flex;
  flex-direction: row;
}
.col_container {
  display: flex;
  flex-direction: column;
}
.sidebar {
  overflow: scroll;
}
</style>


