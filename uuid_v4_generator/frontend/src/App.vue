<script setup lang="ts">
import { ref } from 'vue'

const uuid = ref<string | null>(null)
const createdAt = ref<string | null>(null)
const ipAddress = ref<string | null>(null)
const userAgent = ref<string | null>(null)
const loading = ref(false)
const fieldLoading = ref(false)

async function fetchUUID() {
  if (uuid.value) {
    fieldLoading.value = true
  } else {
    loading.value = true
  }

  uuid.value = null
  createdAt.value = null
  ipAddress.value = null
  userAgent.value = null

  try {
    await new Promise((resolve) => setTimeout(resolve, 1000))

    const res = await fetch('/api/v1/generate-uuid-v4')
    if (!res.ok) {
      throw new Error(`Error: ${res.statusText}`)
    }
    const data = await res.json()
    uuid.value = data.uuid
    createdAt.value = new Date(data.created_at).toLocaleString()
    ipAddress.value = data.ip_address
    userAgent.value = data.user_agent
  } catch (error) {
    uuid.value = `Failed to fetch: ${error}`
  } finally {
    loading.value = false
    fieldLoading.value = false
  }
}
</script>

<template>
    <div id="app">
      <h1>UUIDv4 Generator</h1>
      <button @click="fetchUUID" :disabled="loading || fieldLoading">
        {{ loading || fieldLoading ? 'Loading...' : uuid ? 'Re-generate UUID' : 'Generate UUID' }}
      </button>
      <div v-if="uuid || fieldLoading" class="result">
        <h2>Generated UUIDv4</h2>
        <div v-if="uuid && !uuid.startsWith('Failed to fetch')">
          <table>
            <tbody>
              <tr>
                <td><strong>UUIDv4:</strong></td>
                <td>{{ fieldLoading ? 'Loading...' : uuid }}</td>
              </tr>
              <tr>
                <td><strong>Created At:</strong></td>
                <td>{{ fieldLoading ? 'Loading...' : createdAt }}</td>
              </tr>
              <tr>
                <td><strong>IP Address:</strong></td>
                <td>{{ fieldLoading ? 'Loading...' : ipAddress }}</td>
              </tr>
              <tr>
                <td><strong>User Agent:</strong></td>
                <td>{{ fieldLoading ? 'Loading...' : userAgent }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <p v-else>Failed to generate new UUIDv4</p>
      </div>
      <p v-else>No UUIDv4 generated yet. Click the button to generate one.</p>
    </div>
</template>

<style scoped>
button {
  margin-top: 1rem;
}
.result {
  margin-top: 1rem;
  background-color: #f9f9f9;
  padding: 1rem;
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}
.result h2 {
  margin-bottom: 0.5rem;
}
.result table {
  width: 100%;
  border-collapse: collapse;
}
.result td {
  padding: 0.5rem;
  border-bottom: 1px solid #ddd;
}
.result td:first-child {
  font-weight: bold;
  width: 30%;
}
</style>