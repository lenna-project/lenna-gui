<template>
  <div class="file-upload">
    <input v-model="folder" readonly="true" placeholder="" />
    <button v-on:click="openDialog">Folder</button>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { readBinaryFile, readDir } from "@tauri-apps/api/fs";

function arrayBufferToBase64(buffer, callback) {
  const blob = new Blob([buffer], {
    type: "application/octet-binary",
  });
  const reader = new FileReader();
  reader.onload = function (evt) {
    const dataurl = evt.target.result;
    callback(dataurl.substr(dataurl.indexOf(",") + 1));
  };
  reader.readAsDataURL(blob);
}

export default defineComponent({
  name: "FileUpload",
  props: {
    modelValue: Array,
    extensions: String,
  },
  components: {},
  data() {
    return {
      selected: null,
      files: [],
      folder: ".",
    };
  },
  emits: ["update:modelValue", "input-file"],
  methods: {
    openDialog() {
      open({ defaultPath: this.folder, directory: true, multiple: false }).then(
        (folder) => {
          this.folder = folder;
          this.reloadDir(this.folder);
        }
      );
    },
    loadFile(file) {
      readBinaryFile(file.path, {}).then((response) => {
        arrayBufferToBase64(new Uint8Array(response), (base64) => {
          const src = "data:image/png;base64," + base64;
          this.$emit("input-file", {
            name: file.name,
            path: file.path,
            url: src,
          });
        });
      });
    },
    reloadDir(folder) {
      readDir(folder, { recursive: false }).then((d) => {
        let extensions = this.extensions.split(",");
        this.files = d.filter((i) =>
          extensions.includes(i.name.split(".").pop())
        );
        this.files.forEach((file) => {
          this.loadFile(file);
        });
        this.$emit("update:modelValue", this.files);
      });
    },
  },
  created() {
    this.reloadDir(".");
  },
});
</script>
<style scoped lang="scss">
@import "@/styles/_variables.scss";
.file-upload {
  margin: 10px;
  padding: 10px;
  min-width: 250px;
  min-height: 50px;
  background-color: white;
  border-radius: 5px;
  box-shadow: 5px 5px 5px $shadow;
  text-align: center;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
</style>
