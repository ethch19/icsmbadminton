<script setup lang="ts">
definePageMeta({
    layout: "default"
});
import anime from 'animejs/lib/anime.es.js';

const { data }  = await useAsyncData('about', () => queryContent('/about').findOne());
const items = data.value.body;
const ready = ref(false);
const lineAnimation = reactive({});
const fadeAnimation = reactive({});
const pathAnimation = reactive({});
let ticking = false;

onBeforeMount(() => {
    document.addEventListener("scroll", seekAnimations);
});

onMounted(() => {
    pathAnimation.value = anime.path(".linetree");
    lineAnimation.path = anime({
        targets: ".linetree",
        keyframes: [
            {opacity: [0,0], duration: 6},
            {opacity: [1,1], strokeDashoffset: [anime.setDashoffset, -0], duration: 94},
        ],
        easing: "easeInOutSine",
        direction: "normal",
        autoplay: false,
    });
    lineAnimation.arrow = anime({
        targets: ".arrowhead", 
        keyframes: [
            {opacity: [0,0], duration: 6},
            {opacity: [1,1], translateY: pathAnimation.value("y"), translateX: pathAnimation.value("x"), rotate: pathAnimation.value("angle"), duration: 94},
        ],
        easing: "easeInOutSine",
        direction: "normal",
        autoplay: false,
    });
    anime({
        targets: ".content-box0",
        easing: "easeInSine",
        translateY: ["-1rem", 0],
        opacity: [0,1],
        duration: 1000,
        direction: "normal",
        autoplay: true,
    });
    fadeAnimation.content1 = anime({
        targets: ".content-box1",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 25},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 4},
            {opacity: 1, duration: 71}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.content2 = anime({
        targets: ".content-box2",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 54},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 8},
            {opacity: 1, duration: 38}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.img1 = anime({
        targets: ".photo1",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 6},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 10},
            {opacity: 1, duration: 84}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.img2 = anime({
        targets: ".photo2",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 35},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 6},
            {opacity: 1, duration: 59}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.img3 = anime({
        targets: ".photo3",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 45},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 4},
            {opacity: 1, duration: 51}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.img4 = anime({
        targets: ".photo4",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 68},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 4},
            {opacity: 1, duration: 28}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.silo1 = anime({
        targets: ".silo1",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 5},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 18},
            {opacity: 1, duration: 82}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.silo2 = anime({
        targets: ".silo2",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 40},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 13},
            {opacity: 1, duration: 47}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.silo3net = anime({
        targets: ".silo3, .net",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 64},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 5},
            {opacity: 1, duration: 29}
        ],
        direction: "alternate",
        autoplay: false,
    });
    fadeAnimation.end = anime({
        targets: ".join-container",
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: 83},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: 12},
            {opacity: 1, duration: 5}
        ],
        direction: "alternate",
        autoplay: false,
    });
    seekAnimations();
    ready.value = true;
});

onBeforeUnmount(() => {
    document.removeEventListener("scroll", seekAnimations);
});

function seekAnimations(event) {
    if (!ticking) {
        window.requestAnimationFrame(() => {
            let scrollPercent = useScrollPercent();
            for (const i in fadeAnimation) {
                fadeAnimation[i].seek((scrollPercent / 100)* fadeAnimation[i].duration);
            }
            if (scrollPercent <= 95) {
                for (const i in lineAnimation) {
                    lineAnimation[i].seek(((scrollPercent + 5) / 100)* lineAnimation[i].duration);
                }
            } else {
                for (const i in lineAnimation) {
                    lineAnimation[i].seek(lineAnimation[i].duration);
                }
                document.removeEventListener("scroll", seekAnimations);
            }
            ticking = false;
        });
        ticking = true;
    }
}
</script>

<template>
    <main class="flex-column">
        <h1 class="title text-center">About</h1>
        <div v-show="ready" class="about-container flex-column">
            <div class="path-container">
                <svg class="path-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 814.2 920.7" preserveAspectRatio="xMidYMid meet">
                    <path class="linetree" d="m350.2,81.5s40,31.7,117.2,0c77.2-31.7,113.1-93.8,168.9-77.2,55.9,16.6,117.2,29.7,162.7,77.2,45.5,47.6-31.7,224.8-113.1,235.8-81.4,11-458.6,41.4-477.9,138.6-19.3,97.2-292.4,114-179.3,185.7,113.1,71.7,76.5,227.6,206.9,196.5,171.4-46.3,375.5-233.9,465.4-119.3s-91.3,83.9-34.3,201.8" style="fill: none; stroke: #707070; stroke-width: 3px;"/>
                    <image class="arrowhead" :href="'/img/arrowhead.svg'"/>
                </svg>
            </div>
            <div class="block-container">
                <ContentBlock v-for="(item, index) in items" :class="'content-box'+index" :subtitleHeight="'1.2rem'" :boxPadding="'1.5rem'" v-bind="item"/>
                <img v-for="n in 4" class="photo" :class="'photo'+n" :src="'/img/gallery/about-'+n+'.jpg'"/>
                <img v-for="n in 3" class="silhouette" :class="'silo'+n" :src="'/img/silhouette'+n+'.svg'"/>
                <img class="net" src="/img/badminton-net.svg"/>
            </div>
            <div class="end-container flex-row">
                <div class="join-container flex-column">
                    <h2 class="subhead join-h2">Start your journey with us my drilla</h2>
                    <a class="linkbutton button primary-button" href="https://www.imperialcollegeunion.org/activities/a-to-z/badminton-icsm" target="_blank">Find out more</a>
                </div>
            </div>
        </div>
    </main>
</template>

<style scoped>
@import url( "~/assets/css/about.css");
</style>
