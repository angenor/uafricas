<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Africonnect"
    sous-titre="Renouer les liens à travers l'Afrique et sa diaspora"
    icone="fa-solid fa-users"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          La vie, les migrations et le temps nous éloignent parfois de personnes qui comptent. <strong class="font-bold text-af-encre">Africonnect</strong> vous aide à <strong class="font-bold text-af-encre">retrouver familles, amis et connaissances perdus de vue</strong>, partout en Afrique et dans la diaspora, et à renouer le contact en toute confiance.
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
            Reconnecter familles, amis et connaissances séparés par la distance ou le temps, renforcer le tissu communautaire panafricain et faciliter les retrouvailles à travers le continent et la diaspora.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Africonnect ? » : portée sur `AfricansModaleDecouverte`, comme
 * Codimoi. Le texte est repris mot pour mot de l'ancienne modale : c'est
 * l'habillage qui change, pas le propos.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-magnifying-glass', titre: 'Recherchez une personne', texte: 'Retrouvez quelqu\'un à partir d\'un nom, d\'un lieu ou d\'une école.' },
  { icone: 'fa-solid fa-bullhorn', titre: 'Lancez un avis de recherche', texte: 'Décrivez la personne recherchée et mobilisez toute la communauté panafricaine.' },
  { icone: 'fa-solid fa-handshake', titre: 'Reconnectez en confiance', texte: 'La mise en relation ne se fait qu\'avec l\'accord des deux personnes concernées.' },
  { icone: 'fa-solid fa-user-check', titre: 'Validez les correspondances', texte: 'Vérifiez et confirmez les pistes proposées avant tout partage de coordonnées.' }]

const OUTILS = ['Moteur de recherche', 'Profils membres', 'Notifications', 'Validation & signalement']
</script>
