<script lang="ts" setup>
import { Form } from "@primevue/forms";
import { invoke } from "@tauri-apps/api/core";
import { Button, InputText } from "primevue";
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
    <Form @submit="calculate_payments" class="flex justify-center flex-col gap-4 m-2 w-1/4">
        <InputText v-model="principal" inputmode="decimal", placeholder="Principal" />
        <InputText v-model="rate" inputmode="decimal", placeholder="Interest Rate" />
        <InputText v-model="numOfMonths" inputmode="decimal", placeholder="Number of Months" />
        <Button type="submit">Calculate</Button>
        
        <p>Monthly payment: ${{ monthlyPayments }}</p>
    </Form>
</template>