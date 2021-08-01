<template>
  <div class="plugins-manager">
    <draggable class="dragArea list-group plugins" :list="plugins">
      <div class="list-group-item" v-for="item in plugins" :key="item.name">
        {{item}}
      </div>
    </draggable>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import { VueDraggableNext } from "vue-draggable-next";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "PluginsManager",
  props: {},
  components: {
    draggable: VueDraggableNext,
  },
  data() {
    return {
      plugins: [],
    };
  },
  created() {
    invoke("get_plugin_ids")
      .then((message) => {
        this.plugins = message;
      })
      .catch((error) => console.error(error));
  },
});
</script>
<style scoped lang="scss">
@import "@/styles/_variables.scss";
.plugins-manager {
  background-color: $body_background;
}
.plugins {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
}
</style>
