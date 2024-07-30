<script setup lang="ts">
const props = defineProps<{
    subtitle: string,
    paragraph: string
}>()
</script>

<template>
    <div class="content-box">
        <div class="row">
            <img src="/img/underground.svg"/>
            <h2 class="subtitle">{{ subtitle }}</h2>
        </div>
        <p class="text">{{ paragraph  }}</p>
    </div>
</template>
