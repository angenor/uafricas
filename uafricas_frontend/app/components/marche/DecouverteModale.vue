<template>
  <AfricansModaleDecouverte
    :model-value="modelValue"
    titre="Afromarket"
    sous-titre="La place de marché de l'Afrique et de ses diasporas"
    icone="fa-solid fa-store"
    :nombre-etapes="3"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <template #default="{ etape }">
      <div class="min-h-[240px]">
        <p v-if="etape === 0" class="text-[14px]/[1.4] text-af-corps">
          <strong class="font-bold text-af-encre">Afromarket</strong> est une place de marché en
          ligne où l'Afrique, sa diaspora et les communautés afrodescendantes
          <strong class="font-bold text-af-encre">achètent, vendent, échangent et s'entraident</strong>.
          En reliant directement les producteurs, les PME et les acheteurs, il fait tourner le
          commerce à l'intérieur du continent comme avec le reste du monde, et met en lumière les
          produits, les savoir-faire et les marques de chez nous.
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
            Faire grandir le commerce entre l'Afrique et sa diaspora, valoriser les savoir-faire du
            continent, et soutenir les PME et les entrepreneurs locaux.
          </p>
        </div>
      </div>
    </template>
  </AfricansModaleDecouverte>
</template>

<script setup lang="ts">
/**
 * « C'est quoi Afromarket ? »
 *
 * Contenu repris de `PresentationModal.vue`, sous le nom que porte le menu :
 * **Afromarket**. Le texte disait « le Marché africain ».
 */
defineProps<{ modelValue: boolean }>()
defineEmits<{ 'update:modelValue': [boolean] }>()

const FONCTIONNALITES = [
  { icone: 'fa-solid fa-store', titre: 'Vendre produits et services', texte: 'Mettez en avant ce que vous fabriquez ou proposez, et touchez toute la communauté.' },
  { icone: 'fa-solid fa-right-left', titre: 'Échanger et troquer', texte: 'Troquez des biens entre membres, sans forcément passer par l\'argent.' },
  { icone: 'fa-solid fa-hand-holding-heart', titre: 'Donner et recevoir', texte: 'Offrez des biens ou des ressources, ou trouvez le coup de pouce dont vous avez besoin.' },
  { icone: 'fa-solid fa-handshake', titre: 'Saisir des opportunités', texte: 'Repérez des affaires et nouez des partenariats à travers le continent.' },
]

const OUTILS = ['Marché dédié', 'Catalogue produits détaillé', 'Messagerie intégrée']
</script>
