<!-- HostList.vue - Lista hostów -->
<template>
  <div class="host-list">
    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Wyszukaj hosta..."
        class="search-input"
      />
    </div>

    <div class="hosts-container">
      <div v-if="loading" class="loading">
        Wczytywanie hostów...
      </div>

      <div v-else-if="error" class="error">
        Błąd: {{ error }}
      </div>

      <div v-else-if="filteredHosts.length === 0" class="no-hosts">
        Brak hostów. Dodaj nowy host, aby rozpocząć.
      </div>

      <div v-else class="hosts-list">
        <HostItem
          v-for="host in filteredHosts"
          :key="host.id"
          :host="host"
          :selected="selectedHost?.id === host.id"
          @select="selectHost(host)"
          @connect="connectToHost(host.id)"
          @edit="$emit('edit', host)"
          @delete="deleteHost(host.id)"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useHostsStore } from '../store/hosts'
import HostItem from './HostItem.vue'

const store = useHostsStore()

// Computed properties z store
const loading = computed(() => store.loading)
const error = computed(() => store.error)
const filteredHosts = computed(() => store.filteredHosts)
const selectedHost = computed(() => store.selectedHost)

// Wyszukiwanie
const searchQuery = computed({
  get: () => store.searchQuery,
  set: (value) => store.setSearchQuery(value)
})

// Akcje
const selectHost = (host) => store.selectHost(host)
const connectToHost = async (hostId) => {
  try {
    await store.connectToHost(hostId)
  } catch (err) {
    alert('Błąd połączenia: ' + err)
  }
}
const deleteHost = async (hostId) => {
  if (confirm('Czy na pewno chcesz usunąć tego hosta?')) {
    try {
      await store.deleteHost(hostId)
    } catch (err) {
      alert('Błąd usuwania hosta: ' + err)
    }
  }
}

// Emitowane eventy
defineEmits(['edit'])
</script>

<style scoped>
.host-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
}

.search-bar {
  padding: 1rem;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.search-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: #4CAF50;
}

.hosts-container {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.hosts-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.loading, .error, .no-hosts {
  text-align: center;
  padding: 2rem;
  color: #666;
}

.error {
  color: #f44336;
}
</style>

