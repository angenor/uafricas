<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Ideaforces"
    sous-titre="Un laboratoire d'intelligence collective pour l'Afrique"
    icone="fa-solid fa-lightbulb"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          L'Afrique regorge d'idées et de talents.
          <strong class="font-bold text-af-encre">Ideaforces</strong> est un laboratoire
          d'intelligence collective : un espace où l'on réunit idées, expertises et expériences pour
          imaginer, ensemble, des <strong class="font-bold text-af-encre">réponses concrètes</strong>
          aux défis du continent, des Afro-descendants et de la diaspora.
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
            Faire émerger des solutions innovantes pour le développement africain, élargir la
            participation citoyenne aux grandes réflexions et rapprocher experts, institutions et
            citoyens.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/** « C'est quoi Ideaforces ? » : contenu repris de la modale inline de la page. */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-lightbulb', titre: 'Proposer une idée', texte: 'Publiez une proposition structurée : objectif, modalités concrètes et bénéficiaires.' },
  { icone: 'fa-solid fa-layer-group', titre: 'Cibler un enjeu', texte: 'Rattachez votre idée à une thématique : gouvernance, éducation, santé, environnement…' },
  { icone: 'fa-solid fa-magnifying-glass', titre: 'Explorer les propositions', texte: 'Parcourez et filtrez les idées de la communauté par thématique et territoire.' },
  { icone: 'fa-solid fa-share-nodes', titre: 'Diffuser à la communauté', texte: 'Partagez les propositions qui vous inspirent sur le mur communautaire.' }]

const OUTILS = [
  'Propositions structurées',
  'Catégories thématiques',
  'Recherche & filtres',
  'Partage communautaire']
</script>
