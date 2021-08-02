<template>
  <div class="main">
    <h1>Convert images online without upload of your data</h1>
    <PluginsManager />
    <div class="top_main">
      <div id="process">
        <button class="v-step-5" v-on:click="processImages">
          process images
        </button>
      </div>
    </div>
    <div class="bottom_main"></div>
    <div id="line">
      <hr />
      <div class="plus radius" v-on:click="onMorePlugins()"></div>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import PluginsManager from '@/components/PluginsManager.vue'

export default defineComponent({
  name: "Home",
  components: {
    PluginsManager
  },
  data() {
    return {
    }
  },
  created() {
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
