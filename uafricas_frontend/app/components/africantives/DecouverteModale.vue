<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Africantives"
    sous-titre="Donner de la force aux initiatives qui font avancer l'Afrique"
    icone="fa-solid fa-lightbulb"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Partout, des Africains, des afrodescendants et des membres de la diaspora lancent des projets qui font avancer le continent, mais beaucoup manquent de visibilité. <strong class="font-bold text-af-encre">Africantives</strong> met en lumière ces initiatives : chacun peut <strong class="font-bold text-af-encre">publier la sienne</strong>, la présenter en détail et la rendre visible auprès de toute la communauté.
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
            Valoriser l'engagement citoyen et associatif, soutenir les projets porteurs d'emplois et de valeur ajoutée, et renforcer la solidarité et la coopération entre les peuples africains et afrodescendants.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Africantives ? » : contenu repris mot pour mot de
 * `AfricantivesPresentationModal`, redécoupé en trois écrans pour entrer dans
 * le composant partagé.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  {
    icone: 'fa-solid fa-lightbulb',
    titre: 'Publier une initiative',
    texte: 'Présentez votre projet : description détaillée, domaine, territoire, liens et coordonnées de contact, avec une image de couverture.',
  },
  {
    icone: 'fa-solid fa-magnifying-glass',
    titre: 'Explorer les initiatives',
    texte: "Parcourez et filtrez les initiatives par domaine d'activité et par territoire, ou lancez une recherche.",
  },
  {
    icone: 'fa-solid fa-file-lines',
    titre: 'Consulter une fiche',
    texte: "Chaque initiative dispose d'une fiche détaillée : description, porteur, liens et coordonnées pour le joindre.",
  },
  {
    icone: 'fa-solid fa-bullhorn',
    titre: 'Gagner en visibilité',
    texte: 'Votre initiative rejoint un annuaire public, consultable par toute la communauté.',
  }]

const OUTILS = [
  'Fiches projets détaillées',
  'Recherche & filtres (domaine, territoire)',
  'Image de couverture',
  'Coordonnées & liens du porteur']
</script>
