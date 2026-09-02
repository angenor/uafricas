<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Afripulse"
    sous-titre="Faire rayonner les territoires et les richesses de l'Afrique"
    icone="fa-solid fa-earth-africa"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          L'Afrique regorge de trésors culturels, touristiques, économiques et humains encore trop peu connus. <strong class="font-bold text-af-encre">Afripulse</strong> invite chacun à les <strong class="font-bold text-af-encre">faire découvrir et à les promouvoir</strong> : mettez en lumière les territoires, les communautés et les opportunités du continent, pour donner à l'Afrique la visibilité qu'elle mérite.
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
            Développer le tourisme et les opportunités locales, valoriser les patrimoines africains et donner de la visibilité aux territoires du continent et à leurs communautés.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Afripulse ? » : portée sur `AfricansModaleDecouverte`, comme
 * Codimoi. Le texte est repris mot pour mot de l'ancienne modale : c'est
 * l'habillage qui change, pas le propos.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-location-dot', titre: 'Promotion territoriale', texte: 'Sites emblématiques et destinations à faire découvrir dans chaque territoire africain.' },
  { icone: 'fa-solid fa-plane', titre: 'Tourisme collaboratif', texte: 'Sites touristiques et avis de visiteurs partagés par la communauté.' },
  { icone: 'fa-solid fa-utensils', titre: 'Saveurs d\'Afrique', texte: 'Recettes culinaires et leurs procédés, pour voyager aussi par le goût.' },
  { icone: 'fa-solid fa-briefcase', titre: 'Opportunités économiques', texte: 'Secteurs porteurs, contacts utiles et références pour explorer les opportunités des territoires.' }]

const OUTILS = ['Cartographie interactive', 'Galeries photos', 'Avis & recommandations', 'Répertoire territorial']
</script>
