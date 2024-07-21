<template>
    <div class="summary-block">
        <p class="text stat-num">{{ num }}</p>
        <span class="column">
            <p class="text stat">{{ line1 }}</p>
            <p class="text stat">{{ line2 }}</p>
        </span>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    line1: string
    line2: string
    num: number
}>()
</script>
