<!-- App.vue - Główny komponent aplikacji -->
<template>
  <div id="app">
    <header class="app-header">
      <h1>🖥️ SD Helper</h1>
      <p class="subtitle">Zarządzanie połączeniami RDP / VNC / SSH</p>
    </header>

    <div class="app-content">
      <aside class="sidebar">
        <div class="sidebar-header">
          <button @click="showAddForm" class="btn btn-add">
            ➕ Dodaj hosta
          </button>
        </div>
        <HostList @edit="showEditForm" />
      </aside>

      <main class="main-content">
        <div v-if="showForm" class="form-container">
          <HostForm
            :host="editingHost"
            @submit="handleFormSubmit"
            @cancel="hideForm"
          />
        </div>

        <div v-else class="welcome">
          <div v-if="store.selectedHost" class="host-details">
            <h2>{{ store.selectedHost.name }}</h2>
            <div class="detail-row">
              <strong>Protokół:</strong>
              <span class="protocol-badge" :class="`protocol-${store.selectedHost.protocol}`">
                {{ store.selectedHost.protocol.toUpperCase() }}
              </span>
            </div>
            <div class="detail-row">
              <strong>Host:</strong>
              <span>{{ store.selectedHost.hostname }}:{{ store.selectedHost.port }}</span>
            </div>
            <div v-if="store.selectedHost.username" class="detail-row">
              <strong>Użytkownik:</strong>
              <span>{{ store.selectedHost.username }}</span>
            </div>
            <div v-if="store.selectedHost.category" class="detail-row">
              <strong>Kategoria:</strong>
              <span>{{ store.selectedHost.category }}</span>
            </div>
            <div v-if="store.selectedHost.notes" class="detail-row notes">
              <strong>Notatki:</strong>
              <p>{{ store.selectedHost.notes }}</p>
            </div>
            <button 
              @click="connectToSelected" 
              class="btn btn-connect-large"
            >
              🔌 Połącz z tym hostem
            </button>
          </div>

          <div v-else class="empty-state">
            <div class="icon">🖥️</div>
            <h2>Witaj w SD Helper!</h2>
            <p>Wybierz hosta z listy lub dodaj nowego, aby rozpocząć.</p>
          </div>
        </div>
      </main>
    </div>

    <footer class="app-footer">
      <span>v0.1.0 | {{ hostsCount }} hostów</span>
    </footer>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useHostsStore } from './store/hosts'
import HostList from './components/HostList.vue'
import HostForm from './components/HostForm.vue'

const store = useHostsStore()

const showForm = ref(false)
const editingHost = ref(null)

// Computed
const hostsCount = computed(() => store.hosts.length)

// Wczytaj hosty przy starcie
onMounted(async () => {
  await store.loadHosts()
})

// Pokaż formularz dodawania
const showAddForm = () => {
  editingHost.value = null
  showForm.value = true
}

// Pokaż formularz edycji
const showEditForm = (host) => {
  editingHost.value = host
  showForm.value = true
}

// Ukryj formularz
const hideForm = () => {
  showForm.value = false
  editingHost.value = null
}

// Obsługa submitu formularza
const handleFormSubmit = async (hostData) => {
  try {
    if (editingHost.value) {
      await store.updateHost(hostData)
    } else {
      await store.addHost(hostData)
    }
    hideForm()
  } catch (err) {
    alert('Błąd zapisu: ' + err)
  }
}

// Połącz z wybranym hostem
const connectToSelected = async () => {
  if (store.selectedHost) {
    try {
      await store.connectToHost(store.selectedHost.id)
    } catch (err) {
      alert('Błąd połączenia: ' + err)
    }
  }
}
</script>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

#app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #fafafa;
}

.app-header {
  background: linear-gradient(135deg, #4CAF50 0%, #45a049 100%);
  color: white;
  padding: 1rem 2rem;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.app-header h1 {
  margin: 0;
  font-size: 24px;
}

.subtitle {
  margin: 0.25rem 0 0 0;
  font-size: 13px;
  opacity: 0.9;
}

.app-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 400px;
  display: flex;
  flex-direction: column;
  background: white;
  border-right: 1px solid #e0e0e0;
}

.sidebar-header {
  padding: 1rem;
  border-bottom: 1px solid #e0e0e0;
}

.btn-add {
  width: 100%;
  padding: 0.75rem;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-add:hover {
  background: #45a049;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
}

.form-container {
  max-width: 600px;
  margin: 0 auto;
}

.welcome {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.empty-state {
  text-align: center;
  color: #999;
}

.empty-state .icon {
  font-size: 64px;
  margin-bottom: 1rem;
}

.empty-state h2 {
  color: #666;
  margin-bottom: 0.5rem;
}

.host-details {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  max-width: 600px;
}

.host-details h2 {
  margin-bottom: 1.5rem;
  color: #333;
}

.detail-row {
  display: flex;
  gap: 1rem;
  padding: 0.75rem 0;
  border-bottom: 1px solid #f0f0f0;
}

.detail-row strong {
  min-width: 120px;
  color: #666;
}

.detail-row.notes {
  flex-direction: column;
  gap: 0.5rem;
}

.detail-row.notes p {
  color: #666;
  line-height: 1.6;
}

.protocol-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
  color: white;
}

.protocol-rdp {
  background: #2196F3;
}

.protocol-vnc {
  background: #FF9800;
}

.protocol-ssh {
  background: #9C27B0;
}

.btn-connect-large {
  width: 100%;
  padding: 1rem;
  margin-top: 2rem;
  background: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  transition: background 0.2s;
}

.btn-connect-large:hover {
  background: #45a049;
}

.app-footer {
  padding: 0.75rem 2rem;
  background: white;
  border-top: 1px solid #e0e0e0;
  text-align: center;
  font-size: 12px;
  color: #999;
}
</style>
