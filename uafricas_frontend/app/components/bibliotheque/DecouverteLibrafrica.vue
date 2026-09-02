<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Librafrica"
    sous-titre="Toute la connaissance sur l'Afrique, à portée de clic"
    icone="fa-solid fa-display"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          <strong class="font-bold text-af-encre">Librafrica</strong> rassemble les travaux qui font
          avancer la connaissance sur l'Afrique : mémoires, thèses, études, rapports et
          publications. L'idée est simple : rendre ces ressources
          <strong class="font-bold text-af-encre">faciles à trouver, à lire et à télécharger</strong>,
          pour que la recherche africaine et sur l'Afrique profite au plus grand nombre.
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
            Élargir l'accès à la connaissance, valoriser la recherche africaine et sur l'Afrique, et
            faire circuler largement les savoirs stratégiques et les travaux de référence.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Librafrica ? »
 *
 * Contenu repris de `NumeriquePresentationModal.vue`, à un détail près : ce
 * texte parlait de « NuMeTech » et de « Bibliothèque Numérique », deux noms
 * que le menu de la plateforme n'emploie nulle part, le module s'appelle
 * **Librafrica** dans `navigation-modules.ts`, seul endroit qui fasse foi.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-file-lines', titre: 'Publier vos travaux', texte: 'Déposez mémoires, thèses, rapports, études et articles avec les informations sur l\'auteur.' },
  { icone: 'fa-solid fa-book-open', titre: 'Lire en ligne', texte: 'Consultez les documents directement depuis votre navigateur, sans rien installer.' },
  { icone: 'fa-solid fa-magnifying-glass', titre: 'Rechercher facilement', texte: 'Retrouvez la bonne ressource grâce à la recherche par titre et au filtre par type de document.' },
  { icone: 'fa-solid fa-download', titre: 'Télécharger', texte: 'Récupérez les fichiers lorsque l\'auteur l\'autorise.' }]

const OUTILS = [
  'Bibliothèque numérique',
  'Moteur de recherche',
  'Lecture en ligne',
  'Téléchargement sécurisé',
  'Gestion documentaire']
</script>
