<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Africalive"
    sous-titre="Les événements en ligne de l'Afrique et de ses diasporas"
    icone="fa-solid fa-calendar-days"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          <strong class="font-bold text-af-encre">Africalive</strong> est votre espace d'événements
          100 % en ligne pour rassembler l'Afrique, ses diasporas et ses afrodescendants.
          Webinaires, conférences, forums ou ateliers : on se retrouve autour des grands enjeux du
          continent pour <strong class="font-bold text-af-encre">échanger des idées, apprendre les
          uns des autres et construire ensemble des solutions concrètes</strong>.
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
            Faire vivre un vrai dialogue panafricain : diffuser les connaissances et les bonnes
            pratiques, et faire naître de nouveaux partenariats et de nouvelles coopérations à
            travers le continent et sa diaspora.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/** « C'est quoi Africalive ? » : contenu repris de `PresentationModal.vue`. */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-calendar-plus', titre: 'Organiser vos événements', texte: 'Webinaires, conférences, forums ou ateliers collaboratifs : lancez et animez votre événement en quelques clics.' },
  { icone: 'fa-solid fa-comments', titre: 'Participer en direct', texte: 'Inscrivez-vous en ligne, intervenez pendant les sessions et posez vos questions en temps réel.' },
  { icone: 'fa-solid fa-play', titre: 'Revoir à tout moment', texte: 'Une fois l\'événement terminé, sa rediffusion reste accessible quand vous le souhaitez.' }]

const OUTILS = ['Visioconférence', 'Streaming en direct', 'Chat interactif', 'Gestion des inscriptions']
</script>
