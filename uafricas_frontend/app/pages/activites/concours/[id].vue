<script setup lang="ts">
/**
 * La page d'un concours (feature 014), qui change avec sa phase :
 *   appel     → règlement, dépôt, état de ma photo, galerie anonyme
 *   vote      → confrontations à l'aveugle, galerie anonyme
 *   résultats → podium et galerie classée, auteurs révélés
 *   annulé    → motif, galerie
 *
 * Rendue côté serveur pour le partage (titre, Open Graph). Le bloc `moi` n'y
 * est pas : le jeton vit dans le navigateur, il est relu au montage.
 */
import { messageErreurJeu } from '~/composables/useJeu'
import {
  LIBELLES_MA_PARTICIPATION,
  LIBELLES_PHASE_PUBLIQUE,
  type ConcoursPublicAPI,
  type MaParticipationAPI,
  type ParticipationAnonymeAPI,
  type ParticipationClasseeAPI,
} from '~/composables/useConcours'
import { libelleModulePlateforme } from '~/utils/modulesPlateforme'

definePageMeta({ layout: false })

const route = useRoute()
const id = computed(() => String(route.params.id))
const userStore = useUserStore()
const { obtenir, galerie, retirer } = useConcours()

const erreur = ref('')
const { data: concours, error } = await useAsyncData(`concours-${id.value}`, () => obtenir(id.value))

/** Relecture côté navigateur : `refresh()` est ignoré pendant l'hydratation. */
const relire = async () => {
  try {
    concours.value = await obtenir(id.value)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Le concours n\'a pas pu être relu.')
  }
}
const photos = ref<Array<ParticipationAnonymeAPI | ParticipationClasseeAPI>>([])
const remplacee = ref<MaParticipationAPI | null>(null)

useSeoMeta({
  title: () => concours.value ? `${concours.value.titre} | Concours | AfricanS` : 'Concours | AfricanS',
  description: () => concours.value?.theme ?? '',
  ogTitle: () => concours.value?.titre ?? 'Concours AfricanS',
  ogDescription: () => concours.value?.theme ?? '',
  ogImage: () => urlMedia(concours.value?.image_url ?? null) ?? undefined,
})

const chargerGalerie = async () => {
  try {
    photos.value = (await galerie(id.value)) ?? []
  }
  catch {
    photos.value = []
  }
}

const recharger = async () => {
  await relire()
  await chargerGalerie()
}

onMounted(async () => {
  // Le rendu serveur n'a pas de jeton : on relit pour obtenir le bloc `moi`.
  // Après un rechargement, le jeton d'accès (gardé en mémoire) est restauré
  // APRÈS le montage : on relit donc dès qu'il apparaît, pas seulement s'il
  // est déjà là.
  watch(() => userStore.accessToken, (jeton) => { if (jeton) relire() }, { immediate: true })
  await chargerGalerie()
})

const moi = computed(() => concours.value?.moi ?? null)
const photosClassees = computed(() =>
  photos.value.filter((p): p is ParticipationClasseeAPI => 'auteur' in p))
// Les plus récentes d'abord : c'est la dernière photo déposée qu'on cherche.
const mesActives = computed(() =>
  (moi.value?.participations ?? []).filter(p => p.etat !== 'retiree').slice().reverse())
/**
 * Une photo refusée ne se remplace que s'il reste de la place : la remettre en
 * attente la rend active, et le plafond du concours compte (le serveur refuse).
 */
const remplacable = (p: MaParticipationAPI) =>
  p.etat === 'en_attente' || (p.etat === 'rejetee' && !!moi.value?.peut_participer)
const phase = computed(() => concours.value?.phase)

const dateLongue = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', { weekday: 'long', day: 'numeric', month: 'long', hour: '2-digit', minute: '2-digit' })

const classeEtat = (etat: MaParticipationAPI['etat']) => ({
  en_attente: 'bg-af-orange/15 text-af-chocolat',
  publiee: 'bg-af-vert/15 text-af-vert',
  rejetee: 'bg-af-live/10 text-af-live',
  retiree: 'bg-af-bordure text-af-corps',
  suspendue: 'bg-af-live/10 text-af-live',
}[etat])

const retirerMaPhoto = async (p: MaParticipationAPI) => {
  if (!confirm('Retirer votre photo de ce concours ?')) return
  erreur.value = ''
  try {
    await retirer(id.value, p.id)
    await recharger()
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
  }
}

const apresDepot = async () => {
  remplacee.value = null
  await recharger()
}

const versConnexion = () => navigateTo({ path: '/login', query: { redirect: route.fullPath } })
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Activités', vers: '/activites' },
          { libelle: 'Concours', vers: '/activites/concours' },
          { libelle: concours?.titre ?? 'Concours' },
        ]"
      />
    </template>

    <div class="flex flex-col gap-6 pb-24">
      <p v-if="error || !concours" class="rounded-[10px] border border-af-bordure bg-af-surface px-5 py-8 text-center text-af-corps">
        Ce concours est introuvable.
      </p>

      <template v-else>
        <header class="flex flex-col gap-3">
          <p class="text-[13px]/[1.3] font-bold uppercase tracking-wide text-af-atone">
            Bataille de photos<template v-if="concours.rattachement"> · {{ libelleModulePlateforme(concours.rattachement) }}</template>
          </p>
          <h1 class="text-[26px]/[1.25] font-bold text-af-encre">{{ concours.titre }}</h1>
          <p class="max-w-3xl text-[16px]/[1.6] text-af-corps">{{ concours.theme }}</p>
          <div class="flex flex-wrap items-center gap-x-5 gap-y-2 text-[14px]/[1.4] text-af-corps">
            <AfricansEtiquette :ton="phase === 'vote' ? 'vert' : 'gris'">{{ LIBELLES_PHASE_PUBLIQUE[concours.phase] }}</AfricansEtiquette>
            <span v-if="phase === 'appel'">Dépôts jusqu'au {{ dateLongue(concours.vote_debut) }}</span>
            <span v-else-if="phase === 'vote'">Vote jusqu'au {{ dateLongue(concours.vote_fin) }}</span>
            <span>{{ concours.participations_publiees }} photo{{ concours.participations_publiees > 1 ? 's' : '' }} publiée{{ concours.participations_publiees > 1 ? 's' : '' }}</span>
          </div>
        </header>

        <p v-if="phase === 'annule'" class="rounded-[10px] border border-af-bordure bg-af-surface px-5 py-4 text-[15px]/[1.5] text-af-encre">
          Ce concours est annulé<template v-if="concours.motif_annulation"> : {{ concours.motif_annulation }}</template>.
          Les photos publiées ont reçu leur récompense de participation.
        </p>

        <p v-if="erreur" class="text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>

        <!-- Appel à participation : mes photos et le dépôt -->
        <section v-if="phase === 'appel'" class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_minmax(0,1fr)]">
          <div class="flex flex-col gap-4">
            <template v-if="moi">
              <article
                v-for="p in mesActives"
                :key="p.id"
                class="flex gap-4 rounded-[10px] border border-af-bordure bg-af-surface p-4"
              >
                <img :src="urlMedia(p.media_url) ?? undefined" alt="Ma photo" class="size-24 shrink-0 rounded-lg object-cover">
                <div class="flex min-w-0 flex-1 flex-col gap-1.5">
                  <span class="w-fit rounded-full px-2.5 py-0.5 text-[12px] font-bold" :class="classeEtat(p.etat)">{{ LIBELLES_MA_PARTICIPATION[p.etat] }}</span>
                  <p v-if="p.legende" class="text-[14px]/[1.4] text-af-encre">{{ p.legende }}</p>
                  <p v-if="p.motif_rejet" class="text-[13px]/[1.4] text-af-live">Motif : {{ p.motif_rejet }}</p>
                  <div class="mt-auto flex flex-wrap gap-x-4 gap-y-1 text-[13px] font-bold">
                    <button v-if="remplacable(p)" type="button" class="text-af-chocolat hover:underline" @click="remplacee = p">
                      <font-awesome-icon icon="fa-solid fa-pen" class="mr-1" />Remplacer
                    </button>
                    <button type="button" class="text-af-corps hover:underline" @click="retirerMaPhoto(p)">
                      <font-awesome-icon icon="fa-solid fa-trash" class="mr-1" />Retirer
                    </button>
                  </div>
                </div>
              </article>

              <JeuDeposerParticipation
                v-if="remplacee"
                :key="remplacee.id"
                :concours-id="concours.id"
                :remplace="remplacee"
                @depose="apresDepot"
                @annuler="remplacee = null"
              />
              <JeuDeposerParticipation
                v-else-if="moi.peut_participer"
                :concours-id="concours.id"
                @depose="apresDepot"
              />
              <p v-else-if="!mesActives.length" class="text-[14px]/[1.5] text-af-corps">Vous ne pouvez plus déposer de photo dans ce concours.</p>
            </template>
            <div v-else class="flex flex-col items-start gap-3 rounded-[10px] border border-af-bordure bg-af-surface p-5">
              <p class="text-[15px]/[1.5] text-af-encre">Connectez-vous pour déposer votre photo.</p>
              <AfricansBouton icone="fa-solid fa-right-to-bracket" @click="versConnexion">Se connecter</AfricansBouton>
            </div>
          </div>

          <div class="flex flex-col gap-3 rounded-[10px] border border-af-bordure bg-af-surface p-5">
            <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Règlement</h2>
            <p class="whitespace-pre-line text-[14px]/[1.6] text-af-corps">{{ concours.reglement }}</p>
            <JeuReglesVote />
          </div>
        </section>

        <!-- Vote à l'aveugle -->
        <section v-else-if="phase === 'vote'" class="flex flex-col gap-4">
          <JeuConfrontation v-if="moi" :concours-id="concours.id" />
          <div v-else class="flex flex-col items-start gap-3 rounded-[10px] border border-af-bordure bg-af-surface p-5">
            <p class="text-[15px]/[1.5] text-af-encre">Le vote est ouvert : connectez-vous pour départager les photos.</p>
            <AfricansBouton icone="fa-solid fa-right-to-bracket" @click="versConnexion">Se connecter pour voter</AfricansBouton>
          </div>
          <JeuReglesVote />
        </section>

        <!-- Résultats : le podium, puis toute la galerie classée -->
        <JeuPodiumConcours v-if="phase === 'resultats'" :elements="photosClassees" />

        <section class="flex flex-col gap-3">
          <h2 class="text-[18px]/[1.4] font-bold text-af-encre">
            {{ phase === 'resultats' ? 'Classement' : 'Les photos' }}
          </h2>
          <p v-if="phase === 'appel' || phase === 'vote'" class="text-[13px]/[1.4] text-af-atone">
            Les auteurs restent anonymes et l'ordre change à chaque visite jusqu'aux résultats.
          </p>
          <JeuGalerieConcours :elements="photos" :concours-id="phase === 'appel' || phase === 'vote' ? concours.id : undefined" />
        </section>
      </template>
    </div>
  </NuxtLayout>
</template>
