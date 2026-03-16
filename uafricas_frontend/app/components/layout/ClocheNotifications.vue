<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useNotifications } from '~/composables/useNotifications'
import { iconeNotification, couleurNotification } from '~/mocks/notifications'

const { nbNonLues, compteurNonLues, listerNotifications, marquerLue, toutMarquerLu } = useNotifications()

const ouvert = ref(false)
const notifications = ref<any[]>([])
const chargement = ref(false)

const charger = async () => {
  chargement.value = true
  try {
    const r = await listerNotifications()
    if (r.success && r.data) notifications.value = r.data
  } finally {
    chargement.value = false
  }
}

const toggle = async () => {
  ouvert.value = !ouvert.value
  if (ouvert.value && notifications.value.length === 0) await charger()
}

const surClicNotif = async (notif: any) => {
  if (!notif.lu) await marquerLue(notif.id)
  ouvert.value = false
  if (notif.lien_action) navigateTo(notif.lien_action)
}

const surToutLire = async () => {
  await toutMarquerLu()
  notifications.value = notifications.value.map((n: any) => ({ ...n, lu: true }))
}

const dateRelative = (date: string): string => {
  const diff = Date.now() - new Date(date).getTime()
  const minutes = Math.floor(diff / 60000)
  if (minutes < 60) return `il y a ${minutes}min`
  const heures = Math.floor(minutes / 60)
  if (heures < 24) return `il y a ${heures}h`
  const jours = Math.floor(heures / 24)
  return `il y a ${jours}j`
}

onMounted(compteurNonLues)
</script>

<template>
  <div class="relative">
    <!-- Cloche -->
    <button
      class="relative rounded-lg p-2 text-stone-500 transition-colors hover:bg-stone-100 hover:text-stone-700"
      @click="toggle"
    >
      <font-awesome-icon :icon="['fas', 'bell']" class="text-lg" />
      <span
        v-if="nbNonLues > 0"
        class="absolute -right-0.5 -top-0.5 flex h-5 w-5 items-center justify-center rounded-full bg-red-500 text-[10px] font-bold text-white"
      >
        {{ nbNonLues > 9 ? '9+' : nbNonLues }}
      </span>
    </button>

    <!-- Panneau déroulant -->
    <div
      v-if="ouvert"
      class="absolute right-0 top-full z-50 mt-2 w-80 rounded-xl border border-stone-200 bg-white shadow-xl"
    >
      <!-- En-tête -->
      <div class="flex items-center justify-between border-b border-stone-100 px-4 py-3">
        <h3 class="text-sm font-bold text-stone-800">Notifications</h3>
        <button
          v-if="nbNonLues > 0"
          class="text-xs text-[var(--color-custom-chocolat)] hover:underline"
          @click="surToutLire"
        >
          Tout marquer comme lu
        </button>
      </div>

      <!-- Liste -->
      <div class="max-h-80 overflow-y-auto">
        <div v-if="chargement" class="p-4 text-center text-sm text-stone-400">Chargement...</div>
        <div v-else-if="notifications.length === 0" class="p-6 text-center">
          <font-awesome-icon :icon="['fas', 'bell']" class="mb-2 text-2xl text-stone-300" />
          <p class="text-sm text-stone-400">Aucune notification</p>
        </div>
        <button
          v-for="notif in notifications.slice(0, 20)"
          :key="notif.id"
          :class="[
            'flex w-full items-start gap-3 border-b border-stone-50 px-4 py-3 text-left transition-colors last:border-b-0',
            notif.lu ? 'bg-white' : 'bg-blue-50/50',
          ]"
          @click="surClicNotif(notif)"
        >
          <font-awesome-icon
            :icon="['fas', iconeNotification(notif.type_)]"
            :class="['mt-0.5 text-sm', couleurNotification(notif.type_)]"
          />
          <div class="min-w-0 flex-1">
            <p :class="['text-sm', notif.lu ? 'text-stone-600' : 'font-semibold text-stone-800']">
              {{ notif.message }}
            </p>
            <p class="text-xs text-stone-400">{{ dateRelative(notif.created_at) }}</p>
          </div>
          <span v-if="!notif.lu" class="mt-1 h-2 w-2 shrink-0 rounded-full bg-blue-500" />
        </button>
      </div>

      <!-- Pied -->
      <div class="border-t border-stone-100 px-4 py-2">
        <NuxtLink
          to="/notifications"
          class="block text-center text-xs text-[var(--color-custom-chocolat)] hover:underline"
          @click="ouvert = false"
        >
          Voir toutes les notifications
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
