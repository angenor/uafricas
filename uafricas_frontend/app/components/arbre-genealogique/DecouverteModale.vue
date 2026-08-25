<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Rootstree"
    sous-titre="Construire, préserver et transmettre votre héritage familial"
    icone="fa-solid fa-sitemap"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          L'histoire de nos familles se perd parfois avec le temps, les distances et les migrations.
          <strong class="font-bold text-af-encre">Rootstree</strong> vous aide à reconstituer votre
          arbre généalogique africain ou afrodescendant, à
          <strong class="font-bold text-af-encre">préserver la mémoire des vôtres</strong> et à
          transmettre votre héritage aux générations futures, ensemble, en famille.
        </p>

        <div v-else-if="etape === 1" class="flex flex-col gap-6">
          <div class="flex flex-col gap-3">
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Ce que vous pouvez faire</h3>
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
            Préserver la mémoire des familles africaines et afrodescendantes, reconnecter les lignées
            dispersées par l'histoire et les migrations, et renforcer les liens entre les générations.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Rootstree ? » : reprise à l'identique du contenu de
 * `PresentationModal.vue`, porté sur la modale à étapes commune aux modules.
 * Rien n'est ajouté : les trois fonctionnalités, les trois outils et l'objectif
 * sont ceux qui étaient déjà écrits.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-sitemap', titre: 'Construisez votre arbre', texte: 'Bâtissez votre arbre généalogique en famille et reliez les générations, main dans la main.' },
  { icone: 'fa-solid fa-image', titre: 'Ajoutez des photos', texte: 'Donnez un visage aux vôtres en attachant une photo à chaque membre de l\'arbre.' },
  { icone: 'fa-solid fa-user-check', titre: 'Retrouvez vos correspondances', texte: 'Découvrez d\'autres membres dont l\'arbre recoupe le vôtre et confirmez ensemble vos correspondances.' }]

const OUTILS = ['Arbre interactif', 'Photos', 'Validation collaborative']
</script>
