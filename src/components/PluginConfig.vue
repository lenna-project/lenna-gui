<template>
  <div>
    <v-runtime-template :template="template"></v-runtime-template>
    <div class="plugin-config" v-if="ui">
      <div v-for="c in config" :key="c.key">
        <div class="parameter">
          <label>{{ c.key }}: </label>
          <input
            type="number"
            :placeholder="c.key"
            v-model.number="c.value"
            @change="updateConfig()"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// eslint-disable-next-line no-unused-vars
import { defineAsyncComponent } from "vue/dist/vue.esm-bundler.js";
import VRuntimeTemplate from "vue3-runtime-template";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  name: "PluginConfig",
  props: {
    name: String,
    defaultConfig: Object,
  },
  components: {
    "v-runtime-template": VRuntimeTemplate,
  },
  data() {
    return {
      ui: false,
      config: [],
      template: null,
    };
  },
  methods: {
    async updateConfig() {
      let config = {};
      for (let c of this.config) {
        config[c.key] = c.value;
      }
      this.$emit("changeConfig", config);
    },
    async loadUi() {
      invoke("get_plugin_ui", { id: this.name })
        .then((v) => {
          this.template = v.template;
          console.log(this.template);
        })
        .catch((e) => {
          console.log(e);
          this.ui = true;
        });
    },
  },
  created() {
    this.loadUi();
    for (let key in this.defaultConfig) {
      let config = { key: key, value: this.defaultConfig[key] };
      this.config.push(config);
    }
    this.updateConfig();
  },
};
</script>

<style scoped>
.plugin-config {
  margin: 5px;
}
.parameter {
  display: flex;
  justify-content: space-between;
}
.parameter input {
  max-width: 100px;
}
</style>
