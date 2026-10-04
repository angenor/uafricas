<template>
  <!-- Point d'entrée des activités dans le rail d'une page de module. Il ne
       s'affiche que si le module est ouvert au jeu : un module fermé par
       l'administration ne laisse aucune porte morte. -->
  <AfricansPanneau v-if="moduleJeu" titre="Activités" icone="fa-solid fa-gamepad">
    <p class="text-[14px]/[1.5] text-af-corps">
      <template v-if="moduleJeu.disponible">
        Testez vos connaissances : {{ moduleJeu.epreuves_jouables }}
        épreuve{{ moduleJeu.epreuves_jouables > 1 ? 's' : '' }} vous attendent.
      </template>
      <template v-else>
        Les épreuves de ce module arrivent bientôt.
      </template>
    </p>
    <AfricansBouton
      class="mt-4"
      pleine-largeur
      icone="fa-solid fa-play"
      :vers="`/activites/${module}`"
      :desactive="!moduleJeu.disponible"
    >
      Jouer
    </AfricansBouton>
  </AfricansPanneau>
</template>

<script setup lang="ts">
import type { ModuleJeuAPI } from '~/composables/useJeu'

const props = defineProps<{
  /** Code du module de jeu : `afrolang`, `codimoi`, `afripulse`, `factcheck`. */
  module: string
}>()

const { listerModules } = useJeu()
const moduleJeu = ref<ModuleJeuAPI | null>(null)

// Chargé côté client, hors du rendu serveur : le panneau est un accessoire du
// rail, il ne doit ni retarder ni faire échouer la page qui l'accueille.
onMounted(async () => {
  try {
    moduleJeu.value = (await listerModules()).find(m => m.code === props.module) ?? null
  }
  catch {
    moduleJeu.value = null
  }
})
</script>
