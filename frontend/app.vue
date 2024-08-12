<template>
    <NuxtLayout>
        <NuxtPage  /> 
    </NuxtLayout> 
</template>

<style>
.page-enter-active, .page-leave-active {
    transition: all 0.4s;
}
.page-enter-from, .page-leave-to {
    transform: translateY(-1rem);
    opacity: 0;
}
.dash-enter-active, .dash-leave-active {
    transition: all 0.4s;
}
.dash-enter-form, .dash-leave-to {
    opacity: 0;
}
.layout-enter-active, .layout-enter-active {
    transition: all 0.4s;
}
.layout-enter-from, .layout-leave-to {
    transform: translateY(-1rem);
    opacity: 0;
}
</style>
