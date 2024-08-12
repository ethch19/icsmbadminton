<script setup lang="ts">
const route = useRoute()

const Home = computed(() => {
    return route.path == "/dash" ? true : false;
})
const SignUp = computed(() => {
    return route.path == "/signup" ? true : false;
})
const Settings = computed(() => {
    return route.path == "/settings" ? true : false;
})

const vw = ref(null);
const mobile = ref(false);
const LogoutText = ref("");

function widthResized() {
    vw.value = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
    if (vw.value <= 480) {
        mobile.value = true;
        LogoutText.value = "";
    } else {
        mobile.value = false;
        LogoutText.value = "Logout";
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

<style scoped>
@import url("~/assets/css/dash-headerfooter.css");
</style>

<template>
    <div class="admin-container flex-column">
        <header class="header flex-row">
            <div class="header-container flex-row">
                <img class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
                <div class="header-title-container flex-column">
                    <h2 class="subtitle header-subtitle">ICSM Badminton</h2>
                    <h1 class="title header-heading">Admin Panel</h1>
                </div>
            </div>
            <button class="button tertiary-button logout-btn" :class="{ 'logout-mobile-btn': mobile }">{{ LogoutText }}</button>
        </header>
        <div class="dash-container">
            <div class="sidebar flex-column">
                <div class="flex-column sidebar-buttons">
                    <NuxtLink :class="{ current: Home }" class="sidebar-link tertiary-button button" to="/dash">
                        <svg v-if="mobile" :class="{ 'current-svg': Home }" class="sidebar-svg" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">
                            <use :href="'/img/home.svg#a'"/>
                        </svg>
                        <span>Home</span>
                    </NuxtLink>
                    <NuxtLink :class="{ current: SignUp }" class="sidebar-link tertiary-button button" to="/signup">
                        <svg v-if="mobile" :class="{ 'current-svg': SignUp }" class="sidebar-svg">
                            <use :href="'/img/database.svg#a'"/>
                        </svg>
                        <span>Responses</span>
                    </NuxtLink>
                    <NuxtLink :class="{ current: Settings }" class="sidebar-link tertiary-button button" to="/settings">
                        <svg v-if="mobile" :class="{ 'current-svg': Settings }" class="sidebar-svg">
                            <use :href="'/img/setting.svg#a'"/>
                        </svg>
                        <span>Settings</span>
                    </NuxtLink>
                </div>
                <p v-if="!mobile" class="credit">© 2024 ICSM Badminton<br/>By <a class="author">Ethan Chang</a> | All Rights Reserved</p>
            </div>
            <div class="page-container">
                <slot/>
            </div>
        </div>
    </div>
</template>
