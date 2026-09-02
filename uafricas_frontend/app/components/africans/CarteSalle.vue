<template>
  <article class="flex flex-col overflow-hidden rounded-[10px] border border-af-bordure bg-white">
    <!-- Vignette 16/9 : la pastille de langue en haut à droite, l'état et la
         métrique en pied, posés sur un voile noir à 65 %, sans ce voile, le
         texte blanc devient illisible sur une vignette claire. -->
    <div class="relative aspect-video w-full overflow-hidden bg-af-bordure">
      <img v-if="image" :src="image" alt="" class="size-full object-cover" />

      <span
        v-if="langue"
        class="absolute top-3 right-3 rounded-full bg-black/65 px-3 py-1 text-[12px]/[1.4] font-bold text-white"
      >{{ langue }}</span>

      <div class="absolute inset-x-0 bottom-0 flex items-center gap-2 bg-black/65 px-3 py-2 text-[12px]/[1.4] text-white">
        <span class="flex items-center gap-2">
          <span class="size-2 rounded-full" :class="enDirect ? 'bg-af-live' : 'bg-white'" />
          {{ enDirect ? 'Live en cours' : 'Non démarrée' }}
        </span>
        <span v-if="metrique" class="ml-auto flex items-center gap-2">
          <font-awesome-icon :icon="metrique.icone" />
          {{ metrique.texte }}
        </span>
      </div>
    </div>

    <div class="flex flex-1 flex-col gap-2 p-4">
      <h3 class="text-[14px]/[1.4] font-bold">{{ titre }}</h3>
      <p v-if="description" class="line-clamp-3 text-[12px]/[1.4] text-af-corps">{{ description }}</p>
      <p v-if="lieu" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
        <font-awesome-icon icon="fa-solid fa-location-dot" />
        {{ lieu }}
      </p>

      <!-- Complément propre à l'appelant : territoires d'origine, animation,
           bandeau de fermeture… Rien de tout cela n'est dans la maquette, mais
           les données existent et les taire vaudrait perte d'information. -->
      <slot />

      <!-- mt-auto : les cartes d'une même rangée ont des descriptions de
           longueurs différentes, les actions doivent rester alignées en pied. -->
      <div class="mt-auto flex flex-col gap-2 pt-2">
        <AfricansBouton
          variante="secondaire"
          pleine-largeur
          :desactive="desactivee || chargement"
          :tourne="chargement"
          :icone="iconeAction"
          :vers="vers"
          @click="$emit('agir')"
        >
          {{ libelleAction }}
        </AfricansBouton>

        <slot name="actions" />
      </div>
    </div>
  </article>
</template>

<script setup lang="ts">
/**
 * Carte de salle Afrolang. L'état gouverne à la fois le libellé du bouton et
 * la couleur de la pastille : « Suivre le live » sur une salle non démarrée
 * n'aurait aucun sens, les deux ne peuvent pas diverger.
 *
 * La métrique de pied est FOURNIE, pas déduite : la maquette y montre un
 * nombre de participants qu'aucune donnée serveur ne porte par salle. Chaque
 * page y met le compteur qu'elle a réellement.
 */
const props = withDefaults(defineProps<{
  titre: string
  description?: string
  lieu?: string
  langue?: string
  image?: string | null
  enDirect?: boolean
  /** Compteur affiché en pied de vignette, à droite de l'état. */
  metrique?: { icone: string, texte: string }
  vers?: string
  /** Une entrée est en cours : le bouton attend et ne peut être rejoué. */
  chargement?: boolean
  /** Salle fermée par l'administration : l'entrée est refusée côté serveur. */
  desactivee?: boolean
}>(), { enDirect: false })

defineEmits<{ agir: [] }>()

const libelleAction = computed(() => {
  if (props.desactivee) return 'Salle fermée'
  if (props.chargement) return 'Connexion…'
  return props.enDirect ? 'Suivre le live' : 'Démarrer'
})

const iconeAction = computed(() => {
  if (props.desactivee) return 'fa-solid fa-ban'
  if (props.chargement) return 'fa-solid fa-spinner'
  return props.enDirect ? 'fa-solid fa-video' : 'fa-solid fa-plus'
})
</script>
