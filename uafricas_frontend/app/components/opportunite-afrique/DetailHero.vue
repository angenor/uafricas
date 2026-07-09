<template>
  <section class="relative isolate overflow-hidden">
    <!-- Fond : image de l'élément ou dégradé thématique -->
    <div class="absolute inset-0 -z-20">
      <img
        v-if="image"
        :src="image"
        :alt="titre"
        class="h-full w-full object-cover"
        :class="positionImage"
      />
      <div v-else class="h-full w-full" :class="degradeFond"></div>
    </div>

    <!-- Voile pour la lisibilité du texte -->
    <div class="absolute inset-0 -z-10 bg-gradient-to-t from-black/85 via-black/45 to-black/25"></div>

    <div
      class="mx-auto flex w-full max-w-5xl flex-col px-4 pt-20 pb-20 sm:px-6 lg:px-8 md:pt-24"
      :class="hauteur"
    >
      <!-- Fil d'Ariane (adapté au fond sombre) -->
      <div class="[&_a]:!text-white/70 [&_span]:!text-white/90 [&_svg]:!text-white/40">
        <CommonBreadcrumbNav :custom-breadcrumbs="breadcrumbs" />
      </div>

      <div class="mt-auto pt-10">
        <div v-if="$slots.badges" class="mb-4 flex flex-wrap items-center gap-2">
          <slot name="badges" />
        </div>

        <h1 class="max-w-3xl font-oswald text-3xl font-bold leading-tight text-white sm:text-4xl md:text-5xl">
          {{ titre }}
        </h1>

        <p v-if="sousTitre" class="mt-3 flex items-center gap-2 text-base text-white/80 sm:text-lg">
          <slot name="sous-titre-icon" />
          {{ sousTitre }}
        </p>

        <slot name="meta" />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
interface BreadcrumbItem {
  label: string
  to?: string
}

withDefaults(
  defineProps<{
    titre: string
    sousTitre?: string | null
    image?: string | null
    breadcrumbs: BreadcrumbItem[]
    /** Dégradé de repli lorsqu'aucune image n'est disponible */
    degradeFond?: string
    /** Position de l'image de fond (ex. portrait -> object-top) */
    positionImage?: string
    /** Hauteur minimale du hero */
    hauteur?: string
  }>(),
  {
    sousTitre: null,
    image: null,
    degradeFond: 'bg-gradient-to-br from-custom-green/80 to-custom-chocolat/80',
    positionImage: 'object-center',
    hauteur: 'min-h-[20rem] md:min-h-[24rem]',
  },
)
</script>
