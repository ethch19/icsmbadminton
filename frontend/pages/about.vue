<script setup lang="ts">
definePageMeta({
    layout: "default"
});

const { data }  = await useAsyncData('about', () => queryContent('/about').findOne());
const items = data.value.body;
</script>

<template>
    <main>
        <h1 class="title text-center">About</h1>
        <div class="about-container">
            <ContentBlock v-for="item in items" :subtitle="item.title" :paragraph="item.text"/>
        </div>
    </main>
</template>

<style scoped>
@import url( "~/assets/css/about.css");
</style>
