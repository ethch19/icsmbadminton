<script setup lang="ts">
import anime from 'animejs/lib/anime.es.js';
const canvas = ref(null);

onMounted(() => {
    const ctx = canvas.value.getContext("2d");

    ctx.beginPath();
    ctx.strokeStyle ="black";
    ctx.moveTo(100, 100);
    ctx.arcTo(1000, 100, 1000, 800, 400);
    ctx.bezierCurveTo(900, 900, 1100, 1000, 1400, 1100)
    ctx.stroke();

    anime({
        targets: ".mobile-line",
        duration: 1500,
        strokeDashoffset: [anime.setDashoffset, 0],
        easing: "easeInOutSine",
        direction: "alternate",
        autoplay: true,
        loop: true,
    });
});

const x = reactive({});
const y = "hello";
x[y] = "world";
console.log(x.hello);
</script>

<template>
    <main class="test-page flex-column">
        <h1 class="title text-center">Test</h1>
        <div>
            <canvas ref="canvas" id="canvas" height=1200 width=1800></canvas>
        </div>
        <svg class="line-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 5 1483.7">
            <path class="mobile-line" d="M2.5 0, L2.5 156.8, 2.5 340.6, 2.5 497.4, 2.5 650.8, 2.5 783.1, 2.5 936.5, 2.5 1120.3, 2.5 1295.9, 2.5 1483.7" style="fill: none; stroke: #707070; stroke-width: 5px;"/>
            <image class="arrowhead" :href="'/img/arrowhead.svg'"/>
        </svg>
    </main>
</template>

<style scoped>
@import url( "~/assets/css/test.css");
</style>