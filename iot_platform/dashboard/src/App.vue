<script setup lang="ts">
import { ref } from "vue";

type Device = {
  id: string;
  name: string;
};

const devices = ref<Device[]>([
  { id: "1", name: "Pump A" },
  { id: "2", name: "Motor B" },
  { id: "3", name: "Sensor C" }
]);
</script>

<template>
  <div>
    <h1>Devices</h1>

    <ul>
      <li v-for="d in devices" :key="d.id">
        {{ d.name }}
      </li>
    </ul>
  </div>
</template>