<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="C'est quoi Codimoi ?"
    sous-titre="La mémoire numérique de l'Afrique et de ses diasporas"
    icone="fa-solid fa-landmark"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[220px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
            L'histoire, les traditions et les savoirs de l'Afrique et des peuples afro-descendants
            risquent parfois de se perdre au fil du temps. <strong class="font-bold text-af-encre">Codimoi</strong>
            est une mémoire collective en ligne où chacun peut
            <strong class="font-bold text-af-encre">documenter, conserver et partager</strong>
            récits, proverbes, traditions et savoirs, pour que rien de précieux ne disparaisse.
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
            Préserver et transmettre la mémoire africaine, diffuser les connaissances historiques
            et renforcer, de génération en génération, le sentiment d'appartenance et de continuité
            entre l'Afrique et ses diasporas.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Codimoi ? » : les trois écrans de la maquette (cadres Figma
 * « Infos Codimoi »). Le texte est repris mot pour mot de l'ancienne modale ;
 * l'illustration, elle, était extraite du Figma depuis le lot 1 et n'était
 * montée nulle part.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  {
    icone: 'fa-solid fa-pen-to-square',
    titre: 'Partagez vos récits',
    texte: 'Publiez des récits historiques, conservez des témoignages et documentez traditions et pratiques.',
  },
  {
    icone: 'fa-solid fa-masks-theater',
    titre: 'Patrimoine immatériel',
    texte: 'Proverbes, adages, citations et savoirs ancestraux valorisés et transmis.',
  },
  {
    icone: 'fa-solid fa-book-open',
    titre: 'Bibliothèque mémorielle',
    texte: 'Une mémoire collective ouverte à tous, classée par thèmes et facile à parcourir.',
  }]

const OUTILS = ['Bibliothèque numérique', 'Recherche avancée', 'Classement thématique']
</script>
