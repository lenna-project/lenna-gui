<template>
  <div class="image-upload">
    <FileUpload
      extensions="gif,jpg,jpeg,png,webp"
      v-model="files"
      @input-file="inputFile"
      ref="upload"
      class="file-upload"
    >
      drop<br />or<br />select files</FileUpload
    >
    <div v-if="images.length > 0" class="image-container">
      <div
        v-for="(src, index) in images"
        :key="index"
        class="pic"
        @click="() => showImg(index)"
      >
        <img height="140" :src="src" />
      </div>
      <vue-easy-lightbox
        escDisabled
        moveDisabled
        :visible="visible"
        :imgs="images"
        :index="index"
        @hide="handleHide"
      ></vue-easy-lightbox>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent } from "vue";
import FileUpload from "./FileUpload.vue";
import VueEasyLightbox from "vue-easy-lightbox";

export default defineComponent({
  components: {
    FileUpload,
    VueEasyLightbox,
  },
  emits: ["changeImage"],
  data() {
    return {
      files: [],
      images: [],
      visible: false,
      index: 0,
    };
  },
  methods: {
    inputFile(newFile) {
      this.images.push(newFile.url);
      this.$emit("changeImage", newFile);
    },
    showImg(index) {
      this.index = index;
      this.visible = true;
    },
    handleHide() {
      this.visible = false;
    },
  },
});
</script>
<style scoped lang="scss">
@import "@/styles/_variables.scss";
.image-upload {
  margin: 10px;
  width: 300px;
  height: 350px;
  background-color: $background_color;
  border: 2px solid black;
  border-radius: 10px;
  box-shadow: 10px 5px 5px $shadow;
  text-align: center;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
.file-upload {
  text-transform: uppercase;
  font-size: 14pt;
  margin: 10px;
  display: flex;
  justify-content: center;
  align-items: center;
}
.file-upload:hover {
  transform: scale(1.05);
}
.image-container {
  height: 200px;
  display: flex;
  flex-wrap: wrap;
  overflow-y: auto;
}
</style>
