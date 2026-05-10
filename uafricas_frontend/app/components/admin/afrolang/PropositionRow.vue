<script setup lang="ts">
// Ligne de tableau pour une proposition de salle (admin) — feature 001-admin-salles-publiques US2
import type { PropositionSalle } from '~/composables/useAfrolang'

defineProps<{
  proposition: PropositionSalle
}>()

const formatDate = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', {
    day: '2-digit', month: '2-digit', year: 'numeric',
  })

const badgeClass = (statut: string) => {
  switch (statut) {
    case 'en_attente': return 'badge badge-warning badge-sm'
    case 'validee': return 'badge badge-success badge-sm'
    case 'rejetee': return 'badge badge-error badge-sm'
    case 'retiree': return 'badge badge-neutral badge-sm'
    default: return 'badge badge-sm'
  }
}

const statutLibelle = (statut: string) => {
  const libelles: Record<string, string> = {
    en_attente: 'En attente',
    validee: 'Validée',
    rejetee: 'Rejetée',
    retiree: 'Retirée',
  }
  return libelles[statut] || statut
}
</script>

<template>
  <tr>
    <td class="font-medium">
      <NuxtLink
        :to="`/admin/afrolang/propositions/${proposition.id}`"
        class="link link-hover"
      >
        {{ proposition.titre }}
      </NuxtLink>
    </td>
    <td>{{ proposition.auteur.prenom }} {{ proposition.auteur.nom }}</td>
    <td>{{ proposition.groupe_ethnique.nom }}</td>
    <td>{{ proposition.langue_cible }}</td>
    <td><span :class="badgeClass(proposition.statut)">{{ statutLibelle(proposition.statut) }}</span></td>
    <td class="text-sm text-base-content/70">{{ formatDate(proposition.created_at) }}</td>
    <td>
      <NuxtLink
        :to="`/admin/afrolang/propositions/${proposition.id}`"
        class="btn btn-ghost btn-xs"
      >
        <font-awesome-icon icon="eye" />
      </NuxtLink>
    </td>
  </tr>
</template>
