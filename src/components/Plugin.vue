<template>
  <div class="plugin" v-if="plugin">
    <div>
      <img id="icon" v-if="icon && !enabled" :src="icon" />
    </div>
    <div>
      <h2 for="checkbox">{{ plugin.name }}</h2>
      <div class="checkbox-container">
        <label class="checkbox">
          <input
            type="checkbox"
            id="checkbox"
            v-model="enabled"
            v-on:change="updateEnabled(enabled)"
          />
          <span></span>
        </label>
      </div>
    </div>
    <div v-if="enabled">
      <span>{{ plugin.description }}</span>
      <PluginConfig
        :name="name"
        :key="pluginKey"
        :defaultConfig="config"
        @changeConfig="updateConfig($event)"
      />
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import PluginConfig from "./PluginConfig.vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "Plugin",
  props: {
    name: String,
    plugin: Object,
    defaultConfig: Object,
  },
  components: {
    PluginConfig,
  },
  data() {
    return {
      icon: null,
      enabled: false,
      config: null,
      ui: null,
      template: "<div></div>",
      message: "World!",
      pluginKey: "key",
    };
  },
  methods: {
    async loadDefaultConfig() {
      this.config = this.defaultConfig;
      console.log(this.config);
    },
    async loadIcon() {
      invoke("get_plugin_icon", { id: this.name })
        .then((icon) => {
          this.icon = icon;
        })
        .catch((error) => {
          console.error(error);
        });
    },
    async updateConfig(config) {
      this.$emit("changeConfig", config);
    },
    async updateEnabled(enabled) {
      enabled;
    },
  },
  created() {
    console.log("created");
    this.loadDefaultConfig();
    this.loadIcon();
  },
});
</script>
<style scoped lang="scss">
@import "@/styles/_variables.scss";
.plugin {
  margin: 10px;
  padding: 10px;
  min-width: 250px;
  min-height: 250px;
  background-color: white;
  border-radius: 5px;
  box-shadow: 5px 5px 5px $shadow;
  text-align: center;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
.plugin:hover {
  box-shadow: 10px 10px 5px $shadow;
  transform: scale(1.1);
}
.plugin h2 {
  font-weight: normal;
  font-style: normal;
  font-size: 14pt;
  text-transform: uppercase;
}
.checkbox-container {
  height: 50px;
}

.checkbox {
  cursor: pointer;
  position: relative;
}

.checkbox > input {
  height: 40px;
  width: 50px;
  position: absolute;
  left: -25px;
  top: 0rem;
  -webkit-appearance: none;
  -moz-appearance: none;
  -o-appearance: none;
  appearance: none;
  border-radius: 25px;
  outline: none;
  transition-duration: 0.3s;
  background-color: #a2f6f2;
  cursor: pointer;
}

.checkbox > input:checked {
  background-color: #a2f6f2;
}

.checkbox > input:checked + span::before {
  font-size: 2.4em;
  content: "\2713";
  text-align: center;
  color: white;
  position: absolute;
  left: -0.5rem;
  top: 0rem;
}

.checkbox > input:active {
  border: 2px solid #34495e;
}
.checkbox-container:hover {
  transform: scale(0.9);
}

#icon {
  margin: 10px;
  margin-top: 20px;
  height: 96px;
  left: 50%;
}
</style>
