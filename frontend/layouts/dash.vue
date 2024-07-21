<script setup lang="ts">
import "~/assets/css/dash.css"
const route = useRoute()

const Home = computed(() => {
    return route.path == "/home" ? "current" : ""
})
const SignUp = computed(() => {
    return route.path == "/signup" ? "current" : ""
})
const Settings = computed(() => {
    return route.path == "/settings" ? "current" : ""
})
</script>

<template>
    <header>
        <div class="row">
            <img class="logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
            <span class="column">
                <h2>ICSM Badminton</h2>
                <h3>Admin Panel</h3>
            </span>
        </div>
        <button>Logout</button>
    </header>
    <div class="dash-container">
        <div class="sidebar">
            <span class="column">
                <NuxtLink :class="Home" to="/home">Home</NuxtLink>
                <NuxtLink :class="SignUp" to="/signup">Sign-Up Forms</NuxtLink>
                <NuxtLink :class="Settings" to="/settings">Settings</NuxtLink>
            </span>
            <p class="text footer">© 2024 ICSM Badminton<br/>By <span class="author">Ethan Chang</span> | All Rights Reserved</p>
        </div>
        <slot/>
    </div>
</template>
