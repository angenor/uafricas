<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Factcheck"
    sous-titre="Vérifier l'info et défendre la vérité sur l'Afrique"
    icone="fa-solid fa-scale-balanced"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Sur Internet, beaucoup d'idées reçues et de fausses informations circulent sur l'Afrique.
          <strong class="font-bold text-af-encre">Factcheck</strong> est un espace où l'on vérifie
          <strong class="font-bold text-af-encre">ensemble</strong> les contenus qui circulent, 
          articles, images, vidéos, rumeurs, pour distinguer le vrai du faux et raconter une
          Afrique <strong class="font-bold text-af-encre">fiable, nuancée et vérifiée</strong>.
        </p>

        <div v-else-if="etape === 1" class="flex flex-col gap-6">
          <div class="flex flex-col gap-3">
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Ce que vous pouvez y faire</h3>
            <div class="grid gap-3 sm:grid-cols-2">
              <div
                v-for="item in FONCTIONNALITES"
                :key="item.titre"
                class="flex gap-3 rounded-[10px] border border-af-bordure p-4"
              >
                <font-awesome-icon :icon="item.icone" class="mt-0.5 size-6 shrink-0 text-af-vert" />
                <div class="min-w-0">
                  <p class="text-[14px]/[1.4] font-bold">{{ item.titre }}</p>
                  <p class="mt-1 text-[12px]/[1.4] text-af-corps">{{ item.texte }}</p>
                </div>
              </div>
            </div>
          </div>

          <div class="flex flex-col gap-3">
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Les outils à votre disposition</h3>
            <div class="flex flex-wrap gap-2">
              <AfricansEtiquette v-for="outil in OUTILS" :key="outil">{{ outil }}</AfricansEtiquette>
            </div>
          </div>
        </div>

        <div v-else class="flex flex-col gap-3">
          <h3 class="flex items-center gap-3 text-[17px]/[1.4] font-bold text-af-vert">
            <font-awesome-icon icon="fa-solid fa-seedling" class="size-6" />
            Notre objectif
          </h3>
          <p class="text-[14px]/[1.4] text-af-corps">
            Faire reculer la désinformation et les clichés sur l'Afrique, renforcer l'esprit critique
            de chacun et donner à voir une image plus juste du continent.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/** « C'est quoi Factcheck ? » : contenu repris de la modale inline de la page. */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-users', titre: 'Vérifier ensemble', texte: 'Passez au crible articles, images, vidéos et rumeurs, à plusieurs.' },
  { icone: 'fa-solid fa-link', titre: 'Remonter aux sources', texte: 'Indiquez la source d\'origine et joignez une preuve (photo ou PDF).' },
  { icone: 'fa-solid fa-book-open', titre: 'Publier des vérifications', texte: 'Structurez chaque analyse en deux volets : préjugé et réalité.' },
  { icone: 'fa-solid fa-flag', titre: 'Réagir & signaler', texte: 'Aimez les volets, réagissez par emoji et signalez les contenus trompeurs.' }]

const OUTILS = [
  'Vérification collaborative',
  'Volets préjugé / réalité',
  'Sources & preuves',
  'Réactions & signalement']
</script>
