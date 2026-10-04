<template>
  <article class="flex flex-col gap-4 rounded-[10px] border border-af-bordure bg-af-surface p-5">
    <header class="flex items-start gap-3">
      <span class="grid size-11 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-af-chocolat">
        <font-awesome-icon :icon="defi.periodicite === 'jour' ? 'fa-solid fa-bolt' : 'fa-solid fa-calendar-days'" />
      </span>
      <div class="min-w-0 flex-1">
        <h3 class="text-[17px]/[1.4] font-bold text-af-encre">
          {{ defi.periodicite === 'jour' ? 'Défi du jour' : 'Défi de la semaine' }}
        </h3>
        <p v-if="defi.titre" class="truncate text-[14px]/[1.4] text-af-corps">{{ defi.titre }}</p>
        <p class="text-[13px]/[1.4] text-af-atone">
          {{ defi.nombre_epreuves }} épreuves · se termine dans {{ reste }}
        </p>
      </div>
    </header>

    <!-- Résultat du membre, une fois le défi terminé -->
    <p v-if="etat === 'termine'" class="text-[14px]/[1.5] text-af-corps">
      <font-awesome-icon icon="fa-solid fa-circle-check" class="text-af-vert" />
      Terminé : <strong>{{ defi.ma_partie?.bonnes }} / {{ defi.nombre_epreuves }}</strong>,
      +{{ defi.ma_partie?.score_gagne }} de score.
    </p>
    <p v-else class="text-[14px]/[1.5] text-af-corps">
      La même série pour tous, un seul essai.
    </p>

    <div class="mt-auto flex flex-wrap items-center gap-3">
      <AfricansBouton
        v-if="etat !== 'termine'"
        icone="fa-solid fa-play"
        :desactive="occupe"
        @click="$emit('jouer', defi)"
      >
        {{ etat === 'en_cours' ? 'Reprendre' : 'Relever le défi' }}
      </AfricansBouton>
      <AfricansBouton v-else variante="secondaire" :vers="`/activites/defis/${defi.id}`">
        Voir le classement
      </AfricansBouton>
    </div>
  </article>
</template>

<script setup lang="ts">
import { dureeRestante, type DefiAPI } from '~/composables/useJeu'

const props = defineProps<{
  defi: DefiAPI
  /** Heure du serveur à la lecture, pour dire le temps restant sans dépendre de l'horloge locale. */
  maintenant: string
  occupe?: boolean
}>()

defineEmits<{ jouer: [defi: DefiAPI] }>()

/** `a_jouer`, `en_cours` (commencé, pas fini) ou `termine`. */
const etat = computed(() => {
  const partie = props.defi.ma_partie
  if (!partie) return 'a_jouer'
  return partie.etat === 'en_cours' ? 'en_cours' : 'termine'
})

const reste = computed(() => dureeRestante(props.defi.fin_at, props.maintenant))
</script>
