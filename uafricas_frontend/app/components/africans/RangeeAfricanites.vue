<template>
  <div class="flex items-start gap-5 overflow-x-auto pb-2 scrollbar-none">
    <!-- Entrée de publication, toujours en tête. La maquette la nomme
         « Africanité » : c'est le nom du format, pas une action. -->
    <button
      type="button"
      class="flex w-20 shrink-0 flex-col items-center gap-2"
      @click="$emit('publier')"
    >
      <span class="grid size-16 place-items-center rounded-full border-2 border-dashed border-af-chocolat text-af-chocolat transition hover:bg-af-chocolat/[0.07]">
        <font-awesome-icon icon="fa-solid fa-plus" class="text-xl" />
      </span>
      <span class="truncate text-[12px]/[1.4] text-af-corps">Africanité</span>
    </button>

    <button
      v-for="groupe in groupes"
      :key="groupe.auteur_id"
      type="button"
      class="group flex w-20 shrink-0 flex-col items-center gap-2"
      @click="$emit('ouvrir', groupe.auteur_id)"
    >
      <!-- L'anneau EST l'information : plein et coloré tant qu'il reste
           quelque chose à voir, gris une fois tout regardé. C'est la seule
           différence entre les deux états, et elle doit se lire d'un coup. -->
      <span
        class="rounded-full p-0.5 transition"
        :class="groupe.a_du_nouveau
          ? 'ring-2 ring-af-chocolat'
          : 'ring-2 ring-af-bordure group-hover:ring-af-atone-2'"
      >
        <AfricansAvatar
          :nom="`${groupe.prenom} ${groupe.nom}`"
          :src="resoudreMedia(groupe.photo_url)"
          :taille="60"
        />
      </span>
      <span class="w-full truncate text-center text-[12px]/[1.4] text-af-corps">
        {{ groupe.est_moi ? 'Vous' : `${groupe.prenom} ${(groupe.nom ?? '').charAt(0)}.` }}
      </span>
    </button>
  </div>
</template>

<script setup lang="ts">
import type { AuteurAfricanitesAPI } from '~/composables/useAfricanite'

/**
 * Rangée des africanités en tête du fil.
 *
 * L'ordre vient du SERVEUR : le lecteur d'abord, puis ceux qui ont du nouveau.
 * Le retrier ici reproduirait une règle qui vit déjà dans la requête, et les
 * deux finiraient par diverger.
 */
defineProps<{ groupes: AuteurAfricanitesAPI[] }>()

defineEmits<{ publier: [], ouvrir: [auteurId: string] }>()

const { resoudreMedia } = useAfricanite()
</script>
