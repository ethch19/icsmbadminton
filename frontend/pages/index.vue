<script setup lang="ts">
import { vIntersectionObserver } from "@vueuse/components";
const mainContainer = ref(null);

const { data } = await useAsyncData("photos-info", () => queryContent("/landing-photos").findOne());
const info = data.value.body;

function onIntersectionObserver([{ isIntersecting, target }]: IntersectionObserverEntry[]) {
    if (isIntersecting) {
        target.classList.add("show-box");
    }
}

function scroll(event) {
    mainContainer.value.scrollIntoView({behavior: "smooth", block: "start"});
}
</script>

<template>
    <main class="landing-container">
        <div class="jumbo">
            <span class="title-container" >
                <h1 class="title align-left">ICSM<br/> Badminton</h1>
                <button @click="scroll" class="primary-button button">Find out more</button>
            </span>
            <img class="jumbo-img" src="/img/badminton-players.svg" />
        </div>
        <div ref="mainContainer" class="flex-column main-container">
            <Suspense>
                <section v-for="block in info" v-intersection-observer="onIntersectionObserver" class="content-box flex-row hidden-box" :class="{ 'box-reverse': block.reverse}">
                    <Collage v-bind="block.photos"/>
                    <ContentBlock v-bind="block.content" boxPadding="3rem var(--margin-xl) var(--margin-xl)"/>
                </section>
            </Suspense>
        </div>
        <div class="flex-row ending">
            <h2 class="subtitle landing-ending">No better reason to join us!</h2>
        </div>
    </main>
</template>

<style scoped>
@import url("~/assets/css/index.css");
</style>
