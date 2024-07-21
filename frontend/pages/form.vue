<script setup lang="ts">
definePageMeta({
    layout: "forms"
})
</script>

<template>
    <main class="form">
        <FormTitle/>
        <FormInfo/>
        <div class="content">
            <FieldShort/>
            <FieldCID/>
            <button class="submit">Submit</button>
        </div>
    </main>
</template>
