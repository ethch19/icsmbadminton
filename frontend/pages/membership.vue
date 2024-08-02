<script setup lang="ts">
definePageMeta({
    layout: "default"
});

interface itemType {
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

const { data }  = await useAsyncData('memberships', () => queryContent('/membership').findOne());

const rawItems = data.value.body;
let items: Array<itemType> = [];  
rawItems.forEach((x) => {
    let content: Array<string> = [];
    let bold: Array<Array<string>> = [];
    for (const p of x.feature) {
        content.push(p.content);
        if (typeof p.bold != "undefined") {
            let tempBold: Array<string> = [];
            for (const q of p.bold) {
                tempBold.push(q);
            }
            bold.push(tempBold);
        } else {
            bold.push([""]);
        }
    }
    if (typeof x.header != "undefined") {
        let tempHeaderBold: Array<string> = [];
        for (const y of x.header.bold) {
            tempHeaderBold.push(y);
        }
        const newItem: itemType = {
            name: x.name,
            price: x.price,
            text: content,
            boldText: bold,
            link: x.link,
            headerText: x.header.text,
            headerBold: tempHeaderBold,
            headerColour: x.header.colour,
            headerBgColour: x.header.bgcolour,
        };
        items.push(newItem);
    } else {
        const newItem: itemType = {
            name: x.name,
            price: x.price,
            text: content,
            boldText: bold,
            link: x.link,
        };
        items.push(newItem);
    }
});
</script>

<template>
    <main>
        <h1 class="title text-center">Membership</h1>
        <div class="opt-container">
            <MemCard v-for="item in items" :name="item.name" :price="item.price" :text="item.text" :boldText="item.boldText" :headerText="item.headerText" :headerBold="item.headerBold" :headerColour="item.headerColour" :headerBgColour="item.headerBgColour" :link="item.link"/>
        </div>
    </main>
</template>

<style scoped>
@import url( "~/assets/css/membership.css");
</style>
