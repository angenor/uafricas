<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Africans Radio"
    sous-titre="Écouter les radios africaines en direct"
    icone="fa-solid fa-radio"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          L'Afrique a mille voix, mille langues et mille histoires. <strong class="font-bold text-af-encre">Africans Radio</strong> réunit les stations de radio du continent, nationales, locales et internationales, à écouter en direct depuis un simple lecteur, dans nos langues et nos musiques, <strong class="font-bold text-af-encre">pour rapprocher l'Afrique et sa diaspora</strong>.
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
            Faire rayonner les langues et les cultures africaines, nourrir le dialogue citoyen et panafricain, et diffuser à tous des informations et des savoirs utiles, sur les ondes, partout.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Africans Radio ? » : reprend mot pour mot le contenu de
 * `MediaRadioPresentationModal`, redécoupé en trois écrans pour entrer dans le
 * composant partagé, comme Afripulse, Codimoi, Vidafrica et Afroculture.
 *
 * L'ancienne modale déroulait tout d'un bloc et se refermait sur « J'ai
 * compris » ; celle-ci avance par pastilles, et la mascotte accueille au
 * premier écran.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  {
    icone: 'fa-solid fa-radio',
    titre: 'Écouter les radios',
    texte: 'Stations nationales, locales et internationales, à écouter en direct depuis un simple lecteur audio.',
  },
  {
    icone: 'fa-solid fa-magnifying-glass',
    titre: 'Explorer par territoire et genre',
    texte: 'Filtrez les radios par territoire et par genre musical pour retrouver vos ondes africaines.',
  }]

const OUTILS = [
  'Écoute en direct',
  'Streaming audio',
  'Filtres par territoire & genre']
</script>
