<script setup lang="ts">
interface Props {
    name: string;
    price: string;
    text: Array<string>;
    boldText: Array<Array<string>>;
    link: string;
    headerText?: string;
    headerBold?: Array<string>;
    headerColour?: string;
    headerBgColour?: string;
};

const props = withDefaults(defineProps<Props>(), {
    headerColour: "none",
    headerBgColour: "none",
});

let features = ref<Array<string>>([]);
const startHtml = '<span class="text-bold">'
const endHtml = '</span>'

const headerBool = ref(true);
const nHeaderText = ref(props.headerText);

if (typeof props.headerText != "undefined") {
    if (typeof props.headerBold != "undefined") {
        for (const i of props.headerBold) {
            const j = "".concat(startHtml,i,endHtml);
            nHeaderText.value = nHeaderText.value.replaceAll(i,j);
        }
    }
} else {
    headerBool.value = false;
}

for (const i in props.text) {
    for (const j of props.boldText[i]) {
        const p = "".concat(startHtml,j,endHtml);
        props.text[i] = props.text[i].replaceAll(j, p);
    }
}
</script>

<template>
    <div class="mem-container flex-column" :class="{ 'container-border': headerBool }" :style="{ 'border-color': headerBgColour }">
        <div v-if="headerBool" :style="{ 'border-color': headerBgColour, 'background-color': headerBgColour }" class="mem-header">
            <p class="mem-header-title text" v-html="nHeaderText" :style="{ 'color': headerColour }"></p>
        </div>
        <div class="mem-sub">
            <div class="mem-main flex-column">
                <h3 class="subhead">{{ name }}</h3>
                <h3 class="mem-main-price subhead">£<span class="mem-price">{{ price }}</span>/yr</h3>
            </div>
            <div class="sep-line"/>
            <div class="feature-container flex-column">
                <div v-for="textHtml in text" class="mem-feature">
                    <img class="feature-img" src="/img/tick.svg"/>
                    <p class="text" v-html="textHtml"></p>
                </div>
            </div>
            <a :href="link" target="_blank" class="primary-button">Buy</a>
        </div>
    </div>
</template>

<style scoped>
@import url("~/assets/css/memcard.css");
</style>
