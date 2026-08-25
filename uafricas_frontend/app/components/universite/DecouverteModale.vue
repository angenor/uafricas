<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Muniversa"
    sous-titre="Institut universitaire pour le développement de l'Afrique"
    icone="fa-solid fa-graduation-cap"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          <strong class="font-bold text-af-encre">Muniversa</strong> démocratise l'accès à
          l'enseignement supérieur de qualité pour tous les Africains, où qu'ils se trouvent dans le
          monde. L'institut vulgarise des <strong class="font-bold text-af-encre">formations de
          masse</strong> sur les enjeux qui comptent pour le développement du continent et le
          renforcement de sa gouvernance.
        </p>

        <div v-else-if="etape === 1" class="flex flex-col gap-6">
          <div class="flex flex-col gap-3">
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Notre vision</h3>
            <p class="text-[14px]/[1.4] text-af-corps">
              Devenir la référence de l'éducation numérique en Afrique, en créant un pont entre les
              meilleures institutions académiques du continent et les apprenants de la diaspora.
            </p>
          </div>

          <div class="flex flex-col gap-3">
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Nos valeurs</h3>
            <div class="flex flex-wrap gap-2">
              <AfricansEtiquette v-for="v in VALEURS" :key="v">{{ v }}</AfricansEtiquette>
            </div>
          </div>
        </div>

        <div v-else class="flex flex-col gap-3">
          <h3 class="flex items-center gap-3 text-[17px]/[1.4] font-bold text-af-vert">
            <font-awesome-icon icon="fa-solid fa-seedling" class="size-6" />
            Notre impact
          </h3>
          <p class="text-[14px]/[1.4] text-af-corps">
            Permettre à des milliers d'apprenants d'accéder à une éducation de qualité, et contribuer
            ainsi au développement des compétences et à la transformation numérique de l'Afrique.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Muniversa ? ».
 *
 * Elle réunit DEUX surcouches qui coexistaient : l'ancienne modale de
 * présentation et la modale « À propos » ouverte depuis une carte de la page.
 * Les deux disaient la même chose : mission, vision, valeurs, impact, à deux
 * endroits différents.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const VALEURS = [
  'Excellence académique',
  'Accessibilité et inclusion',
  'Innovation pédagogique',
  'Collaboration panafricaine',
  'Développement durable',
  'Intégrité et transparence']
</script>
