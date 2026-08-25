<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Diapertise"
    sous-titre="Le répertoire des expertises africaines et afrodescendantes"
    icone="fa-solid fa-user-tie"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Partout dans le monde, des Africaines et des Africains, sur le continent comme dans les
          diasporas : portent des savoir-faire précieux.
          <strong class="font-bold text-af-encre">Diapertise</strong> les rassemble en un seul
          endroit pour les rendre
          <strong class="font-bold text-af-encre">visibles et faciles à solliciter</strong>, au
          service du développement du continent.
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
            Mobiliser la richesse des talents de la diaspora, renforcer les compétences techniques,
            scientifiques et managériales en Afrique, et faire circuler les savoirs et les bonnes
            pratiques.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Diapertise ? » : contenu repris à l'identique de
 * `PresentationModal.vue`, porté sur la modale à étapes commune aux modules.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-users', titre: 'Un répertoire mondial', texte: 'Découvrez des experts et des professionnels africains et afrodescendants, où qu\'ils soient.' },
  { icone: 'fa-solid fa-magnifying-glass', titre: 'Une recherche sur mesure', texte: 'Filtrez par domaine, territoire ou situation professionnelle et affinez selon vos critères.' },
  { icone: 'fa-solid fa-envelope', titre: 'Un contact direct', texte: 'Consultez la fiche d\'un expert et prenez contact directement avec lui.' },
  { icone: 'fa-solid fa-user-plus', titre: 'Proposez votre expertise', texte: 'Déposez votre candidature et votre CV pour rejoindre le répertoire des experts.' }]

const OUTILS = [
  'Base de données d\'experts',
  'Recherche multicritère',
  'Fiches détaillées',
  'Candidature en ligne',
  'Notation & recommandations']
</script>
