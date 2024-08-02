<script setup lang="ts">
definePageMeta({
    layout: "default"
})
</script>

<template>
    <main class="landing-container">
        <div class="jumbo">
            <span class="title-container" >
                <h1 class="title align-left">ICSM<br/> Badminton</h1>
                <button class="primary-button">Find out more</button>
            </span>
            <img class="jumbo-img" src="/img/badminton-players.svg" />
        </div>
        <div class="scroll-container">
            <Collage mainimg="/img/gallery/social-main.jpg" leftimg="/img/gallery/social-left.jpg" rightimg="/img/gallery/social-right.jpg" />
            <ContentBlock img="/img/underground.svg"  subtitle="Social Events" paragraph="Welcome to the friendliest, most exciting, up and coming clubs at ICSM! We welcome people of all subjects (not just medics) and have a wide mix of ages and abilities. It's the best of both worlds! Whether you want to play competitively, in one of our four highly competitive teams or just want some casual exercise with your friends – there’s something for everyone! For team members, we host training twice a week and weekly games run by our fabulous team captains!"/>
        </div>
    </main>
</template>

<style scoped>
@import url("~/assets/css/index.css");
</style>
