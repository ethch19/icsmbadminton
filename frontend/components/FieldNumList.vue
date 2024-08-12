<template>
    <div class="field-num-list flex-column">
        <label class="field-label title">{{ title + ":" }}</label>
        <div class="num-list flex-row">
            <input v-for="num in parseInt(length)" ref="numItems" @beforeinput="input" @keyup.left="direction" @keyup.right="direction" class="field">
        </div>
    </div>
</template>

<style scoped>
@import url("~/assets/css/fieldnumlist.css");
</style>

<script setup lang="ts">
const props = defineProps<{
    title: string,
    length: number,
}>();
const numItems = ref<HTMLInputElement | null>([]);

const input = (event) => {
    if (isNaN(event.data) || event.data == " ") {
        event.preventDefault();
        return;
    }
    let length = event.target.value.length;
    let index = numItems.value.indexOf(event.target);
    let nextIndex = numItems.value.findIndex((x, y) => (x.value.length == 0 && y > index));
    nextIndex = nextIndex == -1 ? numItems.value.findIndex((x, y) => (x.value.length == 0 && y != index)) : nextIndex;
    switch (length) {
        case 0:
            if (event.inputType == "deleteContentBackward") {
                event.preventDefault();
                index = index > 0 ? index-1 : 0;
                let range = numItems.value[index].value.length;
                numItems.value[index].focus();
                numItems.value[index].setSelectionRange(range, range);
                return;
            } else if (event.inputType == "deleteContentForward") {
                event.preventDefault();
                return;
            }
            event.preventDefault();
            event.target.value = event.data;
            if (nextIndex != -1) {
                numItems.value[nextIndex].focus();
            }
            return;
        case 1:
            if (event.inputType == "deleteContentBackward" || event.inputType == "deleteContentForward") {
                return;
            }
            event.preventDefault();
            if (nextIndex != -1) {
                numItems.value[nextIndex].value = event.data;
                numItems.value[nextIndex].focus();
            } else {
                event.target.value = event.data;
            }
            return;
    }
};

const direction = (event) => {
    let key = event.key;
    let index = numItems.value.indexOf(event.target);
    if (key == "ArrowLeft" && index != 0){
        numItems.value[index-1].focus();
    } else if (key == "ArrowRight" && index != props.length-1){
        let range = numItems.value[index+1].value.length;
        numItems.value[index+1].focus();
        numItems.value[index+1].setSelectionRange(range, range);
    }
}
</script>
