<template>
  <!-- Une image : pleine largeur, ratio 16/10.
       Trois et plus : une grande à gauche, deux empilées à droite. Au-delà de
       trois, la troisième vignette porte le compte restant, la maquette ne
       montre jamais plus de trois pavés, et en ajouter casserait la hauteur
       constante de la carte dans le fil. -->
  <div v-if="images.length === 1" class="aspect-[16/10] w-full overflow-hidden bg-af-bordure">
    <img :src="images[0]" :alt="alt" class="size-full object-cover" />
  </div>

  <div v-else-if="images.length === 2" class="grid aspect-[16/10] grid-cols-2 gap-1 overflow-hidden bg-af-bordure">
    <img v-for="(src, i) in images" :key="i" :src="src" :alt="alt" class="size-full object-cover" />
  </div>

  <div v-else-if="images.length > 2" class="grid aspect-[16/10] grid-cols-2 grid-rows-2 gap-1 overflow-hidden bg-af-bordure">
    <img :src="images[0]" :alt="alt" class="row-span-2 size-full object-cover" />
    <img :src="images[1]" :alt="alt" class="size-full object-cover" />
    <div class="relative">
      <img :src="images[2]" :alt="alt" class="size-full object-cover" />
      <div
        v-if="images.length > 3"
        class="absolute inset-0 grid place-items-center bg-black/65 text-[20px]/[1.4] font-bold text-white"
      >
        +{{ images.length - 3 }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  images: string[]
  /**
   * Texte alternatif. Vide par défaut : ces images illustrent une publication
   * dont le texte est juste au-dessus. Un alt répétant ce texte le ferait lire
   * deux fois par un lecteur d'écran ; l'appelant ne le renseigne que si
   * l'image porte une information que le texte ne donne pas.
   */
  alt?: string
}>(), { alt: '' })
</script>
