<template>
  <section
    class="rounded-[10px] border p-5"
    :class="ton.bordure"
    aria-live="polite"
  >
    <p class="flex items-center gap-2 text-[17px]/[1.4] font-bold" :class="ton.texte">
      <font-awesome-icon :icon="ton.icone" />
      {{ ton.titre }}
      <span v-if="correction.score_gagne > 0" class="ml-auto text-af-vert">
        +{{ correction.score_gagne }}
      </span>
    </p>

    <p v-if="correction.bon_pays && correction.issue !== 'bonne'" class="mt-3 text-[15px]/[1.5] text-af-encre">
      Le pays attendu : <strong>{{ correction.bon_pays.nom }}</strong>
    </p>

    <p v-if="correction.explication" class="mt-3 text-[15px]/[1.6] text-af-corps">
      {{ correction.explication }}
    </p>

    <div class="mt-4 flex flex-wrap items-center gap-x-6 gap-y-2">
      <NuxtLink
        v-if="correction.lien"
        :to="correction.lien"
        target="_blank"
        class="inline-flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat underline-offset-2 hover:underline"
      >
        <font-awesome-icon icon="fa-solid fa-book-open" />
        Voir le contenu d'origine
      </NuxtLink>
      <JeuSignalerEpreuve
        v-if="correction.signalable"
        :key="correction.epreuve_id"
        :epreuve-id="correction.epreuve_id"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import type { CorrectionAPI } from '~/composables/useJeu'

const props = defineProps<{ correction: CorrectionAPI }>()

const ton = computed(() => {
  switch (props.correction.issue) {
    case 'bonne':
      return {
        titre: 'Bonne réponse',
        icone: 'fa-solid fa-circle-check',
        texte: 'text-af-vert',
        bordure: 'border-af-vert/40 bg-af-vert/5',
      }
    case 'mauvaise':
      return {
        titre: 'Mauvaise réponse',
        icone: 'fa-solid fa-circle-xmark',
        texte: 'text-af-live',
        bordure: 'border-af-live/30 bg-af-live/5',
      }
    case 'injouable':
      return {
        titre: 'Épreuve passée, elle ne compte pas',
        icone: 'fa-solid fa-circle-exclamation',
        texte: 'text-af-corps',
        bordure: 'border-af-bordure bg-af-surface',
      }
    default:
      return {
        titre: 'Temps écoulé',
        icone: 'fa-solid fa-hourglass-half',
        texte: 'text-af-corps',
        bordure: 'border-af-bordure bg-af-surface',
      }
  }
})
</script>
