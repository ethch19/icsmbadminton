<script setup lang="ts">
definePageMeta({
    layout: "dash",
    pageTransition: {
        name: "dash",
    },
})
</script>

<template>
    <div class="page-container flex-column">
        <h1 class="title">Summary</h1>
        <div class="home-container flex-row">
            <SummaryBlock line1="Total Responses" line2="Past 7 Days" :num=34 />
            <SummaryBlock line1="Responses" line2="Since Last Login" :num=3 />
            <SummaryBlock line1="Page Visits" line2="Past 7 days" :num=5 />
        </div>
    </div>
</template>

<style scoped>
@import url("~/assets/css/dash-page.css");
</style>
