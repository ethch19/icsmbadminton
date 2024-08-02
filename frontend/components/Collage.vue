<script setup lang="ts">
const prop = defineProps<{
    mainimg: string,
    leftimg: string,
    rightimg: string
}>()
</script>

<template>
    <div class="collage">
        <img class="main-collage collage-img" :src=mainimg />
        <img class="left-collage collage-img" :src=leftimg />
        <img class="right-collage collage-img" :src=rightimg />
    </div>
</template>

<style scoped>
@import url("~/assets/css/collage.css");
</style>
