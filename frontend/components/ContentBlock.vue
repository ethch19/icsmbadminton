<script setup lang="ts">
const props = defineProps<{
    subtitle: string,
    paragraph: string,
    img?: string,
    subtitleHeight?: string,
    boxPadding?: string,
}>()

const imgBool = ref(true);
if (typeof props.img == "undefined") {
    imgBool.value = false;
}
</script>

<template>
    <div  class="content-box flex-column">
        <div class="content-box-header flex-row">
            <img class="content-box-logo" v-if="imgBool" :src="img"/>
            <h2 class="subtitle">{{ subtitle }}</h2>
        </div>
        <p class="text">{{ paragraph  }}</p>
    </div>
</template>

<style scoped>
@import url("~/assets/css/contentblock.css");
.content-box-header {
    transform: translateY(v-bind(subtitleHeight));
}
.content-box .text {
    padding: v-bind(boxPadding);
}
</style>
