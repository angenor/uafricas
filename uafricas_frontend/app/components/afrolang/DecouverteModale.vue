<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Afrolang"
    sous-titre="Préserver, transmettre et faire revivre nos langues"
    icone="fa-solid fa-language"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Beaucoup de langues africaines et afro-descendantes disparaissent peu à peu, et les nouvelles générations s'en éloignent. <strong class="font-bold text-af-encre">Afrolang</strong> est un espace en ligne collaboratif qui réunit locuteurs natifs, apprenants, enseignants, chercheurs et membres de la diaspora pour <strong class="font-bold text-af-encre">apprendre, transmettre et sauver ces langues</strong>, ensemble.
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
            Sauvegarder et faire revivre les langues africaines, rendre leur apprentissage accessible à tous, et renforcer le lien culturel entre l'Afrique et sa diaspora.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Afrolang ? » : portée sur `AfricansModaleDecouverte`, comme
 * Codimoi. Le texte est repris mot pour mot de l'ancienne modale : c'est
 * l'habillage qui change, pas le propos.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-video', titre: 'Salles de langue en direct', texte: 'Rejoignez des salles de visioconférence par langue et par territoire pour pratiquer avec des locuteurs natifs.' },
  { icone: 'fa-solid fa-chalkboard-user', titre: 'Ateliers &amp; accompagnement', texte: 'Participez à des ateliers de conversation en direct et trouvez un accompagnateur pour progresser.' },
  { icone: 'fa-solid fa-book-open', titre: 'Ressources par langue', texte: 'Retrouvez l\'alphabet, un dictionnaire et les documents, vidéos et liens partagés dans chaque salle.' },
  { icone: 'fa-solid fa-door-open', titre: 'Vos propres salles', texte: 'Proposez une salle publique ou créez une salle privée protégée par un code pour votre groupe.' }]

const OUTILS = ['Visioconférence', 'Tableau blanc interactif', 'Chat en direct', 'Ressources partagées', 'Réactions en direct']
</script>
