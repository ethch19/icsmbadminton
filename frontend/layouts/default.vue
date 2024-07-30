<style>
@import url("~/assets/css/header.css");
</style>

<template>
    <header>
        <img class="logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
        <span>
            <NuxtLink to="/">Home</NuxtLink>
            <NuxtLink to="/">About</NuxtLink>
            <NuxtLink to="/membership">Membership</NuxtLink>
            <NuxtLink to="/">Gallery</NuxtLink>
        </span>
    </header>
    <slot />
</template>
