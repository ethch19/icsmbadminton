<script setup lang="ts">
const prop = defineProps<{
    mainimg: string,
    leftimg: string,
    rightimg: string
}>()
</script>

<template>
    <div class="collage">
        <img class="main-collage" :src=mainimg />
        <img class="left-collage" :src=leftimg />
        <img class="right-collage" :src=rightimg />
    </div>
</template>
