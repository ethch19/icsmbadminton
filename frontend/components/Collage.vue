<script setup lang="ts">
const prop = defineProps<{
    mainLink: string,
    leftLink: string,
    leftTop: string,
    leftLeft: string,
    rightLink: string,
    rightTop: string,
    rightLeft: string,
}>();
</script>

<template>
    <div class="collage">
        <img class="main-collage collage-img" :src="mainLink"/>
        <img class="left-collage collage-img" :src="leftLink" :style="{ top: leftTop, left: leftLeft }" />
        <img class="right-collage collage-img" :src="rightLink" :style="{ top: rightTop, left: rightLeft }"/>
    </div>
</template>

<style scoped>
@import url("~/assets/css/collage.css");
</style>
