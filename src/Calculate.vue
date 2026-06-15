<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

const principal = ref("");
const rate = ref("");
const numOfMonths = ref("");
const monthlyPayments = ref("");


async function calculate_payments() {
    monthlyPayments.value = await invoke("calculate_monthly_payment", {
        principal: principal.value,
        rate: rate.value,
        numberOfMonths: numOfMonths.value
    });
}

</script>

<template>
<form @submit.prevent="calculate_payments">
    <input v-model="principal" />
    <input v-model="rate" />
    <input v-model="numOfMonths" />
    <button type="submit">Calculate</button>

    <p>Monthly payment: ${{ monthlyPayments }}</p>
</form>
</template>