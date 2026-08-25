<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Afroculture"
    sous-titre="Un pont entre les cultures d'Afrique et de sa diaspora"
    icone="fa-solid fa-masks-theater"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <!-- Hauteur plancher : sans elle, la modale change de taille d'un écran
           à l'autre et le bouton « Suivant » se dérobe sous le curseur. -->
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          Les cultures africaines et afrodescendantes sont d'une richesse immense, mais restent souvent dispersées et peu visibles. <strong class="font-bold text-af-encre">Afroculture</strong> est un espace en ligne qui relie les communautés culturelles du continent, les afrodescendants et la diaspora pour <strong class="font-bold text-af-encre">promouvoir, transmettre et faire dialoguer nos cultures</strong>.
        </p>

        <div v-else-if="etape === 1" class="flex flex-col gap-3">
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

        <div v-else class="flex flex-col gap-3">
          <h3 class="flex items-center gap-3 text-[17px]/[1.4] font-bold text-af-vert">
            <font-awesome-icon icon="fa-solid fa-seedling" class="size-6" />
            Notre objectif
          </h3>
          <p class="text-[14px]/[1.4] text-af-corps">
            Renforcer l'identité africaine et afrodescendante, et nourrir le dialogue interculturel et la compréhension mutuelle entre nos communautés.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Afroculture ? » : le contenu est repris mot pour mot de la
 * modale qui vivait en ligne dans la page, redécoupé en trois écrans pour
 * entrer dans le composant partagé.
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  {
    icone: 'fa-solid fa-masks-theater',
    titre: 'Promotion des valeurs',
    texte: "Découvrez le patrimoine culturel africain et afrodescendant, matériel comme immatériel, et les rendez-vous, centres culturels et produits qui le font vivre.",
  },
  {
    icone: 'fa-solid fa-user-graduate',
    titre: 'Expertise de la diaspora',
    texte: 'Explorez les compétences et les talents de la diaspora africaine et découvrez ses experts.',
  },
  {
    icone: 'fa-solid fa-earth-africa',
    titre: 'Opportunités en Afrique',
    texte: "Consultez des fiches d'information détaillées sur les opportunités, secteur par secteur et territoire par territoire.",
  },
  {
    icone: 'fa-solid fa-right-left',
    titre: 'Échanges Sabbafrica',
    texte: "Proposez ou découvrez des échanges d'expérience et de formation entre la diaspora et les organisations d'Afrique.",
  }]
</script>
