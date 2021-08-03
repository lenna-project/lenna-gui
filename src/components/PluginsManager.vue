<template>
  <div class="plugins-manager">
    <draggable class="dragArea list-group plugins" :list="plugins">
      <div class="list-group-item" v-for="item in plugins" :key="item.name">
        <Plugin
          :name="item.name"
          :plugin="item"
          :defaultConfig="item.config"
          @changeEnabled="changeEnabled(item.name, $event)"
          @changeConfig="changeConfig(item.name, $event)"
        />
      </div>
    </draggable>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import { VueDraggableNext } from "vue-draggable-next";
import Plugin from "@/components/Plugin.vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "PluginsManager",
  props: {},
  components: {
    draggable: VueDraggableNext,
    Plugin,
  },
  data() {
    return {
      plugins: [],
    };
  },
  created() {
    invoke("get_plugin_ids")
      .then((plugins) => {
        plugins.forEach((id) => {
          invoke("get_plugin", { id })
            .then((plugin) => {
              invoke("get_plugin_config", { id: plugin.name })
                .then((config) => {
                  delete config.id;
                  plugin.config = config;
                  this.plugins.push(plugin);
                })
                .catch((error) => {
                  this.plugins.push(plugin);
                  console.error(error);
                });
            })
            .catch((error) => console.error(error));
        });
      })
      .catch((error) => console.error(error));
  },
  methods: {
    changeConfig(name, config) {
      invoke("set_plugin_config", { id: name, config })
        .then(() => {})
        .catch((error) => {
          console.error(error);
        });
    },
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
