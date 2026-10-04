<template>
  <section class="flex flex-col gap-4" aria-live="polite">
    <div v-if="chargement" class="h-80 animate-pulse rounded-[10px] bg-af-bordure" />

    <!-- Plus rien à départager pour ce votant -->
    <div v-else-if="tirage && 'termine' in tirage" class="rounded-[10px] border border-af-bordure bg-af-surface p-6 text-center">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-3xl text-af-vert" />
      <p class="mt-3 text-[17px]/[1.4] font-bold text-af-encre">
        {{ tirage.raison === 'plafond' ? 'Vous avez donné tous vos votes.' : 'Vous avez départagé toutes les paires.' }}
      </p>
      <p class="mt-1 text-[14px]/[1.5] text-af-corps">Merci ! Revenez à la clôture du vote pour découvrir le podium.</p>
    </div>

    <template v-else-if="tirage && 'id' in tirage">
      <div class="flex items-center justify-between gap-3">
        <p class="text-[16px]/[1.4] font-bold text-af-encre">Quelle photo préférez-vous ?</p>
        <p class="text-[13px]/[1.4] text-af-atone">
          {{ tirage.votes_exprimes }} vote{{ tirage.votes_exprimes > 1 ? 's' : '' }}<template v-if="tirage.votes_max"> sur {{ tirage.votes_max }}</template>
        </p>
      </div>

      <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <button
          v-for="cote in (['gauche', 'droite'] as const)"
          :key="tirage[cote].id"
          type="button"
          class="group flex flex-col overflow-hidden rounded-[10px] border-2 bg-af-surface text-left transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat disabled:opacity-60"
          :class="choisi === cote ? 'border-af-chocolat' : 'border-af-bordure hover:border-af-chocolat'"
          :disabled="envoi"
          :aria-label="`Choisir la photo de ${cote === 'gauche' ? 'gauche' : 'droite'}${tirage[cote].legende ? ' : ' + tirage[cote].legende : ''}`"
          @click="choisir(cote)"
        >
          <img :src="urlMedia(tirage[cote].media_url) ?? undefined" alt="" class="aspect-[4/3] w-full bg-af-fond object-contain">
          <span class="flex items-center gap-2 p-3 text-[14px]/[1.4] text-af-encre">
            <span class="flex-1">{{ tirage[cote].legende || 'Sans légende' }}</span>
            <span class="rounded-full bg-af-chocolat/10 px-3 py-1 text-[13px] font-bold text-af-chocolat group-hover:bg-af-chocolat group-hover:text-white">
              Je préfère
            </span>
          </span>
        </button>
      </div>
      <div class="flex justify-between">
        <JeuSignalerParticipation :key="`g-${tirage.gauche.id}`" :concours-id="concoursId" :participation-id="tirage.gauche.id" />
        <JeuSignalerParticipation :key="`d-${tirage.droite.id}`" :concours-id="concoursId" :participation-id="tirage.droite.id" />
      </div>
      <p class="hidden text-center text-[12px]/[1.4] text-af-atone sm:block">Au clavier : ← pour la photo de gauche, → pour celle de droite.</p>
    </template>

    <p v-if="erreur" class="text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>
  </section>
</template>

<script setup lang="ts">
import { messageErreurJeu } from '~/composables/useJeu'
import type { TirageAPI } from '~/composables/useConcours'

/**
 * Le vote à l'aveugle (feature 014, US5) : deux photos, sans auteur ni
 * décompte ; on choisit, la paire suivante arrive aussitôt. Le serveur décide
 * de tout (paire, équilibre, admissibilité de la voix) : ce composant
 * n'affiche que ce qu'il reçoit.
 */
const props = defineProps<{ concoursId: string }>()

const { confrontation, voter } = useConcours()

const tirage = ref<TirageAPI | null>(null)
const chargement = ref(true)
const envoi = ref(false)
const choisi = ref<'gauche' | 'droite' | null>(null)
const erreur = ref('')

const charger = async () => {
  try {
    tirage.value = await confrontation(props.concoursId)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Le vote n\'a pas pu être chargé.')
  }
  finally {
    chargement.value = false
  }
}

const choisir = async (cote: 'gauche' | 'droite') => {
  if (!tirage.value || !('id' in tirage.value) || envoi.value) return
  envoi.value = true
  choisi.value = cote
  erreur.value = ''
  try {
    tirage.value = await voter(props.concoursId, tirage.value.id, cote)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Votre vote n\'a pas pu être enregistré.')
  }
  finally {
    envoi.value = false
    choisi.value = null
  }
}

const clavier = (e: KeyboardEvent) => {
  const cible = e.target as HTMLElement | null
  if (cible && ['INPUT', 'TEXTAREA', 'SELECT'].includes(cible.tagName)) return
  if (e.key === 'ArrowLeft') choisir('gauche')
  else if (e.key === 'ArrowRight') choisir('droite')
}

onMounted(() => {
  charger()
  window.addEventListener('keydown', clavier)
})
onBeforeUnmount(() => window.removeEventListener('keydown', clavier))
</script>
