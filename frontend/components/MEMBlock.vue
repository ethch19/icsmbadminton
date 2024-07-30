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
    <div class="mem-container" :class="{ 'container-border': headerBool }" :style="{ 'border-color': headerBgColour }">
        <div v-if="headerBool" :style="{ 'border-color': headerBgColour, 'background-color': headerBgColour }" class="mem-header">
            <p v-html="nHeaderText" :style="{ 'color': headerColour }"></p>
        </div>
        <div class="mem-sub">
            <div class="mem-main">
                <h2>{{ name }}</h2>
                <p>£<span class="mem-price">{{ price }}</span>/yr</p>
            </div>
            <div class="sep-line"/>
            <div class="feature-container">
                <div v-for="textHtml in text" class="mem-feature">
                    <img src="/img/tick.svg"/>
                    <p v-html="textHtml"></p>
                </div>
            </div>
            <a :href="link" class="button">Buy</a>
        </div>
    </div>
</template>
