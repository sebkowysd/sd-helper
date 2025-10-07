// store/hosts.js - Store Pinia dla zarządzania hostami
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export const useHostsStore = defineStore('hosts', {
  state: () => ({
    hosts: [],
    loading: false,
    error: null,
    searchQuery: '',
    selectedHost: null,
  }),

  getters: {
    // Filtrowane hosty na podstawie wyszukiwania
    filteredHosts: (state) => {
      if (!state.searchQuery) return state.hosts
      
      const query = state.searchQuery.toLowerCase()
      return state.hosts.filter(host => 
        host.name.toLowerCase().includes(query) ||
        host.hostname.toLowerCase().includes(query) ||
        (host.category && host.category.toLowerCase().includes(query))
      )
    },

    // Hosty pogrupowane po protokole
    hostsByProtocol: (state) => {
      const groups = {}
      state.hosts.forEach(host => {
        if (!groups[host.protocol]) {
          groups[host.protocol] = []
        }
        groups[host.protocol].push(host)
      })
      return groups
    },

    // Hosty pogrupowane po kategorii
    hostsByCategory: (state) => {
      const groups = { 'Bez kategorii': [] }
      state.hosts.forEach(host => {
        const category = host.category || 'Bez kategorii'
        if (!groups[category]) {
          groups[category] = []
        }
        groups[category].push(host)
      })
      return groups
    },
  },

  actions: {
    // Wczytuje hosty z backendu
    async loadHosts() {
      this.loading = true
      this.error = null
      try {
        this.hosts = await invoke('load_hosts')
      } catch (err) {
        this.error = err
        console.error('Błąd wczytywania hostów:', err)
      } finally {
        this.loading = false
      }
    },

    // Zapisuje hosty do backendu
    async saveHosts() {
      this.loading = true
      this.error = null
      try {
        await invoke('save_hosts', { hosts: this.hosts })
      } catch (err) {
        this.error = err
        console.error('Błąd zapisywania hostów:', err)
      } finally {
        this.loading = false
      }
    },

    // Dodaje nowy host
    async addHost(host) {
      this.loading = true
      this.error = null
      try {
        // Generuj ID dla nowego hosta
        host.id = Date.now().toString()
        this.hosts = await invoke('add_host', { host })
      } catch (err) {
        this.error = err
        console.error('Błąd dodawania hosta:', err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Aktualizuje istniejący host
    async updateHost(host) {
      this.loading = true
      this.error = null
      try {
        this.hosts = await invoke('update_host', { host })
        if (this.selectedHost && this.selectedHost.id === host.id) {
          this.selectedHost = host
        }
      } catch (err) {
        this.error = err
        console.error('Błąd aktualizacji hosta:', err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Usuwa host
    async deleteHost(hostId) {
      this.loading = true
      this.error = null
      try {
        this.hosts = await invoke('delete_host', { hostId })
        if (this.selectedHost && this.selectedHost.id === hostId) {
          this.selectedHost = null
        }
      } catch (err) {
        this.error = err
        console.error('Błąd usuwania hosta:', err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Łączy się z hostem
    async connectToHost(hostId) {
      this.error = null
      try {
        await invoke('connect_to_host', { hostId })
      } catch (err) {
        this.error = err
        console.error('Błąd połączenia:', err)
        throw err
      }
    },

    // Ustawia wyszukiwane zapytanie
    setSearchQuery(query) {
      this.searchQuery = query
    },

    // Wybiera host
    selectHost(host) {
      this.selectedHost = host
    },

    // Czyści wybranego hosta
    clearSelection() {
      this.selectedHost = null
    },
  },
})

