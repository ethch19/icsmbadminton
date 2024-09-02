<script setup lang="ts">
const props = defineProps<{
    title: string,
    text: string,
    logo?: string,
    subtitleHeight?: string,
    boxPadding?: string,
}>()
const imgBool = ref(true);
if (typeof props.logo == "undefined") {
    imgBool.value = false;
}
</script>

<template>
    <div  class="content-box flex-column">
        <div class="content-box-header flex-row">
            <img class="content-box-logo" v-if="imgBool" :src="logo"/>
            <h2 class="subtitle">{{ title }}</h2>
        </div>
        <p class="text">{{ text  }}</p>
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
