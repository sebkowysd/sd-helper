<!-- HostItem.vue - Pojedynczy element hosta na liście -->
<template>
  <div 
    class="host-item"
    :class="{ selected }"
    @click="$emit('select')"
  >
    <div class="host-info">
      <div class="host-header">
        <span class="protocol-badge" :class="`protocol-${host.protocol}`">
          {{ host.protocol.toUpperCase() }}
        </span>
        <h3 class="host-name">{{ host.name }}</h3>
      </div>
      
      <div class="host-details">
        <span class="hostname">{{ host.hostname }}:{{ host.port }}</span>
        <span v-if="host.username" class="username">{{ host.username }}</span>
        <span v-if="host.category" class="category">{{ host.category }}</span>
      </div>

      <div v-if="host.notes" class="host-notes">
        {{ host.notes }}
      </div>
    </div>

    <div class="host-actions">
      <button 
        @click.stop="$emit('connect')" 
        class="btn btn-connect"
        title="Połącz"
      >
        🔌 Połącz
      </button>
      <button 
        @click.stop="$emit('edit')" 
        class="btn btn-edit"
        title="Edytuj"
      >
        ✏️
      </button>
      <button 
        @click.stop="$emit('delete')" 
        class="btn btn-delete"
        title="Usuń"
      >
        🗑️
      </button>
    </div>
  </div>
</template>

<script setup>
defineProps({
  host: {
    type: Object,
    required: true
  },
  selected: {
    type: Boolean,
    default: false
  }
})

defineEmits(['select', 'connect', 'edit', 'delete'])
</script>

<style scoped>
.host-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: white;
  border: 2px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.host-item:hover {
  border-color: #4CAF50;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.host-item.selected {
  border-color: #4CAF50;
  background: #f1f8f4;
}

.host-info {
  flex: 1;
}

.host-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.protocol-badge {
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 10px;
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

.host-name {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.host-details {
  display: flex;
  gap: 1rem;
  font-size: 13px;
  color: #666;
  margin-bottom: 0.25rem;
}

.hostname {
  font-family: 'Courier New', monospace;
  color: #333;
}

.username, .category {
  color: #888;
}

.host-notes {
  font-size: 12px;
  color: #999;
  margin-top: 0.5rem;
  font-style: italic;
}

.host-actions {
  display: flex;
  gap: 0.5rem;
  margin-left: 1rem;
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-connect {
  background: #4CAF50;
  color: white;
}

.btn-connect:hover {
  background: #45a049;
}

.btn-edit {
  background: #f0f0f0;
  padding: 0.5rem 0.75rem;
}

.btn-edit:hover {
  background: #e0e0e0;
}

.btn-delete {
  background: #f0f0f0;
  padding: 0.5rem 0.75rem;
}

.btn-delete:hover {
  background: #ffcdd2;
}
</style>

