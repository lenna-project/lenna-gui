<template>
  <div class="main">
    <div class="top_main">
      <ImageUpload
        class="v-step-3"
        ref="imageUpload"
        @folderChanged="sourceChanged($event)"
      />
      <div id="process">
        <button class="v-step-5" v-on:click="processImages">
          process images
        </button>
      </div>
      <ImagePreview
        class="v-step-6"
        :images="resultImages"
        @folderChanged="targetChanged($event)"
        @extensionChanged="extensionChanged($event)"
      />
    </div>
    <div class="bottom_main">
      <PluginsManager />
    </div>
    <div id="line">
      <hr />
      <div class="plus radius" v-on:click="onMorePlugins()"></div>
    </div>
    <help/>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import { useToast } from "vue-toastification";
import PluginsManager from "@/components/PluginsManager.vue";
import ImageUpload from "@/components/ImageUpload.vue";
import ImagePreview from "@/components/ImagePreview.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import Help from "@lenna-project/lenna-web/src/components/Help.vue";

export default defineComponent({
  name: "Home",
  components: {
    PluginsManager,
    ImageUpload,
    ImagePreview,
    Help
  },
  data() {
    return {
      source: ".",
      target: ".",
      extension: "png",
      sourceImages: [],
      resultImages: [],
    };
  },
  methods: {
    sourceChanged(folder) {
      console.log("source: ", folder);
      this.source = folder;
    },
    targetChanged(folder) {
      console.log("target: ", folder);
      this.target = folder;
    },
    extensionChanged(extension) {
      this.extension = extension;
      console.log(this.extension);
    },
    processImages() {
      invoke("process", {
        source: this.source,
        target: this.target,
        extension: this.extension,
      }).then((res) => {
        console.log(res);
      });
    },
    async registerEvents() {
      this.unlistenInfo = await listen("info", (event) => {
        const toast = useToast();
        toast.info(event.payload.message);
      });
      this.unlistenSuccess = await listen("success", (event) => {
        const toast = useToast();
        toast.success(event.payload.message);
      });
    },
  },
  created() {
    this.registerEvents();
  },
});
</script>
<style scoped lang="scss">
@import "@/styles/_variables.scss";
@import url("https://fonts.googleapis.com/css2?family=Amatic+SC&display=swap");

.main {
  background-color: $body_background;
  padding: 150px;
  padding-top: 120px;
  text-align: center;
}
.main h1 {
  text-transform: uppercase;
  font-family: "Amatic SC", cursive;
  font-size: 32pt;
}
.main hr {
  margin-top: 10px;
  border: none;
  border-top: 2px dotted blue;
  color: #fff;
  background-color: #fff;
  height: 2px;
  width: 100%;
}
.top_main {
  display: flex;
  justify-content: space-between;
}
.bottom_main {
  padding-top: 20px;
  padding-bottom: 0;
}
#process button {
  margin-top: 40%;
  width: 350px;
  align-self: center;
  font-size: 20pt;
  text-transform: uppercase;
  padding: 12px;
  border-radius: 30px;
  border: none;
  box-shadow: 5px 5px 5px $shadow;
}
#process button:hover {
  transform: scale(1.05);
}
#line {
  margin-top: 50px;
  position: relative;
}

.plus {
  position: absolute;
  top: -25px;
  left: calc(50% - 20px);

  display: inline-block;
  width: 50px;
  height: 50px;

  background: linear-gradient(#fff, #fff), linear-gradient(#fff, #fff), blue;
  background-position: center;
  background-size: 50% 2px, 2px 50%; /*thickness = 2px, length = 50% (25px)*/
  background-repeat: no-repeat;
}
.radius {
  border-radius: 50%;
}
.plus:hover {
  transform: scale(1.1);
}

@media screen and (max-width: 800px) {
  .main {
    margin-top: 120px;
    padding: 0;
  }
  #process button {
    width: 150px;
  }
}
</style>
