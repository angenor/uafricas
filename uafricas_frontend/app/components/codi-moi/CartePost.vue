<template>
  <AfricansCartePublication
    :auteur="auteur"
    :titre="citationOuProverbe ? undefined : post.contenu"
    :categorie="categorie"
    :etiquettes="post.hashtags?.map(h => `#${h}`)"
    :likes="post.nombre_likes"
    :commentaires="post.nombre_commentaires"
    :quand="formatDateRelative(post.created_at)"
    :jaime="post.user_reaction === 'like'"
    @jaime="$emit('jaime')"
    @commenter="$emit('commenter')"
    @partager="$emit('partager')"
  >
    <template #media>
      <!-- Proverbe et citation : le texte EST le média. Le fond coloré est
           choisi par l'auteur à la publication, il n'est pas décoratif. -->
      <blockquote
        v-if="citationOuProverbe"
        class="px-8 py-10 text-center text-white"
        :style="{ backgroundColor: post.couleur_fond || '#2D5A27' }"
      >
        <p class="text-[20px]/[1.4] font-bold">« {{ post.contenu }} »</p>
        <footer v-if="post.nom_auteur_originel" class="mt-3 text-[14px]/[1.4] text-white/90">
          {{ post.nom_auteur_originel }}
        </footer>
      </blockquote>

      <AfricansMosaiqueMedia
        v-else-if="post.image_couverture_url"
        :images="[post.image_couverture_url]"
      />
    </template>

    <template #sous-media>
      <div v-if="post.explication" class="relative isolate overflow-hidden px-4 py-4">
        <template v-if="post.image_arriere_plan_url">
          <img :src="post.image_arriere_plan_url" alt="" class="absolute inset-0 -z-10 size-full object-cover" />
          <div class="absolute inset-0 -z-10 bg-black/60" />
        </template>

        <p
          class="text-[14px]/[1.4] italic"
          :class="post.image_arriere_plan_url ? 'text-white/80' : 'text-af-atone'"
        >
          Explication
        </p>
        <p
          class="mt-1 text-[14px]/[1.4] whitespace-pre-line"
          :class="post.image_arriere_plan_url ? 'text-white' : 'text-af-corps'"
        >
          {{ post.explication }}
        </p>
      </div>
    </template>

    <!-- Trois choses que la barre de la maquette ne prévoit pas et que Codimoi
         porte réellement : le « je n'aime pas », le compteur de vues et le
         cadeau. Les taire les supprimerait du produit. -->
    <template #actions>
      <button
        type="button"
        class="flex items-center gap-2 transition hover:text-af-chocolat"
        :class="post.user_reaction === 'dislike' && 'text-af-live'"
        :aria-pressed="post.user_reaction === 'dislike'"
        @click="$emit('jaime-pas')"
      >
        <font-awesome-icon icon="fa-solid fa-thumbs-down" />
        {{ post.nombre_dislikes }}
      </button>

      <!-- Pas de compteur de vues : le serveur n'en renvoie aucun, et Codimoi
           ne compte pas les consultations. -->

      <EngagementOffrirCadeauBouton
        type-objet="codimoi"
        :objet-id="post.id"
        :auteur-id="post.auteur.id"
        :destinataire="`${post.auteur.prenom ?? ''} ${post.auteur.nom}`"
        taille="sm"
      />
    </template>
  </AfricansCartePublication>
</template>

<script setup lang="ts">
import {
  formatDateRelative,
  getCategoryLabel,
  type CodiMoiPostAPI,
  type CategoriePost,
} from '~/composables/useCodiMoi'

/**
 * Publication Codimoi sur la carte de la refonte. Le mapping se fait ici et
 * non dans la page : c'est le seul endroit où le type d'un post décide de ce
 * qui occupe l'emplacement média.
 */
const props = defineProps<{ post: CodiMoiPostAPI }>()

defineEmits<{ jaime: [], 'jaime-pas': [], commenter: [], partager: [] }>()

const citationOuProverbe = computed(() =>
  props.post.type === 'proverbe_adage' || props.post.type === 'citation')

const categorie = computed(() => getCategoryLabel(props.post.type as CategoriePost))

/**
 * `CodiMoiAuteur` ne porte pas de photo : l'avatar retombe sur les initiales,
 * ce pour quoi il a été écrit. Le lieu affiché est le territoire du POST, pas
 * celui de l'auteur : c'est ce que la maquette montre sous le nom.
 */
const auteur = computed(() => ({
  nom: `${props.post.auteur.prenom ?? ''} ${props.post.auteur.nom}`.trim(),
  lieu: [props.post.pays, props.post.groupe_ethnique].filter(Boolean).join(' · ') || undefined,
  vers: `/profil/${props.post.auteur.id}`,
}))
</script>
