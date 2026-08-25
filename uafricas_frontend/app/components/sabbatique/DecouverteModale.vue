<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Sabbafrica"
    sous-titre="Offrir un peu de son temps au développement de l'Afrique"
    icone="fa-solid fa-plane"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Vous avez des compétences et un peu de temps, un congé, une pause sabbatique, quelques
          heures ? <strong class="font-bold text-af-encre">Sabbafrica</strong> vous permet de les
          mettre <strong class="font-bold text-af-encre">au service d'organisations africaines</strong>,
          que vous soyez sur le continent ou dans la diaspora. Un échange concret, utile et humain.
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
            Renforcer les organisations locales : ONG, institutions, entreprises, collectivités, 
            partager les compétences et les bonnes pratiques, et tisser des réseaux de coopération
            durables.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/** « C'est quoi Sabbafrica ? » : contenu repris de `PresentationModal.vue`. */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-bullhorn', titre: 'Publier un besoin', texte: 'Les organisations font connaître leurs missions et leurs besoins d\'expertise.' },
  { icone: 'fa-solid fa-clipboard-list', titre: 'Organiser la mission', texte: 'Précisez la durée, les dates et les conditions de prise en charge de l\'intervention.' },
  { icone: 'fa-solid fa-paper-plane', titre: 'Candidater à une mission', texte: 'Répondez à un besoin en déposant votre candidature et votre CV.' },
  { icone: 'fa-solid fa-user-check', titre: 'Trouver le bon profil', texte: 'Consultez les candidatures reçues et retenez l\'expert le plus adapté.' }]

const OUTILS = [
  'Mise en relation par mission',
  'Carte interactive de l\'Afrique',
  'Recherche et filtres',
  'Candidature avec CV']
</script>
