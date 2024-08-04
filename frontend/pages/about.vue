<script setup lang="ts">
definePageMeta({
    layout: "default"
});

const { data }  = await useAsyncData('about', () => queryContent('/about').findOne());
const items = data.value.body;

import anime from 'animejs/lib/anime.es.js';

const readysvg = ref(false);

onMounted(() => {
    readysvg.value = true;
    var path = anime.path(".linetree");
    anime({
        targets: ".linetree",
        strokeDashoffset: [anime.setDashoffset, -0],
        easing: "easeInOutSine",
        duration: 5000,
        loop: 1,
        direction: "normal",
    }); 
    anime({
        targets: ".arrowhead", 
        translateY: path("y"), 
        translateX: path("x"),
        rotate: path("angle"),
        easing: "easeInOutSine",
        duration: 5000,
        loop: 1,
        direction: "normal",
    });
});

</script>

<template>
    <main class="flex-column">
        <h1 class="title text-center">About</h1>
        <div class="about-container flex-column">
            <div class="path-container">
                <svg v-show="readysvg" class="path-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 814.2 920.7" preserveAspectRatio="xMidYMid meet">
                    <path class="linetree" d="m350.2,81.5s40,31.7,117.2,0c77.2-31.7,113.1-93.8,168.9-77.2,55.9,16.6,117.2,29.7,162.7,77.2,45.5,47.6-31.7,224.8-113.1,235.8-81.4,11-458.6,41.4-477.9,138.6-19.3,97.2-292.4,114-179.3,185.7,113.1,71.7,76.5,227.6,206.9,196.5,171.4-46.3,375.5-233.9,465.4-119.3s-91.3,83.9-34.3,201.8" style="fill: none; stroke: #707070; stroke-width: 3px;"/>
                    <image class="arrowhead" :href="'/img/arrowhead.svg'"/>
                </svg>
            </div>
            <div class="block-container">
                <ContentBlock v-for="(item, index) in items" :class="'content-box'+index" :subtitleHeight="'1.2rem'" :boxPadding="'1.5rem'" :subtitle="item.title" :paragraph="item.text"/>
                <img v-for="n in 4" class="photo" :class="'photo'+n" :src="'/img/gallery/about-'+n+'.jpg'"/>
                <img v-for="n in 3" class="silhouette" :class="'silo'+n" :src="'/img/silhouette'+n+'.svg'"/>
                <img class="net" src="/img/badminton-net.svg"/>
            </div>
            <div class="end-container flex-row">
                <div class="join-container flex-column">
                    <h2 class="subhead join-h2">Start your journey with us my drilla</h2>
                    <a class="linkbutton primary-button" href="https://www.imperialcollegeunion.org/activities/a-to-z/badminton-icsm" target="_blank">Find out more</a>
                </div>
            </div>
        </div>
    </main>
</template>

<style scoped>
@import url( "~/assets/css/about.css");
</style>
