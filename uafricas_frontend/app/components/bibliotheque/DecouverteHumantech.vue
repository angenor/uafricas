<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Humantech"
    sous-titre="Rencontrer celles et ceux qui portent la mémoire du continent"
    icone="fa-solid fa-chalkboard-user"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          En Afrique, on dit qu'un vieillard qui meurt est une bibliothèque qui brûle.
          <strong class="font-bold text-af-encre">Humantech</strong> relie celles et ceux qui
          détiennent un savoir : aînés, griots, experts, témoins de l'histoire, à celles et ceux
          qui veulent apprendre. Ici, on ne prête pas des livres : on
          <strong class="font-bold text-af-encre">écoute des personnes</strong>, on recueille leurs
          expériences de vie et on garde vivante la mémoire du continent et de ses diasporas.
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
            Préserver le patrimoine immatériel vivant de l'Afrique, transmettre directement les
            savoirs et les expériences, et donner toute leur place à celles et ceux qui portent ces
            mémoires.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Humantech ? »
 *
 * Contenu repris de `HumainePresentationModal.vue`, sous le nom que le menu
 * emploie : **Humantech** (`navigation-modules.ts`). Le texte disait
 * « HumanTech » et « Bibliothèques Humaines » selon l'endroit.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-user-tie', titre: 'Devenir bibliothèque humaine', texte: 'Créez votre profil et présentez les domaines et expériences que vous souhaitez transmettre.' },
  { icone: 'fa-solid fa-id-card', titre: 'Consulter les profils', texte: 'Parcourez les biographies, les spécialités et les domaines de nos personnes ressources.' },
  { icone: 'fa-solid fa-magnifying-glass', titre: 'Rechercher une personne ressource', texte: 'Trouvez la bonne personne grâce à la recherche et aux filtres par spécialité.' },
  { icone: 'fa-solid fa-comments', titre: 'Réagir & recommander', texte: 'Aimez, commentez et recommandez les personnes ressources dont le savoir vous a marqué.' }]

const OUTILS = [
  'Répertoire de personnes ressources',
  'Recherche par spécialité',
  'Réactions & recommandations']
</script>
