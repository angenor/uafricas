<script setup lang="ts">
import {
  CATEGORIES_POST,
  PAYS_AFRICAINS,
  type CodiMoiPostAPI,
  type CommentaireAPI,
} from '~/composables/useCodiMoi'
import type { BrouillonCodimoi } from '~/components/codi-moi/PublierModale.vue'
import type { MembreLightAPI } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

/**
 * Codimoi : fil des publications, porté sur le gabarit de la refonte.
 *
 * Logique de données inchangée : mêmes endpoints, mêmes filtres serveur, même
 * pagination « Voir plus ». Ce qui bouge est la présentation : la carte passe
 * sur `AfricansCartePublication`, relevée sur cet écran précis au lot 2, et
 * la place des commandes : la recherche monte dans le rail, les deux filtres
 * servis par l'API restent en tête de colonne comme sur la maquette.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Codimoi - Codification des valeurs | AfricanS',
})

const { erreur: apiErreur, listerPosts, creerPost, reagir, listerCommentaires, creerCommentaire } = useCodiMoi()
const { listerAmis } = useAmis()
const { demanderOuverture } = useMessagerie()
// Composables appelés au SETUP et non dans les gestionnaires : hors de la
// portée du composant, Nuxt ne sait plus à quelle instance les rattacher.
const { initAuth, redirigerVersConnexion } = useAuth()
const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

// Modale « C'est quoi Codimoi ? »
const decouverteOuverte = ref(false)

// Données : on utilise directement CodiMoiPostAPI (pas de mapping)
const posts = ref<CodiMoiPostAPI[]>([])
const totalPosts = ref(0)

// Amis de l'utilisateur connecté (alimente le rail : profil + messagerie)
const amis = ref<MembreLightAPI[]>([])

const chargerAmis = async () => {
  if (!userStore.isAuthenticated) {
    amis.value = []
    return
  }
  const liste = await listerAmis()
  amis.value = liste.map(a => a.utilisateur)
}

/**
 * La ventilation par catégorie est calculée sur les posts CHARGÉS, pas sur le
 * fonds : aucun endpoint ne la sert, et la liste est paginée. Le panneau le
 * dit : un décompte présenté comme global serait faux dès la deuxième page.
 */
const ventilation = computed(() => {
  const p = posts.value
  return [
    { libelle: 'Proverbes & adages', valeur: p.filter(x => x.type === 'proverbe_adage').length },
    { libelle: 'Citations', valeur: p.filter(x => x.type === 'citation').length },
    { libelle: 'Ressources historiques', valeur: p.filter(x => x.type === 'ressource_historique').length },
    { libelle: 'Bonnes pratiques', valeur: p.filter(x => x.type === 'bonne_pratique').length },
    // Garde contre les valeurs manquantes/non numériques (constat #14 : total NaN)
    { libelle: 'Likes', valeur: p.reduce((s, x) => s + (Number(x.nombre_likes) || 0), 0) }]
})

// Posts populaires triés par likes
const popularPosts = computed(() =>
  [...posts.value].sort((a, b) => b.nombre_likes - a.nombre_likes).slice(0, 5))

// Modale détail de post
const selectedPost = ref<CodiMoiPostAPI | null>(null)
const selectedPostCommentaires = ref<CommentaireAPI[]>([])
const chargementCommentaires = ref(false)

// UI state
const publierOuvert = ref(false)
const loading = ref(false)
const loadingMore = ref(false)
const currentPage = ref(1)
const parPage = 10
const pageErreur = ref<string | null>(null)

// Filtres
const activeCategory = ref('')
const searchKeywords = ref('')
const searchPays = ref('')

// Toast
const showToast = ref(false)
const toastMessage = ref('')

const hasActiveFilters = computed(() =>
  !!(activeCategory.value || searchKeywords.value || searchPays.value))

const hasMorePosts = computed(() => posts.value.length < totalPosts.value)

// Charger les posts depuis l'API
async function chargerPosts(append = false) {
  if (!append) loading.value = true
  pageErreur.value = null

  const resultat = await listerPosts({
    type: activeCategory.value || undefined,
    recherche: searchKeywords.value || undefined,
    pays: searchPays.value || undefined,
    page: currentPage.value,
    par_page: parPage,
  })

  if (resultat) {
    if (append) posts.value.push(...resultat.posts)
    else posts.value = resultat.posts
    totalPosts.value = resultat.total
  }
  else if (apiErreur.value) {
    pageErreur.value = apiErreur.value
  }

  loading.value = false
}

// Actions : Modale détail
const openPostDetail = async (postId: string) => {
  const post = posts.value.find(p => p.id === postId)
  if (!post) return
  selectedPost.value = post
  chargementCommentaires.value = true
  selectedPostCommentaires.value = []

  const resultat = await listerCommentaires(postId)
  if (resultat) selectedPostCommentaires.value = resultat.commentaires
  chargementCommentaires.value = false
}

const closePostDetail = () => {
  selectedPost.value = null
  selectedPostCommentaires.value = []
}

const handleModalReaction = async (type: 'like' | 'dislike') => {
  if (!selectedPost.value) return
  const postId = selectedPost.value.id
  const updatedPost = await reagir(postId, type)
  if (updatedPost) {
    selectedPost.value = updatedPost
    const index = posts.value.findIndex(p => p.id === postId)
    if (index !== -1) posts.value[index] = updatedPost
  }
}

const handleModalComment = async (contenu: string) => {
  if (!selectedPost.value) return
  const commentaire = await creerCommentaire(selectedPost.value.id, contenu)
  if (commentaire) {
    selectedPostCommentaires.value.unshift(commentaire)
    // Mettre à jour le compteur dans la liste et la modale
    const found = posts.value.find(p => p.id === selectedPost.value?.id)
    if (found) found.nombre_commentaires++
    if (selectedPost.value) selectedPost.value.nombre_commentaires++
  }
  else {
    showNotification(apiErreur.value || 'Erreur lors de la publication du commentaire')
  }
}

const handleModalShare = () => {
  if (!selectedPost.value) return
  copierLien(selectedPost.value.id)
}

// Actions : Carte (liste)
const handleReaction = async (postId: string, type: 'like' | 'dislike') => {
  const updatedPost = await reagir(postId, type)
  if (updatedPost) {
    const index = posts.value.findIndex(p => p.id === postId)
    if (index !== -1) posts.value[index] = updatedPost
  }
}

/** Codimoi n'ENREGISTRE pas les partages : l'action copie un lien. C'est
 *  pourquoi la barre d'interactions n'affiche aucun compteur de partage. */
const copierLien = (postId: string) => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(`${window.location.origin}/codi-moi/${postId}`)
  }
  showNotification('Lien copié dans le presse-papiers.')
}

const showNotification = (message: string) => {
  toastMessage.value = message
  showToast.value = true
  setTimeout(() => { showToast.value = false }, 2500)
}

// Publication
const isSubmitting = ref(false)

const publier = async (brouillon: BrouillonCodimoi & { hashtags: string[] }) => {
  isSubmitting.value = true

  const nouveauPost = await creerPost({
    type: brouillon.categorie,
    contenu: brouillon.contenu,
    explication: brouillon.explication || undefined,
    nom_auteur_originel: brouillon.nomAuteur || undefined,
    pays: brouillon.pays || undefined,
    groupe_ethnique: brouillon.groupeEthnique || undefined,
    couleur_fond: brouillon.couleurFond || undefined,
    hashtags: brouillon.hashtags.length > 0 ? brouillon.hashtags : undefined,
  })

  isSubmitting.value = false

  // La modale ne se referme QUE si la publication a abouti : la refermer sur
  // un échec jetterait la saisie avec elle.
  if (nouveauPost) {
    publierOuvert.value = false
    showNotification('Publication créée.')
    await chargerPosts()
  }
  else {
    showNotification(apiErreur.value || 'Erreur lors de la création')
  }
}

const ouvrirPublication = () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  publierOuvert.value = true
}

const resetFilters = () => {
  activeCategory.value = ''
  searchKeywords.value = ''
  searchPays.value = ''
  currentPage.value = 1
  chargerPosts()
}

const loadMore = async () => {
  loadingMore.value = true
  currentPage.value++
  await chargerPosts(true)
  loadingMore.value = false
}

// Recharger quand les filtres changent (avec debounce)
let debounceTimer: ReturnType<typeof setTimeout> | null = null
watch([activeCategory, searchKeywords, searchPays], () => {
  currentPage.value = 1
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => chargerPosts(), 300)
})

const photoComplete = (url: string | null): string | null => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

onMounted(async () => {
  // Attendre que l'auth soit initialisee pour envoyer le token et recuperer user_reaction
  await initAuth()
  chargerPosts()
  chargerAmis()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Codimoi"
        image="/images/africans/heros/hero-codimoi.jpg"
        aide="C'est quoi Codimoi ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africarise', vers: '/codi-moi' }, { libelle: 'Codimoi' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="ouvrirPublication">
            Nouvelle publication
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Les deux filtres servis par l'API. La maquette en montre un
           troisième, « Groupe Ethniques » : aucun paramètre serveur ne le
           porte, et le filtrer côté client ne toucherait que la page
           courante : il est donc omis plutôt que faussement proposé. -->
      <div class="flex flex-wrap items-end gap-4">
        <AfricansChamp v-model="searchPays" libelle="Territoire" type="select" class="min-w-52 flex-1">
          <option value="">Tous les territoires</option>
          <option v-for="p in PAYS_AFRICAINS" :key="p" :value="p">{{ p }}</option>
        </AfricansChamp>

        <AfricansChamp v-model="activeCategory" libelle="Catégorie" type="select" class="min-w-52 flex-1">
          <option v-for="c in CATEGORIES_POST" :key="c.value" :value="c.value">{{ c.label }}</option>
        </AfricansChamp>

        <button
          v-if="hasActiveFilters"
          type="button"
          class="flex h-11 items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
          @click="resetFilters"
        >
          <font-awesome-icon icon="fa-solid fa-xmark" />
          Effacer les filtres
        </button>
      </div>

      <h2 class="flex flex-wrap items-baseline gap-3 text-[20px]/[1.4] font-bold text-af-chocolat">
        Publications
        <span v-if="hasActiveFilters" class="text-[14px]/[1.4] font-normal text-af-atone">
          {{ totalPosts }} résultat{{ totalPosts > 1 ? 's' : '' }}
        </span>
      </h2>

      <!-- Chargement : squelettes aux dimensions d'une carte de publication. -->
      <div v-if="loading && posts.length === 0" class="flex flex-col gap-6">
        <div v-for="n in 3" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="flex items-center gap-3 p-4">
            <div class="size-11 animate-pulse rounded-full bg-af-bordure" />
            <div class="flex-1 space-y-2">
              <div class="h-3 w-1/3 animate-pulse rounded bg-af-bordure" />
              <div class="h-3 w-1/4 animate-pulse rounded bg-af-bordure" />
            </div>
          </div>
          <div class="aspect-[16/10] w-full animate-pulse bg-af-bordure" />
          <div class="h-10 animate-pulse bg-white" />
        </div>
      </div>

      <!-- Erreur : le message technique est montré, pas masqué derrière un
           « une erreur est survenue » qui n'aide personne à diagnostiquer. -->
      <div
        v-else-if="pageErreur && posts.length === 0"
        class="rounded-[10px] border border-af-live/30 bg-af-live/[0.05] p-6"
      >
        <p class="flex items-center gap-3 text-[16px]/[1.4] font-bold">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-af-live" />
          Les publications n'ont pas pu être chargées
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">{{ pageErreur }}</p>
        <AfricansBouton class="mt-5" icone="fa-solid fa-rotate-right" @click="chargerPosts()">
          Réessayer
        </AfricansBouton>
      </div>

      <template v-else-if="posts.length">
        <div class="flex flex-col gap-6">
          <CodiMoiCartePost
            v-for="post in posts"
            :key="post.id"
            :post="post"
            @jaime="handleReaction(post.id, 'like')"
            @jaime-pas="handleReaction(post.id, 'dislike')"
            @commenter="openPostDetail(post.id)"
            @partager="copierLien(post.id)"
          />
        </div>

        <div v-if="hasMorePosts" class="flex justify-center">
          <AfricansBouton
            variante="secondaire"
            :desactive="loadingMore"
            :tourne="loadingMore"
            :icone="loadingMore ? 'fa-solid fa-spinner' : 'fa-solid fa-arrow-down'"
            @click="loadMore"
          >
            {{ loadingMore ? 'Chargement…' : 'Voir plus de publications' }}
          </AfricansBouton>
        </div>
      </template>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien
           n'existe », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-file-circle-xmark" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ hasActiveFilters ? 'Aucune publication ne correspond à vos critères' : 'Aucune publication pour le moment' }}
        </p>
        <AfricansBouton
          class="mt-5"
          :variante="hasActiveFilters ? 'secondaire' : 'primaire'"
          :icone="hasActiveFilters ? 'fa-solid fa-rotate-right' : 'fa-solid fa-plus'"
          @click="hasActiveFilters ? resetFilters() : ouvrirPublication()"
        >
          {{ hasActiveFilters ? 'Effacer les filtres' : 'Publier la première valeur' }}
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansRecherche v-model="searchKeywords" placeholder="Proverbe, récit, mot-clé…" />

      <AfricansPanneau titre="Statistiques Codimoi" icone="fa-solid fa-chart-line">
        <dl class="flex flex-col">
          <div class="flex items-baseline justify-between gap-4 pb-3">
            <dt class="text-[14px]/[1.4] font-bold">Publications totales</dt>
            <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ totalPosts }}</dd>
          </div>
          <div
            v-for="ligne in ventilation"
            :key="ligne.libelle"
            class="flex items-baseline justify-between gap-4 border-t border-af-bordure py-3"
          >
            <dt class="text-[14px]/[1.4] text-af-corps">{{ ligne.libelle }}</dt>
            <dd class="text-[14px]/[1.4] font-bold">{{ ligne.valeur }}</dd>
          </div>
        </dl>
        <p class="mt-3 text-[12px]/[1.4] text-af-atone">
          Ventilation calculée sur les {{ posts.length }} publications affichées.
        </p>
      </AfricansPanneau>

      <AfricansPanneau titre="Mes ami(e)s" icone="fa-solid fa-users">
        <ul v-if="amis.length" class="flex flex-col">
          <li
            v-for="ami in amis"
            :key="ami.id"
            class="flex items-center gap-3 border-t border-af-bordure py-2.5 first:border-t-0"
          >
            <NuxtLink :to="`/profil/${ami.id}`" class="flex min-w-0 flex-1 items-center gap-3 hover:text-af-chocolat">
              <AfricansAvatar :nom="`${ami.prenom} ${ami.nom}`" :src="photoComplete(ami.photoUrl)" :taille="32" />
              <span class="truncate text-[14px]/[1.4] font-bold">{{ ami.prenom }} {{ ami.nom }}</span>
            </NuxtLink>
            <button
              type="button"
              class="grid size-8 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-af-chocolat transition hover:bg-af-chocolat hover:text-white"
              :aria-label="`Envoyer un message à ${ami.prenom} ${ami.nom}`"
              @click="demanderOuverture(ami)"
            >
              <font-awesome-icon icon="fa-solid fa-paper-plane" class="text-xs" />
            </button>
          </li>
        </ul>
        <p v-else class="text-[14px]/[1.4] text-af-atone">
          {{ userStore.isAuthenticated ? 'Aucun ami pour le moment.' : 'Connectez-vous pour retrouver vos ami(e)s.' }}
        </p>
      </AfricansPanneau>

      <AfricansPanneau titre="Posts populaires" icone="fa-solid fa-fire">
        <ul v-if="popularPosts.length" class="flex flex-col gap-3">
          <li v-for="post in popularPosts" :key="post.id">
            <button
              type="button"
              class="w-full rounded-[10px] border border-af-bordure p-3 text-left transition hover:bg-af-chocolat/[0.07]"
              @click="openPostDetail(post.id)"
            >
              <p class="line-clamp-2 text-[14px]/[1.4]">{{ post.contenu }}</p>
              <p class="mt-2 flex items-center justify-between gap-3 text-[12px]/[1.4] text-af-atone">
                <span class="truncate">{{ post.auteur.prenom || post.auteur.nom }}</span>
                <span class="flex shrink-0 items-center gap-1.5">
                  <font-awesome-icon icon="fa-solid fa-thumbs-up" />
                  {{ post.nombre_likes }}
                </span>
              </p>
            </button>
          </li>
        </ul>
        <p v-else class="text-[14px]/[1.4] text-af-atone">Aucun post populaire.</p>
      </AfricansPanneau>
    </template>

    <!-- ══════════════ Surcouches ══════════════ -->

    <CodiMoiDecouverteModale v-model="decouverteOuverte" />

    <CodiMoiPublierModale
      v-model="publierOuvert"
      :en-cours="isSubmitting"
      @publier="publier"
    />

    <!-- Détail de post : la modale héritée, non encore portée, aucun cadre
         Figma ne la décrit (« Infos Codimoi » est l'écran de découverte). -->
    <CodiMoiPostModal
      :post="selectedPost"
      :commentaires="selectedPostCommentaires"
      :chargement-commentaires="chargementCommentaires"
      @close="closePostDetail"
      @like="handleModalReaction('like')"
      @dislike="handleModalReaction('dislike')"
      @share="handleModalShare"
      @commenter="handleModalComment"
    />

    <Transition name="af-surgir">
      <div
        v-if="showToast"
        class="fixed right-6 bottom-6 z-100 max-w-sm rounded-[10px] border border-af-vert bg-white px-5 py-4 shadow-xl font-af"
        role="status"
      >
        <p class="flex items-center gap-3 text-[14px]/[1.4]">
          <font-awesome-icon icon="fa-solid fa-circle-check" class="text-af-vert" />
          {{ toastMessage }}
        </p>
      </div>
    </Transition>
  </NuxtLayout>
</template>

<style scoped>
.af-surgir-enter-active,
.af-surgir-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}
.af-surgir-enter-from,
.af-surgir-leave-to {
  opacity: 0;
  transform: translateY(12px);
}
</style>
