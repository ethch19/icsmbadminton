<template>
    <label class="field-label title">{{ title }}</label>
    <span class="checkbox-container">
        <p class="text">{{ text }}</p>
        <label class="checkbox">
            <input type="checkbox" checked>
            <span class="checked"></span>
        </label>
    </span>
</template>

<script>
import {ref} from 'vue'

export default {
    setup() {
        const title = ref("Registration")
        const text = ref("Yes, I'll be coming to the team training session on Tuesday") 
        
        return {
            title, text
        }
    }
}
</script> 
