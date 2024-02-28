<template>
    <label class="field-label title">CID</label>
    <ol class="cidList" ref="cidList">
        <num-item
            v-for="(num, index) in numlist"
            @numInput="newInput"
            @numDirection="newDirection"
        ></num-item>
    </ol>
</template>

<script setup>
import { ref } from "vue";
import NumItem from "./NumItem.vue";

const numlist = ref(Array.from(Array(8).keys()));
const cidList = ref(null);

const newInput = (event) => {
    let numitems = Array.from(cidList.value.children).map(x => x.children[0]);
    if (event.target.value.length == 1 && event.data == null){
        return;
    }
    else if (event.target.value.length == 0 && event.data == null){
        const index = numitems.indexOf(event.target);
        if (index > 0){
            numitems[index-1].focus();
        }
    }
    else if (event.target.value.length == 1){
        for (let i=0; i < numitems.length; i++){
            if (numitems[i].value.length == 0){
                numitems[i].value = event.data;
                numitems[i].focus();
                event.preventDefault();
                return;
            }
        }
        event.preventDefault();
    }
    else if (event.target.value.length == 0){
        let index = null;
        for (let i=0; i < numitems.length; i++){
            if (numitems[i]==event.target){
                index = i;
                continue;
            }
            if (numitems[i].value.length == 0 && index!=null){
                event.preventDefault();
                event.target.value = event.data;
                numitems[i].focus();
                return;
            }
        }
    }
};

const newDirection = (event) => {
    let key = event.key;
    let numitems = Array.from(cidList.value.children); 
    for (let i=0; i < numitems.length; i++){
        if (numitems[i].children[0] == event.target){
            if (key == "ArrowLeft" && i!=0){
                numitems[i-1].children[0].focus();
            }
            else if (key == "ArrowRight" && i!=numitems.length-1){
                let input = numitems[i+1].children[0]
                input.focus();
                input.setSelectionRange(input.value.length,input.value.length);
            }
        }
    }
};
</script>
