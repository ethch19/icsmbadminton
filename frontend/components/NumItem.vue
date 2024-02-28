<template>
    <li>
        <input @beforeinput="handleInput" @keyup.left="direction" @keyup.right="direction" class="field">
    </li>
</template>

<script>
export default {
    methods: {
        handleInput(event){
            if (event.data == null){
                this.$emit("numInput", event);
            }
            else if (isNaN(event.data) || event.data == " "){
                event.preventDefault();
            }
            else {
                this.$emit("numInput", event);
            }
        },
        direction(event){ 
            this.$emit("numDirection", event);
        }
    }
};
</script>
