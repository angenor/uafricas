<template>
  <!-- Grille du hub, ou ligne compacte dans le rail d'un module. Rien ne
       s'affiche s'il n'y a aucun concours ouvert : pas de bloc vide. -->
  <section v-if="concours.length && !compact" class="flex flex-col gap-4">
    <div class="flex items-end justify-between gap-3">
      <h2 class="text-[20px]/[1.4] font-bold text-af-encre">Concours en cours</h2>
      <NuxtLink to="/activites/concours" class="text-[14px] font-bold text-af-chocolat hover:underline">Tous les concours</NuxtLink>
    </div>
    <div class="grid gap-4 sm:grid-cols-2">
      <JeuCarteConcours v-for="c in concours.slice(0, 4)" :key="c.id" :concours="c" />
    </div>
  </section>

  <ul v-else-if="concours.length" class="flex flex-col gap-2" role="list">
    <li v-for="c in concours.slice(0, 2)" :key="c.id">
      <NuxtLink :to="`/activites/concours/${c.id}`" class="flex items-start gap-2 text-[14px]/[1.45] text-af-encre hover:text-af-chocolat">
        <font-awesome-icon icon="fa-solid fa-images" class="mt-1 text-af-chocolat" />
        <span><strong>{{ c.titre }}</strong> · {{ c.phase === 'appel' ? 'déposez votre photo' : 'votez' }}</span>
      </NuxtLink>
    </li>
  </ul>
</template>

<script setup lang="ts">
import type { ConcoursPublicAPI } from '~/composables/useConcours'

/** Les concours ouverts (appel ou vote), éventuellement ceux d'un module. */
const props = defineProps<{ rattachement?: string, compact?: boolean }>()
const emit = defineEmits<{ charge: [nombre: number] }>()

const { lister } = useConcours()
const concours = ref<ConcoursPublicAPI[]>([])

// Côté client seulement : un accessoire de page ne doit ni la retarder ni la
// faire échouer.
onMounted(async () => {
  try {
    concours.value = (await lister({ phase: 'en_cours', rattachement: props.rattachement, taille: 4 }))?.elements ?? []
  }
  catch {
    concours.value = []
  }
  emit('charge', concours.value.length)
})
</script>
