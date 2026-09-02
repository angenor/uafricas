<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Afroculture"
    sous-titre="Les centres culturels africains et afro-descendants"
    icone="fa-solid fa-masks-theater"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Les <strong class="font-bold text-af-encre">centres culturels africains et afro-descendants</strong>
          (CCAD) mettent en avant les valeurs et les bonnes pratiques communes aux peuples issus ou
          descendant d'Afrique. Installés sous la coordination d'AfricanS en fonction de l'intérêt et
          de la population cible, ils existent <strong class="font-bold text-af-encre">aussi bien en
          Afrique qu'en dehors</strong>, et portent leurs activités en ligne comme en présentiel.
        </p>

        <div v-else-if="etape === 1" class="flex flex-col gap-6">
          <div class="flex flex-col gap-3">
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Ce qu'un centre porte</h3>
            <div class="grid gap-3 sm:grid-cols-2">
              <div
                v-for="item in ACTIVITES"
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
            <h3 class="text-[17px]/[1.4] font-bold text-af-chocolat">Deux familles de centres</h3>
            <div class="flex flex-wrap gap-2">
              <AfricansEtiquette>Africans International</AfricansEtiquette>
              <AfricansEtiquette>Centres culturels locaux</AfricansEtiquette>
            </div>
          </div>
        </div>

        <div v-else class="flex flex-col gap-3">
          <h3 class="flex items-center gap-3 text-[17px]/[1.4] font-bold text-af-vert">
            <font-awesome-icon icon="fa-solid fa-seedling" class="size-6" />
            Notre objectif
          </h3>
          <p class="text-[14px]/[1.4] text-af-corps">
            Faire rayonner la culture africaine et afro-descendante, valoriser les alliances entre
            communautés, et transmettre aux générations suivantes les langues et les bonnes pratiques
            qui viennent du continent.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Afroculture ? »
 *
 * Cette modale N'EXISTAIT PAS : le bandeau de `/centres` affichait le lien
 * depuis le lot 3, sans aucun gestionnaire derrière, cliquer ne faisait rien.
 * Le contenu est repris de la description que porte le cadre Afroculture du
 * Figma, seul endroit où elle était écrite.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const ACTIVITES = [
  { icone: 'fa-solid fa-earth-africa', titre: 'Rayonnement culturel', texte: 'Faire connaître la culture africaine et afro-descendante là où le centre est implanté.' },
  { icone: 'fa-solid fa-users', titre: 'Réseautage', texte: 'Mettre en relation les communautés entre territoires et entre groupes ethniques.' },
  { icone: 'fa-solid fa-handshake', titre: 'Alliances inter-ethniques', texte: 'Valoriser les alliances qui lient les peuples, plutôt que ce qui les sépare.' },
  { icone: 'fa-solid fa-masks-theater', titre: 'Promotion d\'artistes', texte: 'Mettre en avant les artistes qui portent les traditions ciblées par le centre.' },
  { icone: 'fa-solid fa-graduation-cap', titre: 'Transmission des langues', texte: 'Renforcer les capacités des enfants et des adultes en langues africaines et afro-descendantes.' },
  { icone: 'fa-solid fa-seedling', titre: 'Bonnes pratiques', texte: 'Promouvoir les pratiques d\'origine africaine sur les enjeux globaux : environnement, gouvernance, éducation.' }]
</script>
