<template>
    <div class="summary-block">
        <p class="text stat-num">{{ num }}</p>
        <div class="flex-column">
            <p class="text stat">{{ line1 }}</p>
            <p class="text stat">{{ line2 }}</p>
        </div>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    line1: string
    line2: string
    num: number
}>()
</script>

<style scoped>
@import url("~/assets/css/summaryblock.css");
</style>
