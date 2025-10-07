<!-- HostForm.vue - Formularz dodawania/edycji hosta -->
<template>
  <div class="host-form">
    <h2>{{ isEdit ? 'Edytuj hosta' : 'Dodaj nowego hosta' }}</h2>

    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="name">Nazwa *</label>
        <input
          id="name"
          v-model="formData.name"
          type="text"
          required
          placeholder="np. Serwer produkcyjny"
        />
      </div>

      <div class="form-group">
        <label for="hostname">Host / IP *</label>
        <input
          id="hostname"
          v-model="formData.hostname"
          type="text"
          required
          placeholder="np. 192.168.1.100 lub server.example.com"
        />
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="protocol">Protokół *</label>
          <select id="protocol" v-model="formData.protocol" required>
            <option value="rdp">RDP</option>
            <option value="vnc">VNC</option>
            <option value="ssh">SSH</option>
          </select>
        </div>

        <div class="form-group">
          <label for="port">Port *</label>
          <input
            id="port"
            v-model.number="formData.port"
            type="number"
            required
            min="1"
            max="65535"
          />
        </div>
      </div>

      <div class="form-group">
        <label for="username">Nazwa użytkownika</label>
        <input
          id="username"
          v-model="formData.username"
          type="text"
          placeholder="Opcjonalne"
        />
      </div>

      <div class="form-group">
        <label for="category">Kategoria</label>
        <input
          id="category"
          v-model="formData.category"
          type="text"
          placeholder="np. Klient X, Produkcja, Test"
        />
      </div>

      <div class="form-group">
        <label for="notes">Notatki</label>
        <textarea
          id="notes"
          v-model="formData.notes"
          rows="3"
          placeholder="Dodatkowe informacje..."
        ></textarea>
      </div>

      <div class="form-actions">
        <button type="button" @click="handleCancel" class="btn btn-cancel">
          Anuluj
        </button>
        <button type="submit" class="btn btn-submit">
          {{ isEdit ? 'Zapisz zmiany' : 'Dodaj hosta' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  host: {
    type: Object,
    default: null
  }
})

const emit = defineEmits(['submit', 'cancel'])

const isEdit = ref(!!props.host)

// Domyślne porty dla protokołów
const defaultPorts = {
  rdp: 3389,
  vnc: 5900,
  ssh: 22
}

// Dane formularza
const formData = ref({
  id: props.host?.id || null,
  name: props.host?.name || '',
  hostname: props.host?.hostname || '',
  port: props.host?.port || defaultPorts.rdp,
  protocol: props.host?.protocol || 'rdp',
  username: props.host?.username || '',
  category: props.host?.category || '',
  notes: props.host?.notes || ''
})

// Zmień domyślny port przy zmianie protokołu
watch(() => formData.value.protocol, (newProtocol) => {
  if (!props.host) {
    formData.value.port = defaultPorts[newProtocol]
  }
})

// Obsługa submitu
const handleSubmit = () => {
  // Wyczyść puste pola opcjonalne
  const data = { ...formData.value }
  if (!data.username) data.username = null
  if (!data.category) data.category = null
  if (!data.notes) data.notes = null

  emit('submit', data)
}

// Obsługa anulowania
const handleCancel = () => {
  emit('cancel')
}
</script>

<style scoped>
.host-form {
  padding: 2rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

h2 {
  margin: 0 0 1.5rem 0;
  color: #333;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-row {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 1rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #555;
}

input, select, textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  font-family: inherit;
}

input:focus, select:focus, textarea:focus {
  outline: none;
  border-color: #4CAF50;
}

textarea {
  resize: vertical;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-cancel {
  background: #f0f0f0;
  color: #333;
}

.btn-cancel:hover {
  background: #e0e0e0;
}

.btn-submit {
  background: #4CAF50;
  color: white;
}

.btn-submit:hover {
  background: #45a049;
}
</style>

