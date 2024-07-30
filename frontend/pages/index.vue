<script setup lang="ts">
definePageMeta({
    layout: "default"
})
</script>

<template>
    <main>
        <div class="jumbo">
            <span class="title-container" >
                <h1 class="title">ICSM<br/> Badminton</h1>
                <button>Find out more</button>
            </span>
            <img class="main" src="/img/badminton-players.svg" />
        </div>
        <div class="section">
            <Collage mainimg="/img/gallery/social-main.jpg" leftimg="/img/gallery/social-left.jpg" rightimg="/img/gallery/social-right.jpg" />
            <ContentBlock subtitle="Social Events N Shit" paragraph="Placeholder"/>
        </div>
    </main>
</template>

<style>
@import url("~/assets/css/index.css");
</style>
