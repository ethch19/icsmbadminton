<script setup lang="ts">
import "~/assets/css/index.css"
</script>

<template>
    <header>
        <img class="logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
        <span>
            <NuxtLink to="/">Home</NuxtLink>
            <NuxtLink to="/">About</NuxtLink>
            <NuxtLink to="/">Membership</NuxtLink>
            <NuxtLink to="/">Teams</NuxtLink>
            <NuxtLink to="/">Socials</NuxtLink>
        </span>
    </header>
    <slot />
</template>
