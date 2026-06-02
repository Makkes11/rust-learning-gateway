<script setup lang="ts">
import { ref, onMounted } from "vue";
import { getDevices, type Device } from "./services/devices";

const devices = ref<Device[]>([]);
const selectedDevice = ref<Device | null>(null);

onMounted(async () => {
  devices.value = await getDevices();
});
</script>

<template>
  <div style="display: flex; gap: 20px;">
    
    <!-- LISTE -->
    <div>
      <h2>Devices</h2>

      <ul>
        <li
          v-for="d in devices"
          :key="d.id"
          @click="selectedDevice = d"
          style="cursor: pointer;"
        >
          {{ d.name }}
        </li>
      </ul>
    </div>

    <!-- DETAIL -->
    <div>
      <h2>Detail</h2>

      <div v-if="selectedDevice">
        <p>ID: {{ selectedDevice.id }}</p>
        <p>Name: {{ selectedDevice.name }}</p>
      </div>

      <div v-else>
        <p>Kein Device ausgewählt</p>
      </div>
    </div>

  </div>
</template>