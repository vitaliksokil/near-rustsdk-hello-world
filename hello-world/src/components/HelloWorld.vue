<template>
  <div class="hello">
    <div class="webflow-style-input">
      <input class="" type="text" placeholder="What's your name?" v-model="name" @keyup.enter="getHelloMessage"/>
      <button type="submit" @click="getHelloMessage"><i class="icon ion-android-arrow-forward">&#8594;</i></button>
    </div>
  </div>
</template>

<script>
import { getHelloMessage } from '../utils';

export default {
  name: 'HelloWorld',
  data(){
    return {
      name: ''
    }
  },
  methods:{
      async getHelloMessage()
      {
        if (this.name){
          const result = await getHelloMessage(this.name);
          alert(result);
          this.name = '';
        }
      }
  },
  async mounted() {
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>
