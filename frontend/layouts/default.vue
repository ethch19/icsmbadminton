<style scoped>
@import url("~/assets/css/headerfooter.css");
</style>

<script setup lang="ts">
const route = useRoute();

const menuToggle = ref(false);

function menuPressed(event) {
    setTimeout(() => {
        menuToggle.value = menuToggle.value == false ? true : false;
    }, 100);
}

const vw = ref(null);
const mobile = ref(false);

function widthResized() {
    vw.value = Math.max(document.documentElement.clientWidth);
    if (vw.value <= 992) {
        mobile.value = true;
    } else {
        mobile.value = false;
    }
}

onMounted(() => {
    widthResized();
    var doit;
    window.onresize = () => {
      clearTimeout(doit);
      doit = setTimeout(widthResized, 100);
    };
});

onUnmounted(() => {
    window.onresize = null;
});
</script>

<template>
    <div class="landing-page">
        <header class="landing-header">
            <div class="header-container">
                <img class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
                <span v-if="!mobile" class="link-container">
                    <NuxtLink to="/">Home</NuxtLink>
                    <NuxtLink to="/about">About</NuxtLink>
                    <NuxtLink to="/membership">Membership</NuxtLink>
                    <NuxtLink to="/gallery">Gallery</NuxtLink>
                </span>
                <button v-else class="menu-btn button" @click="menuPressed">
                    <img class="menu-svg" src="/img/menu.svg"/>
                </button>
            </div>
        </header>
        <Transition name="menu">
            <div  v-if="menuToggle" class="mobile-menu flex-column">
                <div class="menu-links flex-column">
                    <NuxtLink @click="menuPressed" to="/">Home</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/about">About</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/membership">Membership</NuxtLink>
                    <div class="sep-line"></div>
                    <NuxtLink @click="menuPressed" to="/gallery">Gallery</NuxtLink>
                    <div class="sep-line"></div>
                </div>
                <p class="credit">Copyright © 2024 ICSM Badminton by <a class="author">Ethan Chang</a> | All Rights Reserved.</p>
            </div>
        </Transition>
        <div v-show="!menuToggle" class="page-container flex-column">
            <div>
                <slot />
            </div>
            <footer class="landing-footer flex-column">
                <div class="landing-footer-container flex-row">
                    <img v-if="!mobile" class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
                    <div class="landing-footer-columns flex-row">
                        <div v-if="!mobile" class="flex-column">
                            <h3 class="landing-footer-h3 landing-footer-link-cap">Site Map</h3>
                            <ul class="ul-no-dec">
                                <li><NuxtLink class="linkbutton" to="/">Home</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/about">About</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/membership">Membership</NuxtLink></li>
                                <li><NuxtLink class="linkbutton" to="/gallery">Gallery</NuxtLink></li>
                            </ul>
                        </div>
                        <div class="footer-2-column flex-column">
                            <NuxtLink class="linkbutton landing-footer-link-cap" to="/terms">Terms of Use</NuxtLink>
                            <NuxtLink class="linkbutton landing-footer-link-cap" to="/privacy">Privacy Policy</NuxtLink>
                        </div>
                    </div>
                </div>
                <p class="credit">Copyright © 2024 ICSM Badminton by <a class="author">Ethan Chang</a> | All Rights Reserved.</p>
            </footer>
        </div>
    </div>
</template>
