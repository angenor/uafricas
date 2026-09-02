<script setup lang="ts">
import type { VideoAfrica } from '~/composables/useVidafrica'
import { useUserStore } from '~/stores/user'

/**
 * Vidafrica : fil des vidéos sous-titrées, porté sur le gabarit de la refonte.
 *
 * Les données ne changent pas : même endpoint, mêmes filtres, même pagination.
 * La présentation, si : la grille de trois vignettes devient le FIL de cartes
 * pleine largeur de la maquette, et le filtre par langue quitte la bande de
 * pastilles pour le rail.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Vidafrica : vidéos sous-titrées en langues africaines | AfricanS',
  meta: [
    {
      name: 'description',
      content: "Vidafrica met à l'honneur la musique, les clips et les films africains, et surtout les langues dans lesquelles ils sont créés.",
    }],
})

const { listerVideos, chargerLanguesDisponibles, reagirVideo } = useVidafrica()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()
const estConnecte = computed(() => userStore.isAuthenticated)

const showProposer = ref(false)
const decouverteOuverte = ref(false)

// Partage : la modale porte la légende et publie sur le mur.
const videoAPartager = ref<VideoAfrica | null>(null)
const partageOuvert = ref(false)

const videos = ref<VideoAfrica[]>([])
const languesFiltre = ref<{ code: string, label: string, nombreVideos: number }[]>([])
const chargement = ref(true)

const recherche = ref('')
const langueSelectionnee = ref('')
const page = ref(1)
const totalPages = ref(0)
const total = ref(0)

const charger = async () => {
  chargement.value = true
  const result = await listerVideos({
    page: page.value,
    par_page: 12,
    recherche: recherche.value || undefined,
    langue: langueSelectionnee.value || undefined,
  })
  videos.value = result.videos
  totalPages.value = result.pagination.totalPages
  total.value = result.pagination.total
  page.value = result.pagination.page
  chargement.value = false
}

let debounceTimer: ReturnType<typeof setTimeout> | null = null

watch(recherche, () => {
  if (debounceTimer) clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    page.value = 1
    charger()
  }, 300)
})

watch(langueSelectionnee, () => {
  page.value = 1
  charger()
})

const allerPage = (p: number) => {
  if (p < 1 || p > totalPages.value) return
  page.value = p
  charger()
  window.scrollTo({ top: 300, behavior: 'smooth' })
}

const reinitialiser = () => {
  recherche.value = ''
  langueSelectionnee.value = ''
}

const aucunFiltreActif = computed(() => !recherche.value && !langueSelectionnee.value)

/**
 * La réaction est appliquée sur la vidéo DU FIL, pas rechargée depuis le
 * serveur : `reagirVideo` renvoie déjà les compteurs à jour, et recharger la
 * page entière ferait sauter la position de lecture de l'utilisateur.
 */
const reagir = async (video: VideoAfrica, type: 'like' | 'dislike') => {
  if (!estConnecte.value) {
    redirigerVersConnexion()
    return
  }
  const res = await reagirVideo(video.id, type)
  if (!res) return
  const i = videos.value.findIndex(v => v.id === video.id)
  if (i !== -1) {
    videos.value[i] = {
      ...videos.value[i]!,
      nombreLikes: res.nombreLikes,
      nombreDislikes: res.nombreDislikes,
      maReaction: res.maReaction,
    }
  }
}

const ouvrirPartage = (video: VideoAfrica) => {
  if (!estConnecte.value) {
    redirigerVersConnexion()
    return
  }
  videoAPartager.value = video
  partageOuvert.value = true
}

const surPartage = () => {
  const cible = videoAPartager.value
  if (!cible) return
  const i = videos.value.findIndex(v => v.id === cible.id)
  if (i !== -1) {
    videos.value[i] = { ...videos.value[i]!, nombrePartages: videos.value[i]!.nombrePartages + 1 }
  }
}

const proposer = () => {
  if (!estConnecte.value) {
    redirigerVersConnexion()
    return
  }
  showProposer.value = true
}

onMounted(async () => {
  languesFiltre.value = await chargerLanguesDisponibles()
  await charger()
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Vidafrica"
        sous-titre="Vidafrica met à l'honneur la musique, les clips et les films africains, et surtout les langues dans lesquelles ils sont créés."
        image="/images/africans/heros/hero-vidafrica.jpg"
        aide="C'est quoi Vidafrica ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Africamood', vers: '/vidafrica' }, { libelle: 'Vidafrica' }]">
        <template #action>
          <!-- La maquette dit « Partager du contenu ». Ce que le bouton ouvre
               est une PROPOSITION, soumise à modération avant publication. -->
          <AfricansBouton icone="fa-solid fa-plus" @click="proposer">
            Proposer une vidéo
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <h2 v-if="!chargement" class="text-[20px]/[1.4] font-bold text-af-chocolat">
        {{ total }} vidéo{{ total > 1 ? 's' : '' }}
      </h2>

      <!-- Chargement : squelettes à l'anatomie d'une carte du fil. -->
      <div v-if="chargement" class="flex flex-col gap-6">
        <div v-for="n in 2" :key="n" class="overflow-hidden rounded-[10px] border border-af-bordure bg-white">
          <div class="flex items-center gap-3 p-4">
            <div class="size-11 animate-pulse rounded-full bg-af-bordure" />
            <div class="h-3 w-1/3 animate-pulse rounded bg-af-bordure" />
          </div>
          <div class="aspect-video w-full animate-pulse bg-af-bordure" />
          <div class="h-10" />
        </div>
      </div>

      <template v-else-if="videos.length">
        <div class="flex flex-col gap-6">
          <VidafricaCarteVideoFil
            v-for="v in videos"
            :key="v.id"
            :video="v"
            @jaime="reagir(v, 'like')"
            @jaime-pas="reagir(v, 'dislike')"
            @partager="ouvrirPartage(v)"
          />
        </div>

        <nav v-if="totalPages > 1" class="flex items-center justify-center gap-2" aria-label="Pagination des vidéos">
          <button
            type="button"
            class="grid size-10 place-items-center rounded-lg border border-af-bordure bg-white text-af-corps transition hover:bg-af-chocolat/[0.07] disabled:opacity-40"
            :disabled="page === 1"
            aria-label="Page précédente"
            @click="allerPage(page - 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-left" />
          </button>

          <button
            v-for="p in totalPages"
            :key="p"
            type="button"
            class="h-10 min-w-10 rounded-lg px-3 text-[14px]/[1.4] font-bold transition"
            :class="p === page
              ? 'bg-af-degrade text-white'
              : 'border border-af-bordure bg-white text-af-corps hover:bg-af-chocolat/[0.07]'"
            :aria-current="p === page ? 'page' : undefined"
            @click="allerPage(p)"
          >
            {{ p }}
          </button>

          <button
            type="button"
            class="grid size-10 place-items-center rounded-lg border border-af-bordure bg-white text-af-corps transition hover:bg-af-chocolat/[0.07] disabled:opacity-40"
            :disabled="page === totalPages"
            aria-label="Page suivante"
            @click="allerPage(page + 1)"
          >
            <font-awesome-icon icon="fa-solid fa-chevron-right" />
          </button>
        </nav>
      </template>

      <!-- Deux vides distincts : « rien ne correspond » n'est pas « rien
           n'existe », et la sortie proposée n'est pas la même. -->
      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-video-slash" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ aucunFiltreActif ? 'Aucune vidéo pour le moment' : 'Aucune vidéo ne correspond à votre recherche' }}
        </p>
        <AfricansBouton
          class="mt-5"
          :variante="aucunFiltreActif ? 'primaire' : 'secondaire'"
          :icone="aucunFiltreActif ? 'fa-solid fa-plus' : 'fa-solid fa-rotate-right'"
          @click="aucunFiltreActif ? proposer() : reinitialiser()"
        >
          {{ aucunFiltreActif ? 'Proposer une vidéo' : 'Réinitialiser' }}
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansRecherche v-model="recherche" placeholder="Titre, artiste, film…" />

      <AfricansPanneau titre="Filtres" icone="fa-solid fa-sliders" action-libelle="Réinitialiser" @action="reinitialiser">
        <AfricansChamp v-model="langueSelectionnee" libelle="Langues" type="select">
          <option value="">Toutes les langues</option>
          <!-- Le décompte vient du serveur et porte sur tout le fonds, pas sur
               la page courante : il reste juste quand on tourne les pages. -->
          <option v-for="l in languesFiltre" :key="l.code" :value="l.code">
            {{ l.label }} ({{ l.nombreVideos }})
          </option>
        </AfricansChamp>
      </AfricansPanneau>
    </template>

    <!-- ══════════════ Surcouches ══════════════ -->

    <VidafricaProposerVideoModal v-model="showProposer" />

    <VidafricaPartagerModal
      v-if="videoAPartager"
      v-model="partageOuvert"
      :video="videoAPartager"
      @partage="surPartage"
    />

    <VidafricaDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
