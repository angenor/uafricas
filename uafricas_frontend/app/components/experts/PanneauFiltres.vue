<template>
  <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="$emit('reset')">
    <div class="flex flex-col gap-5">
      <!-- Zone géographique -->
      <div class="flex flex-col gap-2">
        <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Zone</p>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="z in ZONES"
            :key="z.valeur"
            type="button"
            class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
            :class="zone === z.valeur ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
            :aria-pressed="zone === z.valeur"
            @click="zone = z.valeur"
          >
            {{ z.libelle }}
          </button>
        </div>
      </div>

      <!-- Territoire : la liste dépend de la zone -->
      <label class="flex flex-col gap-2">
        <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Territoire</span>
        <select
          v-model="selectedCountry"
          class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
        >
          <option value="">{{ libelleTousTerritoires }}</option>
          <option v-for="t in territoires" :key="t" :value="t">{{ t }}</option>
        </select>
      </label>

      <!-- Spécialités : celles réellement déclarées, jamais une liste inventée. -->
      <label v-if="specialites.length" class="flex flex-col gap-2">
        <span class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Spécialité</span>
        <select
          v-model="selectedSpecialty"
          class="h-10 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
        >
          <option value="">Toutes les spécialités</option>
          <option v-for="s in specialites" :key="s" :value="s">{{ s }}</option>
        </select>
      </label>

      <!-- Situation professionnelle -->
      <div class="flex flex-col gap-2">
        <p class="text-[12px]/[1.4] font-bold text-af-atone uppercase">Situation</p>
        <div class="flex flex-col">
          <button
            v-for="profil in PROFILS"
            :key="profil.id"
            type="button"
            class="flex items-center gap-3 rounded-[10px] px-3 py-2 text-left text-[14px]/[1.4] transition hover:bg-af-fond"
            :class="estActif(profil.id) ? 'font-bold text-af-chocolat' : 'text-af-corps'"
            @click="$emit('filtrerProfil', profil.id)"
          >
            <font-awesome-icon :icon="ICONES[profil.id] ?? 'fa-solid fa-user'" class="w-4 shrink-0" />
            <span class="min-w-0 flex-1">{{ profil.label }}</span>
          </button>
        </div>
      </div>
    </div>
  </AfricansPanneau>
</template>

<script setup lang="ts">
import { PROFILS_PROFESSIONNELS as PROFILS, PAYS_AFRIQUE, PAYS_HORS_AFRIQUE } from '~/composables/useExperts'

/**
 * Filtres de Diapertise, portés dans le rail du gabarit.
 *
 * Ce panneau remplace À LA FOIS `ExpertFilters` (colonne de gauche, desktop) et
 * `ExpertFiltersMobile` (tiroir + voile). Le gabarit empile déjà le rail sous
 * la colonne principale en dessous de 64rem : le tiroir mobile et son bouton
 * hamburger n'avaient plus d'objet, et les deux composants dupliquaient les
 * mêmes champs à deux endroits.
 *
 * Aucun critère n'est ajouté ni retiré : zone, territoire, spécialité et
 * situation, exactement ceux que l'API accepte.
 */
const props = defineProps<{
  selectedProfile: string
  /** Spécialités réellement déclarées par les experts (source : API). */
  specialites: string[]
}>()

defineEmits<{ filtrerProfil: [string], reset: [] }>()

const selectedCountry = defineModel<string>('selectedCountry', { default: '' })
const selectedSpecialty = defineModel<string>('selectedSpecialty', { default: '' })

const ZONES = [
  { valeur: 'tout' as const, libelle: 'Mondial' },
  { valeur: 'afrique' as const, libelle: 'Afrique' },
  { valeur: 'hors_afrique' as const, libelle: 'Hors Afrique' },
]
type ZoneTerritoire = (typeof ZONES)[number]['valeur']

const zone = defineModel<ZoneTerritoire>('zone', { default: 'tout' })

/** Icônes FontAwesome par situation (les constantes portent la syntaxe v5). */
const ICONES: Record<string, string> = {
  tous: 'fa-solid fa-users',
  recherche_emploi: 'fa-solid fa-magnifying-glass',
  en_emploi: 'fa-solid fa-briefcase',
  consultance: 'fa-solid fa-user-tie',
  volontariat_expertise: 'fa-solid fa-heart',
  recherche_nouvelles_opportunites: 'fa-solid fa-right-left',
}

/** « Tous les profils » vaut aussi pour l'absence de choix. */
const estActif = (id: string) =>
  id === 'tous' ? !props.selectedProfile || props.selectedProfile === 'tous' : props.selectedProfile === id

/** Territoires proposés selon la zone, triés alphabétiquement (fr). */
const territoires = computed(() => {
  const source = zone.value === 'tout'
    ? [...PAYS_AFRIQUE, ...PAYS_HORS_AFRIQUE]
    : zone.value === 'afrique' ? PAYS_AFRIQUE : PAYS_HORS_AFRIQUE
  return source.slice().sort((a, b) => a.localeCompare(b, 'fr'))
})

const libelleTousTerritoires = computed(() => {
  if (zone.value === 'tout') return 'Tous les territoires'
  return zone.value === 'afrique' ? 'Tous les territoires d\'Afrique' : 'Tous les territoires hors Afrique'
})

// Changer de zone réinitialise le territoire choisi (listes disjointes).
watch(zone, (nouvelle) => {
  if (nouvelle === 'tout') return
  selectedCountry.value = ''
})
</script>
