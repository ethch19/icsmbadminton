<script setup lang="ts">
import { vIntersectionObserver } from "@vueuse/components";
const mainContainer = ref(null);

const { data } = await useAsyncData("photos-info", () => queryContent("/landing-photos").findOne());
const info = data.value.body;

function onIntersectionObserver([{ isIntersecting, target }]: IntersectionObserverEntry[]) {
    if (isIntersecting) {
        target.classList.add("show-container");
    }
}

function scroll(event) {
    mainContainer.value.scrollIntoView({behavior: "smooth", block: "start"});
}
</script>

<template>
    <main class="landing-container">
        <div class="jumbo">
            <div class="title-container" >
                <h1 class="title align-left">ICSM<br/> Badminton</h1>
                <button @click="scroll" class="primary-button button">Find out more</button>
            </div>
            <img class="jumbo-img" src="/img/badminton-players.svg" />
        </div>
        <div ref="mainContainer" class="flex-column main-container">
            <Suspense>
                <section v-for="block in info" v-intersection-observer="onIntersectionObserver" class="content-container flex-row hidden-container" :class="{ 'container-reverse': block.reverse}">
                    <Collage v-bind="block.photos"/>
                    <ContentBlock :class="{ 'content-block-reverse': block.reverse, 'content-block': !block.reverse}" v-bind="block.content" subtitleHeight="1rem" boxPadding="3rem var(--margin-xl) var(--margin-xl)"/>
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
