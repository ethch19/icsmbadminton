<script setup lang="ts">
definePageMeta({
    layout: "dash"
})

</script>

<template>
    <div class="page-container">
        <h1 class="home title">Summary</h1>
        <div class="home-container">
            <SummaryBlock line1="Total Responses" line2="Past 7 Days" :num=34 />
            <SummaryBlock line1="Responses" line2="Since Last Login" :num=3 />
        </div>
    </div>
</template>
