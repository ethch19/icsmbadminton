<script setup lang="ts">
import "~/assets/css/styles.css"
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
            <FieldCheckbox/>
            <button class="submit">Submit</button>
        </div>
    </main>
</template>
