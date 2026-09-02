<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Africans Télé"
    sous-titre="Regarder l'Afrique en images"
    icone="fa-solid fa-tv"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          L'Afrique se raconte et se filme. <strong class="font-bold text-af-encre">Africans Télé</strong> rassemble les télés et les programmes africains, du documentaire au débat, de la chronique au magazine, pour regarder et faire rayonner la culture, le développement, l'innovation et <strong class="font-bold text-af-encre">l'unité du continent</strong>.
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
            Valoriser les identités, les cultures et les réussites africaines, informer et vulgariser les grands enjeux, et faire connaître les initiatives de développement et de coopération à travers tout le continent.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Africans Télé ? » : contenu repris mot pour mot de
 * `MediaTelePresentationModal`, redécoupé en trois écrans pour entrer dans le
 * composant partagé.
 *
 * La page qui la monte est encore hors gabarit ; la modale, elle, est une
 * surface autonome : elle n'a pas à attendre le portage de sa page.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  {
    icone: 'fa-solid fa-tv',
    titre: 'Regarder les programmes',
    texte: 'Émissions thématiques, débats, documentaires et chroniques à regarder à tout moment.',
  },
  {
    icone: 'fa-solid fa-magnifying-glass',
    titre: 'Explorer les télés africaines',
    texte: 'Parcourez les télés par territoire et par catégorie, et suivez le programme à la une de chaque chaîne.',
  }]

const OUTILS = [
  'Streaming vidéo',
  'Grille des programmes',
  'Programme à la une',
  'Filtres par territoire & catégorie']
</script>
