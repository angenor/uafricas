<template>
  <!-- `flex flex-col` et non `block` : en grille, les cartes s'étirent à la
       hauteur de leur rangée, mais un contenu en flux normal reste collé en
       haut. Dès qu'un titre passait à deux lignes et son voisin à une, la date
       de la carte courte flottait au-dessus du bas de la carte. -->
  <NuxtLink
    :to="`/marche-africain/${annonce.id}`"
    class="group flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white transition hover:-translate-y-1 hover:border-af-chocolat"
  >
    <!-- Image -->
    <div class="relative aspect-[16/10] shrink-0 overflow-hidden">
      <img
        v-if="annonce.photo_url"
        :src="annonce.photo_url"
        :alt="annonce.titre"
        class="size-full object-cover transition-transform duration-500 group-hover:scale-110"
      />
      <!-- Pas de `<img src="…placeholder.jpg">` : ce fichier n'a jamais existé,
           si bien qu'une annonce sans photo affichait une image CASSÉE avec son
           texte de remplacement en travers. Un repli qui doit exister sur le
           disque est un repli qui peut manquer ; celui-ci est du balisage, il
           ne peut pas échouer. -->
      <div v-else class="grid size-full place-items-center bg-af-fond">
        <font-awesome-icon icon="fa-solid fa-image" class="text-4xl text-af-atone-2" />
      </div>

      <!-- Badge type d'échange -->
      <span
        class="absolute top-3 left-3 rounded-full px-3 py-1.5 text-xs font-bold"
        :class="classeTypeEchange(annonce.type_echange)"
      >
        {{ annonce.type_echange }}
      </span>

      <MarcheFavoriBouton
        :annonce-id="annonce.id"
        :favori-initial="estFavori"
        class="absolute top-3 right-3 z-10"
      />

      <!-- Contour indispensable : cette pastille se pose aussi bien sur une
           photo que sur le repli d'image, lui-même `af-fond` (#F5F5F5), où un
           fond blanc seul serait invisible. -->
      <span
        v-if="annonce.quantite && annonce.quantite > 1"
        class="absolute bottom-3 left-3 rounded-full border border-af-bordure bg-white px-2 py-1 text-xs font-bold text-af-corps"
      >
        Min. {{ annonce.quantite }} unités
      </span>

      <div class="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
    </div>

    <!-- Contenu -->
    <div class="flex flex-1 flex-col p-4">
      <div class="flex items-center gap-1.5 text-[14px]/[1.4] text-af-atone">
        <font-awesome-icon icon="fa-solid fa-location-dot" class="shrink-0 text-af-vert" />
        <span>{{ annonce.pays }}</span>
        <template v-if="annonce.ville">
          <span class="text-af-bordure">•</span>
          <span>{{ annonce.ville }}</span>
        </template>
      </div>

      <h3 class="mt-2 line-clamp-2 text-[16px]/[1.4] font-bold text-af-encre transition-colors group-hover:text-af-chocolat">
        {{ annonce.titre }}
      </h3>

      <div class="mt-3 flex items-center justify-between gap-3">
        <span class="text-[18px]/[1.4] font-bold text-af-chocolat">{{ prixFormate }}</span>
        <span class="shrink-0 rounded bg-af-fond px-2 py-1 text-xs text-af-atone">
          {{ annonce.categorie }}
        </span>
      </div>

      <!-- `mt-auto` : la date reste au bas de la carte quelle que soit la
           hauteur du titre, et donc alignée d'une carte à l'autre. -->
      <div class="mt-auto flex items-center gap-1.5 border-t border-af-bordure pt-3 text-xs text-af-atone">
        <font-awesome-icon icon="fa-solid fa-calendar-days" class="shrink-0" />
        {{ dateFormatee }}
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatPrix, formatDateCourte, classeTypeEchange, type AnnonceAPI } from '~/composables/useMarcheAfricain'

const props = withDefaults(
  defineProps<{
    annonce: AnnonceAPI
    estFavori?: boolean
  }>(),
  { estFavori: false },
)

const prixFormate = computed(() => formatPrix(props.annonce.prix, props.annonce.devise))
const dateFormatee = computed(() => formatDateCourte(props.annonce.created_at))

</script>
