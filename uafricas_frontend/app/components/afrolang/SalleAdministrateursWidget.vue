<script setup lang="ts">
// Widget public — Administrateurs d'une salle publique (Tailwind v4 pur).
// Distinct du badge « Admin plateforme ».
// Feature 001-admin-salles-publiques, US3 (FR-018).
import type { AdministrateurLight } from '~/composables/useAfrolang'

const props = defineProps<{
  administrateurs: AdministrateurLight[]
  taille?: 'sm' | 'md'
}>()

const visibles = computed(() => props.administrateurs.slice(0, 6))
const reste = computed(() => Math.max(0, props.administrateurs.length - visibles.value.length))

const isCompact = computed(() => props.taille === 'sm')
</script>

<template>
  <div v-if="administrateurs.length > 0" class="rounded-xl border border-custom-chocolat/20 bg-amber-50/50 px-4 py-3">
    <div class="flex items-center gap-2 mb-2">
      <span class="inline-flex items-center justify-center w-6 h-6 rounded-full bg-custom-chocolat/15 text-custom-chocolat text-xs font-bold">
        ★
      </span>
      <span :class="[isCompact ? 'text-xs' : 'text-sm', 'font-semibold uppercase tracking-wide text-custom-chocolat']">
        Administrateurs de la salle
      </span>
      <span :class="[isCompact ? 'text-xs' : 'text-xs', 'text-gray-500']">
        ({{ administrateurs.length }})
      </span>
    </div>

    <ul class="flex flex-wrap gap-2">
      <li
        v-for="a in visibles"
        :key="a.utilisateur_id"
        class="inline-flex items-center gap-2 rounded-full bg-white border border-custom-chocolat/20 px-2.5 py-1 shadow-sm"
        :title="`${a.prenom} ${a.nom} — administrateur de salle`"
      >
        <img
          v-if="a.photo_url"
          :src="a.photo_url"
          :alt="`${a.prenom} ${a.nom}`"
          class="w-6 h-6 rounded-full object-cover"
        />
        <span
          v-else
          class="inline-flex items-center justify-center w-6 h-6 rounded-full bg-custom-chocolat text-white text-[10px] font-bold"
        >
          {{ (a.prenom?.[0] || '') + (a.nom?.[0] || '') }}
        </span>
        <span :class="[isCompact ? 'text-xs' : 'text-sm', 'text-gray-800']">
          {{ a.prenom }} {{ a.nom }}
        </span>
      </li>
      <li
        v-if="reste > 0"
        class="inline-flex items-center rounded-full bg-white/70 border border-dashed border-custom-chocolat/30 px-3 py-1 text-xs text-custom-chocolat"
      >
        +{{ reste }}
      </li>
    </ul>
  </div>
</template>
