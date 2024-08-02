<style scoped>
@import url("~/assets/css/headerfooter.css");
</style>

<template>
    <header class="landing-header">
        <img class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
        <span class="landing-header-container">
            <NuxtLink to="/">Home</NuxtLink>
            <NuxtLink to="/about">About</NuxtLink>
            <NuxtLink to="/membership">Membership</NuxtLink>
            <NuxtLink to="/gallery">Gallery</NuxtLink>
        </span>
    </header>
    <slot />
    <footer class="landing-footer flex-column">
        <div class="landing-footer-container flex-row">
            <img class="m-logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
            <div class="landing-footer-columns flex-row">
                <div class="flex-column">
                    <h3 class="landing-footer-h3 landing-footer-link-cap">Site Map</h3>
                    <ul class="ul-no-dec">
                        <li><NuxtLink class="linkbutton" to="/">Home</NuxtLink></li>
                        <li><NuxtLink class="linkbutton" to="/about">About</NuxtLink></li>
                        <li><NuxtLink class="linkbutton" to="/membership">Membership</NuxtLink></li>
                        <li><NuxtLink class="linkbutton" to="/gallery">Gallery</NuxtLink></li>
                    </ul>
                </div>
                <div class="flex-column">
                    <NuxtLink class="linkbutton landing-footer-link-cap" to="/terms">Terms of Use</NuxtLink>
                    <NuxtLink class="linkbutton landing-footer-link-cap" to="/privacy">Privacy Policy</NuxtLink>
                </div>
            </div>
        </div>
        <p class="credit">Copyright © 2024 ICSM Badminton by <a class="author">Ethan Chang</a> | All Rights Reserved.</p>
    </footer>
</template>
