<template>
  <AfricansPanneau :titre="fiche?.nom ?? 'Pays'" icone="fa-solid fa-earth-africa">
    <div v-if="chargement" class="h-32 animate-pulse rounded-lg bg-af-bordure" />

    <div v-else-if="fiche" class="flex flex-col gap-4">
      <div class="flex items-center gap-4">
        <img
          v-if="fiche.iso2"
          :src="`https://flagcdn.com/${fiche.iso2}.svg`"
          alt=""
          class="h-8 w-12 shrink-0 rounded-sm border border-af-bordure object-cover"
        >
        <dl class="grid flex-1 grid-cols-3 gap-2 text-center">
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Rang</dt>
            <dd class="text-[18px]/[1.3] font-bold text-af-chocolat">{{ fiche.rang ?? '–' }}</dd>
          </div>
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Score</dt>
            <dd class="text-[18px]/[1.3] font-bold text-af-encre">{{ fiche.score }}</dd>
          </div>
          <div>
            <dt class="text-[12px]/[1.4] text-af-atone">Joueurs</dt>
            <dd class="text-[18px]/[1.3] font-bold text-af-encre">{{ fiche.joueurs }}</dd>
          </div>
        </dl>
      </div>

      <!-- Un pays sans joueur n'est ni masqué ni en erreur : il appelle le premier. -->
      <p v-if="fiche.joueurs === 0" class="text-[14px]/[1.5] text-af-corps">
        Aucun joueur encore pour ce pays : soyez le premier à le faire monter.
      </p>
      <ol v-else class="flex flex-col gap-2">
        <li v-for="j in fiche.meilleurs" :key="j.utilisateur_id" class="flex items-center gap-2 text-[14px]/[1.4]">
          <span class="w-5 text-center font-bold text-af-atone">{{ j.rang }}</span>
          <NuxtLink :to="`/profil/${j.utilisateur_id}`" class="min-w-0 flex-1 truncate text-af-encre hover:text-af-chocolat">
            {{ j.prenom }} {{ j.nom }}
          </NuxtLink>
          <span class="font-bold tabular-nums text-af-encre">{{ j.score }}</span>
        </li>
      </ol>

      <!-- Jouer sur ce pays : une partie dont toutes les épreuves portent sur lui -->
      <div class="flex flex-col gap-2 border-t border-af-bordure pt-4">
        <p class="text-[14px]/[1.4] font-bold text-af-encre">Jouer sur ce pays</p>
        <p v-if="!fiche.modules.some(m => m.disponible)" class="text-[13px]/[1.5] text-af-atone">
          Pas encore assez d'épreuves sur ce pays pour une partie entière.
        </p>
        <AfricansBouton
          v-for="m in fiche.modules.filter(x => x.disponible)"
          :key="m.code"
          variante="secondaire"
          icone="fa-solid fa-play"
          pleine-largeur
          :desactive="lancement"
          @click="$emit('jouer', { module: m.code, paysId: fiche.pays_id })"
        >
          {{ m.libelle }} · {{ m.epreuves_jouables }} épreuves
        </AfricansBouton>
        <p v-if="erreur" class="text-[13px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>
      </div>
    </div>
  </AfricansPanneau>
</template>

<script setup lang="ts">
import type { FichePaysJeuAPI } from '~/composables/useChampionship'

/** La fiche d'un pays choisi sur la carte (FR-065, FR-067). */
const props = defineProps<{
  paysId: string
  lancement?: boolean
  erreur?: string
}>()

defineEmits<{ jouer: [{ module: string, paysId: string }] }>()

const { fichePays } = useChampionship()
const fiche = ref<FichePaysJeuAPI | null>(null)
const chargement = ref(false)

watch(() => props.paysId, async (id) => {
  if (!id) return
  chargement.value = true
  try {
    fiche.value = await fichePays(id)
  }
  catch {
    fiche.value = null
  }
  finally {
    chargement.value = false
  }
}, { immediate: true })
</script>
