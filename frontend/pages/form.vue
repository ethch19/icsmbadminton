<script setup lang="ts">
definePageMeta({
    layout: "form"
})

const vw = ref(null);
const mobile = ref(false);

function widthResized() {
    vw.value = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0);
    if (vw.value <= 480) {
        mobile.value = true;
    } else {
        mobile.value = false;
    }
}

onMounted(() => {
    widthResized();
    var doit;
    window.onresize = () => {
      clearTimeout(doit);
      doit = setTimeout(widthResized, 100);
    };
});

onUnmounted(() => {
    window.onresize = null;
});
</script>

<template>
    <main class="form-container flex-column">
        <div class="form-heading-container flex-column">
            <div class="form-heading-row flex-row">
                <img v-if="!mobile" class="form-logo" src="/img/document.svg" alt="Forms Icon">
                <h1 class="title form-heading">Team Training <wbr/>RSVP</h1>
            </div>
            <h2 class="form-subheading subtitle">Tuesday 25/10/2024</h2>
        </div>
        <div class="info-container flex-column">
            <p class="text info-text"><b>You must be a member of ICSM Badminton to sign up</b><br/><br/>Please wear suitable footwear and bring rackets along if you have one.<br/><br/>A limited number of rackets will be provided on the day.</p>
            <div class="info-buttons flex-row">
                <button class="button secondary-button">Minimise</button>
                <button class="button secondary-button">Privacy Policy</button>
                <button class="button secondary-button">Terms & Conditions</button>
            </div>
        </div>
        <div class="field-container flex-row">
            <FieldInput title="Name" placeholder="Your name" type="text"/>
            <FieldNumList title="CID" length=8 />
        </div>
        <button class="button primary-button">Submit</button>
    </main>
</template>

<style scoped>
@import url("~/assets/css/form.css");
</style>
