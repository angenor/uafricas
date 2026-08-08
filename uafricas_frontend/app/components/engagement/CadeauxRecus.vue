<script setup lang="ts">
/**
 * Cadeaux reçus par un contenu ou un profil — Tailwind v4 pur.
 *
 * **Aucun montant en argent n'est affiché**, et l'API n'en renvoie aucun : ce
 * que le public voit, c'est la reconnaissance reçue, pas ce qu'elle a coûté.
 * Seul le bénéficiaire connaît le cumul, sur sa cagnotte.
 */
import { onMounted, ref, watch } from 'vue'
import { useCadeaux, type CadeauxContenu } from '~/composables/useCadeaux'

const props = withDefaults(defineProps<{
  typeObjet: string
  objetId: string
  /** Version réduite : uniquement les pastilles et le total. */
  compact?: boolean
  /** Palette claire sur fond sombre — les 4 pages médias sont en `neutral-950`. */
  sombre?: boolean
}>(), { compact: false, sombre: false })

const { obtenirCadeauxContenu } = useCadeaux()
const cadeaux = ref<CadeauxContenu | null>(null)
const chargement = ref(true)

const charger = async () => {
  if (!props.objetId) return
  chargement.value = true
  try {
    cadeaux.value = await obtenirCadeauxContenu(props.typeObjet, props.objetId)
  } catch {
    cadeaux.value = null
  } finally {
    chargement.value = false
  }
}

watch(() => [props.typeObjet, props.objetId], charger)
onMounted(charger)

/** Rechargement à la demande, après qu'un cadeau vient d'être offert. */
defineExpose({ rafraichir: charger })

const formaterDate = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: 'short', year: 'numeric' })
</script>

<template>
  <!-- Rien plutôt qu'un « 0 cadeau » : un contenu sans cadeau n'a pas à
       exhiber son absence de soutien. -->
  <section v-if="!chargement && cadeaux && cadeaux.total > 0" class="space-y-3">
    <header class="flex flex-wrap items-baseline justify-between gap-2">
      <h3 class="font-oswald text-base font-bold" :class="sombre ? 'text-white' : 'text-gray-900'">
        <font-awesome-icon icon="fa-solid fa-gift" class="mr-1.5 text-custom-chocolat" />
        Cadeaux reçus
      </h3>
      <span class="text-xs" :class="sombre ? 'text-gray-400' : 'text-gray-500'">
        {{ cadeaux.total }} cadeau{{ cadeaux.total > 1 ? 'x' : '' }}
      </span>
    </header>

    <!-- Résumé par type -->
    <ul class="flex flex-wrap gap-2">
      <li
        v-for="r in cadeaux.resume"
        :key="r.code"
        class="inline-flex items-center gap-1.5 rounded-full px-3 py-1.5 text-xs"
        :class="sombre ? 'bg-white/10 text-gray-200' : 'bg-gray-100 text-gray-700'"
        :title="`${r.nombre} × ${r.libelle}`"
      >
        <font-awesome-icon
          :icon="`fa-solid fa-${r.icone || 'gift'}`"
          class="text-custom-chocolat"
        />
        <span class="font-medium">{{ r.libelle }}</span>
        <span class="font-oswald font-bold" :class="sombre ? 'text-white' : 'text-gray-900'">×{{ r.nombre }}</span>
      </li>
    </ul>

    <!-- Derniers offreurs -->
    <ul v-if="!compact && cadeaux.derniers.length > 0" class="space-y-1.5">
      <li
        v-for="(d, i) in cadeaux.derniers"
        :key="`${d.offreur.id}-${d.created_at}-${i}`"
        class="flex items-start gap-2 text-xs"
        :class="sombre ? 'text-gray-300' : 'text-gray-600'"
      >
        <font-awesome-icon
          :icon="`fa-solid fa-${d.cadeau.icone || 'gift'}`"
          class="mt-0.5 shrink-0 text-custom-chocolat"
        />
        <span class="min-w-0">
          <NuxtLink
            :to="`/profil/${d.offreur.id}`"
            class="font-medium transition hover:text-custom-chocolat"
            :class="sombre ? 'text-gray-100' : 'text-gray-800'"
          >{{ d.offreur.nom_affiche }}</NuxtLink>
          a offert {{ d.cadeau.libelle.toLowerCase() }}
          <span class="text-gray-400">· {{ formaterDate(d.created_at) }}</span>
          <em v-if="d.message" class="mt-0.5 block truncate text-gray-500">« {{ d.message }} »</em>
        </span>
      </li>
    </ul>
  </section>
</template>
