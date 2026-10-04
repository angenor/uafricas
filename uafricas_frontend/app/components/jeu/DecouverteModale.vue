<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="C'est quoi les Activités ?"
    sous-titre="Jouer, apprendre, faire gagner son pays"
    icone="fa-solid fa-gamepad"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : la modale ne change pas de taille d'un écran à l'autre. -->
      <div class="min-h-[220px]">
        <p v-if="etape === 0" class="text-[14px]/[1.5] text-af-corps">
          Les <strong class="font-bold text-af-encre">Activités</strong> transforment ce que la
          plateforme publie (proverbes, idées reçues vérifiées, fiches pays, langues) en
          <strong class="font-bold text-af-encre">épreuves</strong> : une question, quelques
          propositions, une explication. On les joue en temps limité, seul, chaque jour ou contre un ami.
        </p>

        <div v-else-if="etape === 1" class="grid gap-3 sm:grid-cols-2">
          <div
            v-for="item in FACONS_DE_JOUER"
            :key="item.titre"
            class="flex gap-3 rounded-[10px] border border-af-bordure p-4"
          >
            <font-awesome-icon :icon="item.icone" class="mt-0.5 size-6 shrink-0 text-af-chocolat" />
            <div class="min-w-0">
              <p class="text-[14px]/[1.4] font-bold">{{ item.titre }}</p>
              <p class="mt-1 text-[12px]/[1.4] text-af-corps">{{ item.texte }}</p>
            </div>
          </div>
        </div>

        <div v-else class="flex flex-col gap-3">
          <h3 class="flex items-center gap-3 text-[17px]/[1.4] font-bold text-af-vert">
            <font-awesome-icon icon="fa-solid fa-trophy" class="size-6" />
            Ce que ça rapporte
          </h3>
          <p class="text-[14px]/[1.5] text-af-corps">
            Chaque bonne réponse rapporte du <strong class="font-bold text-af-encre">score de jeu</strong>,
            qui compte pour vous et pour votre pays au Championship. Vos accomplissements vous valent de la
            réputation et des distinctions visibles sur votre profil.
          </p>
          <p class="text-[13px]/[1.5] text-af-atone">
            Le jeu ne donne pas de points d'engagement et ne change pas votre statut de membre : ceux-là
            viennent des J'aime, des partages et des cadeaux reçus.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FACONS_DE_JOUER = [
  { titre: 'Une partie', texte: 'Dix épreuves dans un module, en temps limité.', icone: 'fa-solid fa-play' },
  { titre: 'Les défis', texte: 'Le défi du jour et celui de la semaine : la même série pour tous, un seul essai.', icone: 'fa-solid fa-bolt' },
  { titre: 'Les duels', texte: 'Défiez un ami, en différé ou en direct, sur la même série.', icone: 'fa-solid fa-hand-fist' },
  { titre: 'Le Championship', texte: 'Votre score fait monter votre pays, saison après saison, sur la carte de l\'Afrique.', icone: 'fa-solid fa-earth-africa' },
]
</script>
