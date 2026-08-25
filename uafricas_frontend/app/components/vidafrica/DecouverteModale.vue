<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Vidafrica"
    sous-titre="Les œuvres africaines, accessibles dans toutes nos langues"
    icone="fa-solid fa-video"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          <strong class="font-bold text-af-encre">Vidafrica</strong> met à l'honneur la musique, les clips et les films africains, et surtout les langues dans lesquelles ils sont créés. Grâce au sous-titrage et à la traduction <strong class="font-bold text-af-encre">faits à plusieurs</strong>, ces œuvres deviennent accessibles à tous et notre patrimoine artistique voyage bien au-delà de ses frontières.
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
            Faire rayonner les langues africaines, renforcer la compréhension entre les cultures et promouvoir les œuvres audiovisuelles du continent et de sa diaspora.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Vidafrica ? » : portée sur `AfricansModaleDecouverte`, comme
 * Codimoi. Le texte est repris mot pour mot de l'ancienne modale : c'est
 * l'habillage qui change, pas le propos.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-language', titre: 'Traduire &amp; sous-titrer', texte: 'Sous-titrez ensemble chansons, films et documentaires pour les rendre accessibles dans d\'autres langues.' },
  { icone: 'fa-solid fa-play', titre: 'Découvrir des œuvres', texte: 'Explorez des vidéos en langues africaines et afrodescendantes, sous-titrées mot à mot.' },
  { icone: 'fa-solid fa-thumbs-up', titre: 'Aimer &amp; partager', texte: 'Réagissez aux vidéos et partagez vos coups de cœur avec votre communauté.' }]

const OUTILS = ['Lecteur vidéo intégré', 'Sous-titrage collaboratif', 'Réactions &amp; partage']
</script>
