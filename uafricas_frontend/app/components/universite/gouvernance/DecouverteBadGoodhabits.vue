<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="BadGoodhabits"
    sous-titre="Dénoncer ce qui nuit, féliciter ce qui inspire"
    icone="fa-solid fa-triangle-exclamation"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Nos gestes du quotidien façonnent l'Afrique de demain.
          <strong class="font-bold text-af-encre">BadGoodhabits</strong> est un espace citoyen pour
          <strong class="font-bold text-af-encre">mettre en lumière les bonnes pratiques</strong> et
          <strong class="font-bold text-af-encre">signaler ce qui nuit</strong> à l'intérêt commun, 
          sur la base de faits vérifiables : afin de cultiver ensemble responsabilité, transparence
          et citoyenneté active.
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
            Renforcer le civisme et le sens des responsabilités, promouvoir la transparence et la
            redevabilité, et diffuser des modèles positifs qui inspirent le plus grand nombre.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/** « C'est quoi BadGoodhabits ? » : contenu repris de la modale inline. */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-flag', titre: 'Signaler ce qui nuit', texte: 'Documentez les mauvaises pratiques avec preuves (photos, documents PDF).' },
  { icone: 'fa-solid fa-thumbs-up', titre: 'Valoriser le meilleur', texte: 'Mettez en avant des initiatives exemplaires et inspirantes.' },
  { icone: 'fa-solid fa-share-nodes', titre: 'Partager largement', texte: 'Diffusez les contributions sur le mur communautaire pour les faire connaître.' },
  { icone: 'fa-solid fa-book-open', titre: 'Capitaliser les initiatives', texte: 'Constituez un répertoire de bonnes pratiques reproductibles.' }]

const OUTILS = [
  'Signalements avec preuves',
  'Photos & documents (PDF)',
  'Gravité & impact',
  'Partage communautaire']
</script>
