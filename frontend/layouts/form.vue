<template>
    <div class="form">
        <header class="header flex-row">
            <img class="logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
            <h2 class="title header-title">ICSM Badminton</h2>
            <img class="logo" src="/img/icsm-badminton-logo.png" alt="ICSM Badminton Logo">
        </header>
        <slot/>
        <footer class="footer flex-column">
             <p class="credit">© 2024 ICSM Badminton by <a class="author">Ethan Chang</a> | All Rights Reserved</p>
        </footer>
    </div>
</template>

<style scoped>
@import url("~/assets/css/form-headerfooter.css");
</style>
