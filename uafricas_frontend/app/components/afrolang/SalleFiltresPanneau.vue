<template>
  <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="$emit('reset')">
    <div class="flex flex-col gap-5">
      <AfricansChamp v-model="langue" libelle="Langues" type="select">
        <option value="">Toutes les langues</option>
        <option v-for="l in langues" :key="l" :value="l">{{ l }}</option>
      </AfricansChamp>

      <div v-if="pays.length" class="flex flex-col gap-2">
        <p id="af-zone-territoire" class="text-[14px]/[1.4] text-af-atone italic">Territoires d'origine</p>

        <!-- La zone n'est pas qu'un filtre du menu déroulant : elle filtre
             aussi la liste des salles. Rendue en radiogroup : c'est un choix
             exclusif, et les flèches doivent passer d'une option à l'autre. -->
        <div role="radiogroup" aria-labelledby="af-zone-territoire" class="flex flex-wrap gap-2">
          <label
            v-for="option in ZONES_TERRITOIRE"
            :key="option.valeur"
            class="flex cursor-pointer items-center gap-2 rounded-lg border px-3 py-2 text-[12px]/[1.4] transition"
            :class="zone === option.valeur
              ? 'border-af-chocolat bg-af-chocolat/15 font-bold text-af-chocolat'
              : 'border-af-bordure bg-white text-af-corps hover:bg-af-chocolat/[0.07]'"
          >
            <input v-model="zone" type="radio" :value="option.valeur" class="sr-only" />
            {{ option.libelle }}
          </label>
        </div>

        <AfricansChamp v-model="paysId" libelle="Territoire" type="select">
          <option value="">Tous les territoires</option>
          <option v-for="p in territoiresDisponibles" :key="p.id" :value="p.id">{{ p.nom }}</option>
        </AfricansChamp>
      </div>

      <!-- Aucun bouton « Appliquer », contrairement à la maquette : les filtres
           s'appliquent à la sélection. Un bouton laisserait croire qu'un choix
           non validé n'a pas pris, alors que la liste a déjà changé. -->
      <p class="text-[12px]/[1.4] text-af-atone">
        {{ resultats }} salle{{ resultats > 1 ? 's' : '' }} sur {{ total }}
      </p>
    </div>
  </AfricansPanneau>
</template>

<script setup lang="ts">
import type { PaysOrigineLight, SalleFiltres } from '~/composables/useAfrolang'
import { PAYS_AFRICAINS_ISO2 } from '~/constants/afripulsePaysAutorises'

/**
 * Panneau de filtres du rail (maquette « Accueil Afrolang »). Reprend la
 * logique de `SalleFilters.vue`, dont il est la version portée sur le gabarit
 * de la refonte ; la recherche textuelle, elle, vit au-dessus dans le rail.
 */
const props = defineProps<{
  modelValue: SalleFiltres
  langues: string[]
  pays: PaysOrigineLight[]
  /** Nombre de salles au total, et nombre après filtrage. */
  total: number
  resultats: number
}>()

const emit = defineEmits<{
  'update:modelValue': [SalleFiltres]
  reset: []
}>()

const ZONES_TERRITOIRE = [
  { valeur: 'tout' as const, libelle: 'Mondial' },
  { valeur: 'afrique' as const, libelle: 'Afrique' },
  { valeur: 'hors_afrique' as const, libelle: 'Hors Afrique' }]

function maj(champs: Partial<SalleFiltres>) {
  emit('update:modelValue', { ...props.modelValue, ...champs })
}

const langue = computed({
  get: () => props.modelValue.langue ?? '',
  set: valeur => maj({ langue: valeur }),
})

const paysId = computed({
  get: () => props.modelValue.pays_id ?? '',
  set: valeur => maj({ pays_id: valeur }),
})

const zone = computed({
  get: () => props.modelValue.zone ?? 'tout',
  // Contenus disjoints : changer de zone vide le territoire déjà choisi, qui
  // ne figure plus dans le menu déroulant et filtrerait alors sans être visible.
  set: valeur => maj({ zone: valeur, pays_id: '' }),
})

const PAYS_AFRICAINS_SET = new Set<string>(PAYS_AFRICAINS_ISO2)

const estAfricain = (p: PaysOrigineLight): boolean =>
  !!p.code_iso2 && PAYS_AFRICAINS_SET.has(p.code_iso2.toLowerCase())

const territoiresDisponibles = computed(() => {
  if (zone.value === 'tout') return props.pays
  return props.pays.filter(p => (zone.value === 'afrique' ? estAfricain(p) : !estAfricain(p)))
})
</script>
