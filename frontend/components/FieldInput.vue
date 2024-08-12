<script setup lang="ts">
const props = defineProps<{
    title: string,
    placeholder: string,
    type: string,
}>();
</script>

<template>
    <div class="field-input flex-column">
        <label class="field-label title">{{ title + ":" }}</label>
        <input class="field" :type="type" :name="title" :placeholder="placeholder">
    </div>
</template>

<style scoped>
@import url("~/assets/css/fieldinput.css");
</style>
