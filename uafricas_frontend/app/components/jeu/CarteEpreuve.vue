<template>
  <article class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
    <h2 class="text-[20px]/[1.4] font-bold text-af-encre">{{ epreuve.enonce }}</h2>

    <!-- Média de l'énoncé. Un média qui ne se charge pas n'est pas compté contre
         le membre : il peut le déclarer, l'épreuve est alors consommée sans rien
         rapporter. -->
    <div v-if="epreuve.media_type && source" class="mt-5">
      <img
        v-if="epreuve.media_type === 'image'"
        :src="source"
        alt=""
        class="mx-auto max-h-64 rounded-lg border border-af-bordure object-contain"
        @error="mediaEnErreur = true"
      >
      <audio
        v-else
        :src="source"
        controls
        preload="auto"
        class="w-full"
        @error="mediaEnErreur = true"
      >
        Votre navigateur ne lit pas cet extrait sonore.
      </audio>

      <p
        v-if="mediaEnErreur && !verrouillee"
        class="mt-3 flex flex-wrap items-center gap-3 rounded-lg border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-encre"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-af-live" />
        <span class="flex-1">Ce média ne se charge pas.</span>
        <button
          type="button"
          class="font-bold text-af-chocolat underline-offset-2 hover:underline"
          @click="$emit('injouable')"
        >
          Passer cette épreuve
        </button>
      </p>
    </div>

    <ul class="mt-6 flex flex-col gap-3" role="list">
      <li v-for="proposition in epreuve.propositions" :key="proposition.cle">
        <button
          type="button"
          class="flex w-full items-center gap-3 rounded-lg border px-4 py-3 text-left text-[16px]/[1.4] transition focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-af-chocolat"
          :class="classesProposition(proposition.cle)"
          :disabled="verrouillee"
          :aria-pressed="choix === proposition.cle"
          @click="$emit('choisir', proposition.cle)"
        >
          <span class="flex-1">{{ proposition.texte }}</span>
          <font-awesome-icon
            v-if="bonneCle === proposition.cle"
            icon="fa-solid fa-circle-check"
            class="text-af-vert"
          />
          <font-awesome-icon
            v-else-if="bonneCle != null && choix === proposition.cle"
            icon="fa-solid fa-circle-xmark"
            class="text-af-live"
          />
        </button>
      </li>
    </ul>
  </article>
</template>

<script setup lang="ts">
import type { EpreuveServieAPI } from '~/composables/useJeu'

/**
 * L'épreuve telle qu'elle se joue. Le composant ne connaît la bonne réponse
 * qu'APRÈS la correction (`bonneCle`) : avant, il n'a rien à cacher, le
 * serveur ne la lui a pas donnée.
 */
const props = defineProps<{
  epreuve: EpreuveServieAPI
  /** Proposition choisie par le membre, une fois envoyée. */
  choix?: number | null
  /** Renseignée par la correction : colore la bonne et la mauvaise. */
  bonneCle?: number | null
  /** Plus aucun clic : réponse en vol, ou correction affichée. */
  verrouillee?: boolean
}>()

defineEmits<{ choisir: [cle: number], injouable: [] }>()

const mediaEnErreur = ref(false)
watch(() => props.epreuve.id, () => { mediaEnErreur.value = false })

const source = computed(() => urlMedia(props.epreuve.media_url))

const classesProposition = (cle: number) => {
  if (props.bonneCle != null) {
    if (cle === props.bonneCle) return 'border-af-vert bg-af-vert/10 font-bold text-af-encre'
    if (cle === props.choix) return 'border-af-live bg-af-live/5 text-af-encre'
    return 'border-af-bordure text-af-atone'
  }
  if (cle === props.choix) return 'border-af-chocolat bg-af-chocolat/[0.07] font-bold text-af-encre'
  return props.verrouillee
    ? 'border-af-bordure text-af-atone'
    : 'border-af-bordure text-af-encre hover:border-af-chocolat hover:bg-af-chocolat/[0.04]'
}
</script>
