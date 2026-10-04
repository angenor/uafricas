<template>
  <!-- Concours rattachés à un module qui n'est pas (encore) jouable en quiz
       (feature 014, FR-057). Rien ne s'affiche sans concours ouvert : le rail
       ne gagne pas de panneau vide. -->
  <AfricansPanneau v-show="nombre > 0" titre="Concours" icone="fa-solid fa-images">
    <JeuConcoursEnCours :rattachement="rattachement" compact @charge="nombre = $event" />
  </AfricansPanneau>
</template>

<script setup lang="ts">
/** `rattachement` : le code du module dans `utils/modulesPlateforme.ts`. */
defineProps<{ rattachement: string }>()
const nombre = ref(0)
</script>
