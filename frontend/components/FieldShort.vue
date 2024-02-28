<template>
    <label class="field-label title">{{title}}</label>
    <input class="field" type="text" name="name" :placeholder="placeholder">
</template>

<script>
import {ref} from 'vue'

export default {
    setup() {
        const title = ref("Name") 
        const placeholder = reactive("Your Name")
        return {
            title, placeholder
        }
    }
}
</script>
