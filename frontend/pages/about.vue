<script setup lang="ts">
definePageMeta({
    layout: "default"
});
import { vInfiniteScroll } from '@vueuse/components';
import anime from 'animejs/lib/anime.es.js';

const { data }  = await useAsyncData('about', () => queryContent('/about').findOne());
const items = data.value.body;
const ready = ref(false);
const lineAnimation = reactive({});
const fadeAnimation = reactive({});
const pathAnimation = reactive({});
let ticking = false;

const vw = ref(null);
const mobile = ref(false);
const tablet = ref(false);

function widthResized() {
    vw.value = Math.max(document.documentElement.clientWidth);
    if (vw.value <= 768) {
        mobile.value = true;
        tablet.value = false;
    } 
    else if (vw.value <= 1200) {
        mobile.value = false;
        tablet.value = true;
    }
    else {
        mobile.value = false;
        tablet.value = false;
    }
}

function presetFadeAnimation(name: string, target: string, start: number, total: number, remaining: number) {
    fadeAnimation[name] = anime({
        targets: target,
        easing: "easeInSine",
        keyframes: [
            {translateY: [0,0], opacity: [0,0], duration: start},
            {translateY: ["-1rem", 0], opacity: [0,1], duration: total},
            {opacity: 1, duration: remaining}
        ],
        direction: "alternate",
        autoplay: false,
    });
}

onBeforeMount(() => {
    document.addEventListener("scroll", seekAnimations);
});

onMounted(() => {
    widthResized();
    var doit;
    window.onresize = () => {
      clearTimeout(doit);
      doit = setTimeout(widthResized, 100);
    };
    nextTick(() => {
        pathAnimation.value = anime.path(".linetree");
        let easing = "linear";
        if (!(mobile.value || tablet.value)) {
            easing = "easeInOutSine";
            anime({
                targets: ".content-box0",
                easing: "easeInSine",
                translateY: ["-1rem", 0],
                opacity: [0,1],
                duration: 1000,
                direction: "normal",
                autoplay: true,
            });
            presetFadeAnimation("content1", ".content-box1", 19, 10, 71);
            presetFadeAnimation("content2", ".content-box2", 58, 11, 31);
            presetFadeAnimation("img1", ".photo1", 4, 10, 84);
            presetFadeAnimation("img2", ".photo2", 32, 8, 60);
            presetFadeAnimation("img3", ".photo3", 49, 7, 44);
            presetFadeAnimation("img4", ".photo4", 69, 3, 28);
            presetFadeAnimation("silo1", ".silo1", 30, 3, 67);
            presetFadeAnimation("silo2", ".silo2", 42, 6, 52);
            presetFadeAnimation("silo3net", ".silo3, .net", 64, 6, 30);
            fadeAnimation.end = anime({
                targets: ".join-container",
                easing: "easeInSine",
                keyframes: [
                    {translateY: ["-4.5rem", "-4.5rem"], translateX: ["24rem", "24rem"], opacity: [0,0], duration: 82},
                    {translateY: ["-5.5rem", "-4.5rem"], opacity: [0,1], duration: 13},
                    {opacity: 1, duration: 5}
                ],
                direction: "alternate",
                autoplay: false,
            });
        }
        lineAnimation.path = anime({
            targets: ".linetree",
            keyframes: [
                {opacity: [0,0], duration: 2},
                {opacity: [1,1], strokeDashoffset: [anime.setDashoffset, -0], duration: 98},
            ],
            easing: easing,
            direction: "normal",
            autoplay: false,
        });
        lineAnimation.arrow = anime({
            targets: ".arrowhead",
            keyframes: [
                {opacity: [0,0], duration: 2},
                {opacity: [1,1], translateY: pathAnimation.value("y"), translateX: pathAnimation.value("x"), rotate: pathAnimation.value("angle"), duration: 98},
            ],
            easing: easing,
            direction: "normal",
            autoplay: false,
        });
        seekAnimations();
    });
    ready.value = true;
});

onBeforeUnmount(() => {
    document.removeEventListener("scroll", seekAnimations);
});

onUnmounted(() => {
    window.onresize = null;
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
    <main class="about-page flex-column">
        <h1 class="title text-center">About</h1>
        <div v-show="ready" class="about-container flex-column">
            <div class="path-container">
                <svg v-if="!(mobile || tablet)" class="path-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 862.2 1650.9" preserveAspectRatio="xMidYMin slice">
                    <path class="linetree" d="m405.2,1.4s86.5,34.9,214.8,6.2c188.2-42.2,243.5,307.3,92.5,339.5-152,32.4-220.1-58.5-311.4-64.4-76-4.9-132.3,1.2-197.9,67.9-69.9,71.1-92.4,117.9-110,159.2-39.6,93-87.8,210.7,21.1,217.8,144,9.4,272.8,10.5,294.2,151.6,14.8,97.6-185.6,141.1-289.5,173.9-211.9,66.7-99.9,315.9,44.7,295.1,39.3-5.7,107.1-55.2,178.1-108.4,89.8-67.3,185.8-145.7,329.8-98.2,61.9,20.4,58.7,134.1,0,193.7-46.9,47.5-5.3,93,29.3,149.9" style="fill: none; stroke: #707070; stroke-width: 3px;"/>
                    <image class="arrowhead" :href="'/img/arrowhead.svg'"/>
                </svg>
                <svg v-if="tablet" class="line-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 206.9 1497.7" preserveAspectRatio="xMidYMin slice">
                    <path class="linetree" d="m36.6,1.2c142.4,115.2,171.2,206.3,168.6,270.4-2.1,52.2-23.8,56.2-43.9,151-21.1,100-2.2,120.1-17.6,265.2-9,85.4-13.6,128.1-33.4,179.1-44.3,114.1-95.8,108.5-107.1,180.9-14.7,93.8,67.2,132.3,91.3,272.2,12.8,74.6,2.4,138.2-7,177.4" style="fill: none; stroke: #707070; stroke-width: 3px;"/>
                    <image class="arrowhead" :href="'/img/arrowhead.svg'"/>
                </svg>
                <svg v-if="mobile" class="line-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 5 1483.7" preserveAspectRatio="xMidYMin slice">
                    <path class="linetree" d="M2.5 0, L2.5 156.8, 2.5 340.6, 2.5 497.4, 2.5 650.8, 2.5 783.1, 2.5 936.5, 2.5 1120.3, 2.5 1295.9, 2.5 1483.7" style="fill: none; stroke: #707070; stroke-width: 3px;"/>
                    <image class="arrowhead" :href="'/img/arrowhead.svg'"/>
                </svg>
            </div>
            <div class="block-container">
                <ContentBlock v-for="(item, index) in items" :class="'content-box'+index" :subtitleHeight="'1.2rem'" :boxPadding="'1.5rem'" v-bind="item"/>
                <img v-for="n in 4" class="photo" :class="'photo'+n" :src="'/img/gallery/about-'+n+'.jpg'"/>
                <img v-for="n in 2" class="silhouette" :class="'silo'+n" :src="'/img/silhouette'+n+'.svg'"/>
                <div class="flex-row silonet-container">
                    <img class="silo3" src="/img/silhouette3.svg"/>
                    <img class="net" src="/img/badminton-net.svg"/>
                </div>
            </div>
            <!-- <div v-else class="block-container flex-column">
                <ContentBlock class="content-box1 contentbox" :subtitleHeight="'1.2rem'" :boxPadding="'1.5rem'" v-bind="items[0]"/>
                <img class="photo1 photo" src="/img/gallery/about-1.jpg"/>
                <img class="silo1 silhouette" src="/img/silhouette1.svg"/>
                <div class="flex-row silonet-container">
                    <img class="silo3" src="/img/silhouette3.svg"/>
                    <img class="net" src="/img/badminton-net.svg"/>
                </div>
            </div> -->
            <div class="join-container flex-column">
                <h2 class="subhead join-h2">Start your journey with us my drilla</h2>
                <a class="linkbutton button primary-button" href="https://www.imperialcollegeunion.org/activities/a-to-z/badminton-icsm" target="_blank">Find out more</a>
            </div>
        </div>
    </main>
</template>

<style scoped>
@import url( "~/assets/css/about.css");
</style>
